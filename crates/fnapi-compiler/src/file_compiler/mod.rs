use std::sync::Arc;

use anyhow::Result;
use fnapi_api_def::{types::json_schema::ToJsonSchema, ApiFile, ApiFn, ApiParam};
use fnapi_core::Env;
use swc_atoms::JsWord;
use swc_common::{errors::HANDLER, util::take::Take, Mark, Span, Spanned, SyntaxContext, DUMMY_SP};
use swc_ecma_transforms_base::{
    helpers::{inject_helpers, Helpers, HELPERS},
    resolver,
};
use swc_ecma_transforms_proposal::decorators;
use swc_ecmascript::{
    ast::*,
    utils::{prepend_stmts, private_ident, quote_ident, ExprFactory},
    visit::{FoldWith, VisitMut, VisitMutWith},
};
use tokio::task::spawn_blocking;

use self::{import_analyzer::ImportMap, magic_replacer::magic_replacer};
use crate::{project::Project, target::ServerTarget, ServerApiFile};

mod import_analyzer;
mod magic_replacer;

const FNAPI_API_MODULE: &str = "@fnapi/api";

impl ServerApiFile {
    pub async fn process(
        &self,
        env: &Env,
        project: Arc<Project>,
    ) -> Result<(Module, Arc<ApiFile>)> {
        let name = Arc::new(swc_common::FileName::Real(self.path.clone()));
        let filename = self.path.display().to_string();

        let m = env.with(|| project.modules.load(&self.path))?;
        let mut m = (*m).clone();

        let env = env.clone();
        spawn_blocking(move || {
            env.with(|| {
                HELPERS.set(&Helpers::new(true), || {
                    let unresolved_mark = project.modules.unresolved_mark;
                    let top_level_mark = project.modules.get_top_level_mark_for(name);

                    // Resolve all variables.
                    m.visit_mut_with(&mut resolver(unresolved_mark, top_level_mark, true));

                    let imports = Arc::new(ImportMap::analyze(&m));
                    let mut stmts_to_prepend = vec![];
                    let mut extras = vec![];

                    let class_name;
                    let methods;
                    {
                        let mut compiler = FileCompiler {
                            project: &project,
                            filename: &filename,
                            imports,
                            top_level_mark,

                            in_export_default_decl: false,

                            wrap_fnapi_config: Default::default(),

                            stmts_to_prepend: &mut stmts_to_prepend,
                            stmts_to_append: &mut extras,
                            class_name: Default::default(),
                            compiled_method_records: Default::default(),

                            target: project.server_target.clone(),
                        };
                        m.visit_mut_with(&mut compiler);
                        class_name = compiler.class_name;
                        methods = compiler.compiled_method_records;
                    }

                    m = m.fold_with(&mut decorators::decorators(decorators::Config {
                        legacy: true,
                        emit_metadata: false,
                        use_define_for_class_fields: false,
                    }));

                    m.visit_mut_with(&mut swc_ecma_transforms_typescript::strip_with_config(
                        swc_ecma_transforms_typescript::Config {
                            import_not_used_as_values:
                                swc_ecma_transforms_typescript::ImportsNotUsedAsValues::Preserve,
                            ..Default::default()
                        },
                        top_level_mark,
                    ));

                    prepend_stmts(&mut m.body, stmts_to_prepend.into_iter());
                    m.body.extend(extras);

                    m.visit_mut_with(&mut inject_helpers());

                    m.visit_mut_with(&mut swc_ecma_transforms_base::hygiene::hygiene());
                    m.visit_mut_with(&mut swc_ecma_transforms_base::fixer::fixer(None));

                    Ok((
                        m,
                        Arc::new(ApiFile {
                            class_name,
                            functions: methods.iter().map(|v| v.api_def.clone()).collect(),
                        }),
                    ))
                })
            })
        })
        .await?
    }
}

struct FileCompiler<'a> {
    project: &'a Project,
    filename: &'a str,

    imports: Arc<ImportMap>,
    top_level_mark: Mark,

    in_export_default_decl: bool,

    wrap_fnapi_config: Option<Ident>,

    stmts_to_prepend: &'a mut Vec<ModuleItem>,
    stmts_to_append: &'a mut Vec<ModuleItem>,

    class_name: JsWord,
    compiled_method_records: Vec<MethodRecord>,

    target: Arc<dyn ServerTarget>,
}

#[derive(Debug, Clone)]
struct MethodRecord {
    pub name: JsWord,
    pub config_object_var_name: Ident,
    pub api_def: Arc<ApiFn>,
}

