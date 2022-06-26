use std::{borrow::Cow, sync::Arc};

use anyhow::Result;
use fnapi_api_def::{ApiFile, ApiFn, ProjectApis};
use fnapi_core::HasId;
use swc_atoms::JsWord;

/// The target of **server**.
pub trait ServerTarget: HasId {
    fn wrap_api_class_import_path(&self) -> JsWord;
}

#[derive(Debug, Clone)]
pub struct ApiDesc<'a> {
    pub project: &'a ProjectApis,
    pub file: Arc<ApiFile>,
    pub f: Arc<ApiFn>,
}

pub struct Native {}

impl HasId for Native {
    fn id(&self) -> Cow<'static, str> {
        "fnapi".into()
    }

    fn name(&self) -> Cow<'static, str> {
        "FnApi".into()
    }
}

impl ServerTarget for Native {
    fn wrap_api_class_import_path(&self) -> JsWord {
        "@fnapi/api/rt/wrapApiClass.js".into()
    }
}

pub trait ServerlessTarget: HasId {}

pub struct ServerlessService(pub dyn ServerlessTarget);

impl HasId for ServerlessService {
    fn id(&self) -> Cow<'static, str> {
        self.0.id()
    }

    fn name(&self) -> Cow<'static, str> {
        self.0.name()
    }
}

impl ServerTarget for ServerlessService {
    fn wrap_api_class_import_path(&self) -> JsWord {}
}

pub struct NextJs {}

impl ServerlessTarget for NextJs {}

pub struct AwsLambda {}

impl ServerlessTarget for AwsLambda {
    fn name(&self) -> &'static str {
        "AWS Lambda"
    }
}
