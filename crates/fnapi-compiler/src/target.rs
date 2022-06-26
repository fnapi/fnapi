use std::sync::Arc;

use anyhow::Result;
use fnapi_api_def::{ApiFile, ApiFn, ProjectApis};

/// The target of **server**.
pub trait ServerTarget: Send + Sync {
    fn store_api(&self, api: &ApiDesc) -> Result<()>;

    fn clear(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct ApiDesc<'a> {
    pub project: &'a ProjectApis,
    pub file: Arc<ApiFile>,
    pub f: Arc<ApiFn>,
}

pub struct Native {}

impl ServerTarget for Native {
    fn store_api(&self, api: &ApiDesc) -> Result<()> {
        todo!()
    }

    fn clear(&self) -> Result<()> {
        todo!()
    }
}

pub trait ServerlessTarget: Send + Sync {}

pub struct ServerlessService(pub dyn ServerlessTarget);

impl ServerTarget for ServerlessService {
    fn store_api(&self, api: &ApiDesc) -> Result<()> {
        todo!()
    }

    fn clear(&self) -> Result<()> {
        todo!()
    }
}

pub struct NextJs {}

impl ServerlessTarget for NextJs {}

pub struct AwsLambda {}

impl ServerlessTarget for AwsLambda {}