impl FileCompiler<'_> {
    fn compile_api_method(&mut self, method: &mut ClassMethod) -> Option<!> {
        let api_ann_span = get_span_of_api_decorator(&self.imports, &method.function.decorators)?;

        let name = match &method.key {
            PropName::Ident(v) => v.clone(),
            _ => {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(method.span, "API method must use an identifier as a key")
                        .span_note(api_ann_span, "This method has the @FnApi decorator")
                        .emit();
                });
                return None;
            }
        };

        let mut args_to_fn_api = None;
        {
            // Remove `@FnApi`
            method
                .function
                .decorators
                .retain_mut(|dec| match &mut *dec.expr {
                    Expr::Call(CallExpr {
                        callee: Callee::Expr(callee),
                        args,
                        ..
                    }) => {
                        if self.imports.is_import(callee, FNAPI_API_MODULE, "FnApi") {
                            assert_eq!(args_to_fn_api, None, "Multiple @FnApi?");

                            args_to_fn_api = Some(args.take());
                            false
                        } else {
                            true
                        }
                    }
                    _ => true,
                });
        }

        let config_object_name = Ident::new(
            format!("__fnapi_config_for_{}", name.sym).into(),
            name.span.with_ctxt(SyntaxContext::empty()),
        );

        {
            let var_decl = VarDeclarator {
                span: DUMMY_SP,
                name: config_object_name.clone().into(),
                init: Some(box Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: self
                        .wrap_fnapi_config
                        .get_or_insert_with(|| private_ident!("wrapFnApiConfig"))
                        .clone()
                        .as_callee(),
                    args: args_to_fn_api.unwrap_or_default(),
                    type_args: Default::default(),
                })),
                definite: Default::default(),
            };
            self.stmts_to_prepend
                .push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Const,
                    declare: Default::default(),
                    decls: vec![var_decl],
                }))));
        }

        let req_var = private_ident!(api_ann_span, "_req");
        let reply_var = private_ident!(api_ann_span, "_reply");

        let body = match &mut method.function.body {
            Some(body) => body,
            None => {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(method.span, "API method must have a body")
                        .span_note(api_ann_span, "This method has the @FnApi decorator")
                        .emit();
                });
                return None;
            }
        };

        let stmts_for_param_init = if method.function.params.is_empty() {
            vec![]
        } else {
            let mut stmts = vec![];

            let params_var = private_ident!("params");

            stmts.push(Stmt::Decl(Decl::Var(VarDecl {
                span: DUMMY_SP,
                kind: VarDeclKind::Const,
                declare: Default::default(),
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    name: params_var.clone().into(),
                    init: Some(box req_var.clone().make_member(quote_ident!("params"))),
                    definite: Default::default(),
                }],
            })));

            for (idx, param) in method.function.params.take().into_iter().enumerate() {
                let init = box Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: box params_var.clone().into(),
                    prop: MemberProp::Computed(ComputedPropName {
                        span: DUMMY_SP,
                        expr: idx.into(),
                    }),
                });
                stmts.push(Stmt::Decl(Decl::Var(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Let,
                    decls: vec![VarDeclarator {
                        span: DUMMY_SP,
                        name: param.pat,
                        init: Some(init),
                        definite: false,
                    }],
                    declare: false,
                })));
            }
            stmts
        };

        // req
        method.function.params.push(Param {
            span: DUMMY_SP,
            decorators: Default::default(),
            pat: Pat::Ident(req_var.clone().into()),
        });

        // reply
        method.function.params.push(Param {
            span: DUMMY_SP,
            decorators: Default::default(),
            pat: Pat::Ident(reply_var.into()),
        });

        prepend_stmts(&mut body.stmts, stmts_for_param_init.into_iter());

        body.visit_mut_with(&mut magic_replacer(req_var, self.imports.clone()));

        let ret_ty =
            self.extract_return_type(method.function.span, &method.function.return_type)?;

        let method_types = self
            .project
            .type_server
            .query_return_type_of_method_sync(self.filename, &name.sym);

        let method_types = match method_types {
            Ok(v) => v,
            Err(err) => {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(
                            ret_ty.span(),
                            &format!("Failed to detect the return type: {}", err),
                        )
                        .emit();
                });
                return None;
            }
        };

        {
            // Create a record
            self.compiled_method_records.push(MethodRecord {
                name: name.sym.clone(),
                config_object_var_name: config_object_name,
                api_def: Arc::new(ApiFn {
                    name: name.sym,
                    params: method_types
                        .params
                        .iter()
                        .map(|ty| ApiParam {
                            // TODO
                            name: None,
                            ty: Arc::new(ty.clone()),
                        })
                        .collect(),
                    return_type: Arc::new(method_types.return_type.clone()),
                }),
            });
        }

        None
    }

    fn extract_return_type(&self, method_span: Span, ty: &Option<TsTypeAnn>) -> Option<TsType> {
        let ty = match ty {
            Some(ret_ty) => ret_ty,
            None => {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(
                            method_span,
                            "All API functions should have declared return types",
                        )
                        .help("This is to prevent mistakenly sending sensitive data to the client")
                        .emit();
                });
                return None;
            }
        };

        let span = ty.span;
        // Should be Promise<T>

        let ty = match &*ty.type_ann {
            TsType::TsTypeRef(TsTypeRef {
                type_name: TsEntityName::Ident(ident),
                type_params: Some(type_args),
                ..
            }) => {
                if &*ident.sym == "Promise" {
                    if type_args.params.len() == 1 {
                        Some(&type_args.params[0])
                    } else {
                        HANDLER.with(|handler| {
                            handler
                                .struct_span_err(span, "Promise<T> takes only one argument")
                                .emit();
                        });
                        return None;
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        let ty = match ty {
            Some(v) => v,
            None => {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(span, "All API functions should return Promise<T>")
                        .help(
                            "APIs are executed from the server, so they should return a Promise<T>",
                        )
                        .emit();
                });
                return None;
            }
        };

        Some(*ty.clone())
    }
}

