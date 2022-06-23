#![deny(warnings)]

use std::{env, future::Future};

use anyhow::Result;
use fnapi_core::Env;
use swc_common::{sync::Lrc, SourceMap};
use swc_handler::{try_with_handler, HandlerOpts};
use tracing_subscriber::EnvFilter;

pub mod swc_handler;

/// Configures logger
#[must_use]
pub fn init() -> tracing::subscriber::DefaultGuard {
    let log_env = env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string());
    let log_env = format!("{},hyper=off", log_env);

    let logger = tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_ansi(true)
        .with_env_filter(EnvFilter::new(&log_env))
        .with_test_writer()
        .pretty()
        .finish();

    tracing::subscriber::set_default(logger)
}

pub fn run_async_test<F, Fut, Ret>(opts: HandlerOpts, op: F) -> Result<Ret>
where
    F: FnOnce(Env) -> Fut,
    Fut: Future<Output = Result<Ret>>,
{
    let _guard = init();

    let cm = Lrc::new(SourceMap::default());

    try_with_handler(cm.clone(), opts, |handler| {
        let env = Env {
            cm,
            globals: Default::default(),
            handler,
        };

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        rt.block_on(op(env))
    })
}
