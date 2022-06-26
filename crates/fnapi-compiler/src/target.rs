use std::{borrow::Cow, sync::Arc};

use auto_impl::auto_impl;
use fnapi_api_def::{ApiFile, ApiFn, ProjectApis};
use fnapi_core::Entity;
use swc_atoms::JsWord;

/// The target of **server**.
pub trait ServerTarget: Entity {
    fn wrap_api_class_import_path(&self) -> JsWord;
}

#[derive(Debug, Clone)]
pub struct ApiDesc<'a> {
    pub project: &'a ProjectApis,
    pub file: Arc<ApiFile>,
    pub f: Arc<ApiFn>,
}

pub struct Native {}

impl Entity for Native {
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

#[auto_impl(Box)]
pub trait ServerlessTarget: Entity {}

pub struct ServerlessService(Box<dyn ServerlessTarget>);

impl ServerlessService {
    pub fn new(target: impl 'static + ServerlessTarget) -> Self {
        Self(Box::new(target))
    }
}

impl Entity for ServerlessService {
    fn id(&self) -> Cow<'static, str> {
        self.0.id()
    }

    fn name(&self) -> Cow<'static, str> {
        self.0.name()
    }
}

impl ServerTarget for ServerlessService {
    fn wrap_api_class_import_path(&self) -> JsWord {
        format!("@fnapi/api/rt/_vendor/{}/wrapApiClass.js", self.0.id()).into()
    }
}

pub struct NextJs {}

impl Entity for NextJs {
    fn id(&self) -> Cow<'static, str> {
        "nextjs".into()
    }

    fn name(&self) -> Cow<'static, str> {
        "NextJs".into()
    }
}

impl ServerlessTarget for NextJs {}

pub struct AwsLambda {}

impl Entity for AwsLambda {
    fn id(&self) -> Cow<'static, str> {
        "aws-lambda".into()
    }

    fn name(&self) -> Cow<'static, str> {
        "AWS Lambda".into()
    }
}

impl ServerlessTarget for AwsLambda {}
