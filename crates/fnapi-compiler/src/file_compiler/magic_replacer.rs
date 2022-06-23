use std::sync::Arc;

use swc_common::{errors::HANDLER, Spanned};
use swc_ecmascript::{
    ast::*,
    utils::{quote_ident, ExprFactory},
    visit::{VisitMut, VisitMutWith},
};

use super::{import_analyzer::ImportMap, FNAPI_API_MODULE};

/// Replaces `Context` and `ServerConfig` with a correct code.
pub(crate) fn magic_replacer(req_var: Ident, imports: Arc<ImportMap>) -> impl VisitMut {
    MagicReplacer { req_var, imports }
}

#[derive(Debug)]
struct MagicReplacer {
    req_var: Ident,
    imports: Arc<ImportMap>,
}

impl MagicReplacer {
    fn is_magic_type(&self, name: &str, e: &Expr) -> bool {
        self.imports.is_import(e, FNAPI_API_MODULE, name)
    }
}

impl VisitMut for MagicReplacer {
    fn visit_mut_call_expr(&mut self, e: &mut CallExpr) {
        e.visit_mut_children_with(self);

        if let Callee::Expr(box Expr::Member(MemberExpr {
            obj,
            prop: MemberProp::Ident(prop),
            ..
        })) = &e.callee
        {
            if &*prop.sym == "get" {
                if !self.is_magic_type("Context", obj) && !self.is_magic_type("ServerConfig", &obj)
                {
                    return;
                }

                {
                    // Verify
                    let mut has_error = false;

                    let type_arg_cnt = e.type_args.as_ref().map(|t| t.params.len()).unwrap_or(0);

                    if type_arg_cnt != 1 {
                        HANDLER.with(|handler| {
                            handler
                                .struct_span_err(
                                    e.span,
                                    "This is a magic call and should have exactly one type \
                                     argument",
                                )
                                .emit();
                        });
                        has_error = true;
                    }

                    if !e.args.is_empty() {
                        HANDLER.with(|handler| {
                            handler
                                .struct_span_err(
                                    e.span,
                                    "This is a magic call and should have no arguments",
                                )
                                .emit();
                        });
                        has_error = true;
                    }

                    if has_error {
                        return;
                    }
                }

                e.args.push(match &*e.type_args.take().unwrap().params[0] {
                    TsType::TsTypeRef(e) => entity_name_to_expr(&e.type_name).as_arg(),
                    ty => {
                        HANDLER.with(|handler| {
                            handler
                                .struct_span_err(
                                    ty.span(),
                                    "This is a magic call and type argument must be a declared \
                                     class",
                                )
                                .emit();
                        });
                        return;
                    }
                });

                if self.is_magic_type("Context", &obj) {
                    // Context.get
                    // =>
                    // req.getContext(ClassName)
                    e.callee = self
                        .req_var
                        .clone()
                        .make_member(quote_ident!("getContext"))
                        .as_callee();
                } else if self.is_magic_type("ServerConfig", &obj) {
                    // ServerConfig.get
                    // =>
                    // req.getServerConfig(ClassName)
                    e.callee = self
                        .req_var
                        .clone()
                        .make_member(quote_ident!("getServerConfig"))
                        .as_callee();
                }
            }
        }
    }
}

fn entity_name_to_expr(e: &TsEntityName) -> Box<Expr> {
    match e {
        TsEntityName::TsQualifiedName(q) => {
            Box::new(entity_name_to_expr(&q.left).make_member(q.right.clone()))
        }
        TsEntityName::Ident(i) => Box::new(Expr::Ident(i.clone())),
    }
}
