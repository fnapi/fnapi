use std::sync::Arc;

use anyhow::Result;
use fnapi_api_def::{ApiFile, ApiFn, ProjectApis};

/// The target of **server**.
pub trait Target {
    fn store_api(&self, api: &ApiDesc) -> Result<()>;

    fn clear(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct ApiDesc<'a> {
    pub project: &'a ProjectApis,
    pub file: Arc<ApiFile>,
    pub f: Arc<ApiFn>,
}

pub struct Direct {}

impl Target for Direct {}

pub trait ServerlessTarget {}

pub struct Serverless(pub dyn ServerlessTarget);

impl Target for Serverless {}

pub struct NextJs {}

impl ServerlessTarget for NextJs {}

pub struct AwsLambda {}

impl ServerlessTarget for AwsLambda {}
