use fnapi_api_def::{ApiFile, ProjectApis};

/// The target of **server**.
pub trait Target {
    fn store_api(&self, http_path: &str, project: &ProjectApis, file: &ApiFile, api: &ApiFn);
}

pub struct Direct {}

impl Target for Direct {}

pub struct NextJs {}

impl Target for NextJs {}

pub struct AwsLambda {}

impl Target for AwsLambda {}
