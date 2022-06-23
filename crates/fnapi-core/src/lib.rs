use std::sync::Arc;

use swc_common::{
    errors::{Handler, HANDLER},
    Globals, SourceMap, GLOBALS,
};

#[derive(Clone)]
pub struct Env {
    pub cm: Arc<SourceMap>,
    pub globals: Arc<Globals>,
    pub handler: Arc<Handler>,
}

impl Env {
    pub fn with<Ret>(&self, op: impl FnOnce() -> Ret) -> Ret {
        GLOBALS.set(&self.globals, || HANDLER.set(&self.handler, || op()))
    }
}
