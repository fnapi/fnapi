use std::sync::Arc;

use anyhow::Result;
use fnapi_api_def::{ApiFile, ApiFn, ProjectApis};
use fnapi_core::Identifiable;
use swc_atoms::JsWord;

/// The target of **server**.
pub trait ServerTarget: Identifiable {
    fn wrap_api_class_import_path(&self) -> JsWord;
}

#[derive(Debug, Clone)]
pub struct ApiDesc<'a> {
    pub project: &'a ProjectApis,
    pub file: Arc<ApiFile>,
    pub f: Arc<ApiFn>,
}

pub struct Native {}

impl Identifiable for Native {}

impl ServerTarget for Native {
    fn name(&self) -> &'static str {
        "fnapi"
    }

    fn wrap_api_class_import_path(&self) -> JsWord {
        "@fnapi/api/rt/wrapApiClass.js".into()
    }
}

pub trait ServerlessTarget: Identifiable {}

pub struct ServerlessService(pub dyn ServerlessTarget);

impl ServerTarget for ServerlessService {
    fn name(&self) -> &'static str {
        self.0.name()
    }

    fn wrap_api_class_import_path(&self) -> JsWord {}
}

pub struct NextJs {}

impl ServerlessTarget for NextJs {
    fn name(&self) -> &'static str {
        "next.js"
    }
}

pub struct AwsLambda {}

impl ServerlessTarget for AwsLambda {
    fn name(&self) -> &'static str {
        "AWS Lambda"
    }
}
