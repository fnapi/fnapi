//! Client SDK generator for fnapi.
//!
//! This crate should not depend on heavy crates like `swc_ecma_parser`
#![feature(box_syntax)]

use std::sync::Arc;

use anyhow::Result;
use fnapi_api_def::{ApiFile, ApiFn, ProjectApis};
use fnapi_core::Env;
use rayon::prelude::*;
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_utils::{private_ident, quote_ident, ExprFactory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsTargetEnv {
    Web,
    NodeJs,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsClientConfig {
    pub target_env: JsTargetEnv,
}

impl JsClientConfig {
    pub fn generate(&self, env: &Env, project: &ProjectApis) -> Result<Module> {
        env.with(|| {
            let client = private_ident!("__client");
            let import = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                    span: DUMMY_SP,
                    local: client.clone(),
                })],
                src: format!(
                    "@fnapi/api/lib/client/{}.js",
                    match self.target_env {
                        JsTargetEnv::Web => "web",
                        JsTargetEnv::NodeJs => "nodejs",
                    }
                )
                .into(),
                type_only: false,
                asserts: Default::default(),
            }));

            let mut body = project
                .files
                .par_iter()
                .map(|v| {
                    self.generate_file(env, v, &client)
                        .map(ModuleDecl::ExportDecl)
                        .map(ModuleItem::ModuleDecl)
                })
                .collect::<Result<Vec<_>>>()?;

            body.insert(0, import);

            Ok(Module {
                span: DUMMY_SP,
                body,
                shebang: Default::default(),
            })
        })
    }

    fn generate_file(&self, env: &Env, file: &Arc<ApiFile>, client: &Ident) -> Result<ExportDecl> {
        Ok(ExportDecl {
            span: DUMMY_SP,
            decl: Decl::Var(VarDecl {
                span: DUMMY_SP,
                kind: VarDeclKind::Const,
                declare: Default::default(),
                decls: vec![VarDeclarator {
                    span: DUMMY_SP,
                    name: Ident::new(file.class_name.clone(), DUMMY_SP).into(),
                    init: Some(box Expr::Object(
                        self.generate_object_for_file(env, file, client)?,
                    )),
                    definite: Default::default(),
                }],
            }),
        })
    }

    fn generate_object_for_file(
        &self,
        env: &Env,
        file: &Arc<ApiFile>,
        client: &Ident,
    ) -> Result<ObjectLit> {
        env.with(|| {
            let fns = file
                .functions
                .iter()
                .map(|f| {
                    self.generate_fn(file, f, client)
                        .map(|f| {
                            Prop::Method(MethodProp {
                                key: f.ident.clone().into(),
                                function: f.function,
                            })
                        })
                        .map(Box::new)
                        .map(PropOrSpread::from)
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(ObjectLit {
                span: DUMMY_SP,
                props: fns,
            })
        })
    }

    fn generate_fn(&self, file: &ApiFile, f: &Arc<ApiFn>, client: &Ident) -> Result<FnDecl> {
        let stmt = Stmt::Return(ReturnStmt {
            span: DUMMY_SP,
            arg: Some(box Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: client
                    .clone()
                    .make_member(quote_ident!("invoke"))
                    .as_callee(),
                args: vec![
                    file.class_name.clone().as_arg(),
                    f.name.clone().as_arg(),
                    quote_ident!("arguments").as_arg(),
                ],
                type_args: Default::default(),
            })),
        });
        Ok(FnDecl {
            ident: Ident::new(f.name.clone(), DUMMY_SP),
            declare: Default::default(),
            function: Function {
                params: vec![],
                decorators: Default::default(),
                span: DUMMY_SP,
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![stmt],
                }),
                is_generator: false,
                is_async: true,
                type_params: Default::default(),
                return_type: Default::default(),
            },
        })
    }
}