impl VisitMut for FileCompiler<'_> {
    fn visit_mut_class_method(&mut self, n: &mut ClassMethod) {
        n.visit_mut_children_with(self);

        if !self.in_export_default_decl {
            return;
        }

        self.compile_api_method(n);
    }

    fn visit_mut_export_default_decl(&mut self, n: &mut ExportDefaultDecl) {
        match &n.decl {
            DefaultDecl::Class(cls) => {
                if cls.ident.is_none() {
                    HANDLER.with(|handler| {
                        handler
                            .struct_span_err(cls.class.span, "All API classes should have a name")
                            .emit();
                    });
                } else {
                    self.class_name = cls.ident.clone().unwrap().sym;
                }

                // TODO: Ends with Api?
            }
            _ => {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(
                            n.decl.span(),
                            "All api files should export a class like 'export default class \
                             TodoApi'",
                        )
                        .emit();
                });
                return;
            }
        }

        let old = self.in_export_default_decl;
        self.in_export_default_decl = true;
        n.visit_mut_children_with(self);
        self.in_export_default_decl = old;
    }

    fn visit_mut_module_decl(&mut self, n: &mut ModuleDecl) {
        n.visit_mut_children_with(self);

        if !self.compiled_method_records.is_empty() {
            if let ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
                span,
                decl: DefaultDecl::Class(cls),
            }) = n
            {
                let wrapper = private_ident!("wrapApiClass");

                let wrap_api_class_import =
                    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                        span: *span,
                        specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                            span: DUMMY_SP,
                            local: wrapper.clone(),
                        })],
                        src: "@fnapi/api/rt/wrapApiClass.js".into(),
                        type_only: false,
                        asserts: Default::default(),
                    }));

                let wrap_api_config_import =
                    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                        span: *span,
                        specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                            span: DUMMY_SP,
                            local: self.wrap_fnapi_config.clone().unwrap(),
                        })],
                        src: "@fnapi/api/rt/wrapFnApiConfig.js".into(),
                        type_only: false,
                        asserts: Default::default(),
                    }));

                prepend_stmts(
                    self.stmts_to_prepend,
                    vec![wrap_api_class_import, wrap_api_config_import].into_iter(),
                );

                *n = ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
                    span: *span,
                    expr: box Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        callee: wrapper.as_callee(),
                        args: vec![
                            cls.take().as_arg(),
                            ArrayLit {
                                span: DUMMY_SP,
                                elems: self
                                    .compiled_method_records
                                    .iter()
                                    .map(|method_record| ObjectLit {
                                        span: DUMMY_SP,
                                        props: vec![
                                            PropOrSpread::Spread(SpreadElement {
                                                dot3_token: DUMMY_SP,
                                                expr: box method_record
                                                    .config_object_var_name
                                                    .clone()
                                                    .into(),
                                            }),
                                            PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                                                key: quote_ident!("name").into(),
                                                value: box method_record.name.clone().into(),
                                            })),
                                            PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                                                key: quote_ident!("parameterTypes").into(),
                                                value: box ArrayLit {
                                                    span: DUMMY_SP,
                                                    elems: method_record
                                                        .api_def
                                                        .params
                                                        .iter()
                                                        .map(|param| param.ty.to_js_expr().as_arg())
                                                        .map(Some)
                                                        .collect(),
                                                }
                                                .into(),
                                            })),
                                            PropOrSpread::Prop(box Prop::KeyValue(KeyValueProp {
                                                key: quote_ident!("returnType").into(),
                                                value: method_record
                                                    .api_def
                                                    .return_type
                                                    .to_js_expr(),
                                            })),
                                        ],
                                    })
                                    .map(|v| v.as_arg())
                                    .map(Some)
                                    .collect(),
                            }
                            .as_arg(),
                        ],
                        type_args: Default::default(),
                    }),
                });
            }
        }
    }
}

fn get_span_of_api_decorator(imports: &ImportMap, decorators: &[Decorator]) -> Option<Span> {
    decorators.iter().find_map(|dec| match &*dec.expr {
        Expr::Call(CallExpr {
            callee: Callee::Expr(callee),
            ..
        }) => {
            if imports.is_import(callee, FNAPI_API_MODULE, "FnApi") {
                Some(callee.span())
            } else {
                None
            }
        }
        _ => None,
    })
}
