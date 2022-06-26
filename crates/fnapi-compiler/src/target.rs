use fnapi_api_def::{ApiFile, ProjectApis};

/// The target of **server**.
pub trait Target {
    fn store_api(&self, http_path: &str, project: &ProjectApis, file: &ApiFile, api: &ApiFn);
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
