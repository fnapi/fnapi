//! Client SDK generator for fnapi.
//!
//! This crate should not depend on heavy crates like `swc_ecma_parser`
#![feature(box_syntax)]

use std::sync::Arc;

use anyhow::Result;
use fnapi_api_def::{ApiFile, ApiFn, Project};
use fnapi_core::Env;
use rayon::prelude::*;
use swc_common::{util::take::Take, DUMMY_SP};
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
    pub fn generate(&self, env: &Env, project: &Project) -> Result<Vec<Module>> {
        project
            .files
            .par_iter()
            .map(|v| self.generate_file(env, v))
            .collect::<Result<_>>()
    }

    pub fn generate_file(&self, env: &Env, file: &Arc<ApiFile>) -> Result<Module> {
        env.with(|| {
            let client = private_ident!("__client");
            let import = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                    span: DUMMY_SP,
                    local: client.clone(),
                })],
                src: format!(
                    "@fnapi/api/client/{}.js",
                    match self.target_env {
                        JsTargetEnv::Web => "web",
                        JsTargetEnv::NodeJs => "nodejs",
                    }
                )
                .into(),
                type_only: false,
                asserts: Default::default(),
            }));

            let fns = file
                .functions
                .iter()
                .map(|f| {
                    self.generate_fn(file, f, &client)
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

            let expr = box Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: fns,
            });

            let export = ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
                span: DUMMY_SP,
                expr,
            }));
            Ok(Module {
                body: vec![import, export],
                ..Module::dummy()
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
