use std::{borrow::Cow, sync::Arc};

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
        GLOBALS.set(&self.globals, || HANDLER.set(&self.handler, op))
    }
}

/// Any kind of implementation.
pub trait Impl: Send + Sync {
    fn id(&self) -> Cow<'static, str>;

    fn name(&self) -> Cow<'static, str>;
}
