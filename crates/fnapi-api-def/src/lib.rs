//! Type definitions for the FnAPI.
#![feature(box_syntax)]

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use swc_atoms::JsWord;

use self::types::Type;

pub mod types;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiProject {
    pub files: Vec<Arc<ApiFile>>,
}

/// This struct contains enough information to generate client for a api file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiFile {
    pub class_name: JsWord,
    pub functions: Vec<Arc<ApiFn>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiFn {
    pub name: JsWord,

    pub params: Vec<ApiParam>,

    pub return_type: Arc<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiParam {
    pub name: Option<JsWord>,
    pub ty: Arc<Type>,
}
