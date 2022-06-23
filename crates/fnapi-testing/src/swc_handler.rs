use std::{fmt, mem::take, sync::Arc};

use anyhow::Result;
use parking_lot::Mutex;
use swc_common::{
    errors::{ColorConfig, Handler, HANDLER},
    SourceMap,
};
use swc_error_reporters::{
    GraphicalReportHandler, GraphicalTheme, PrettyEmitter, PrettyEmitterConfig,
};

#[derive(Clone, Default)]
struct LockedWriter(Arc<Mutex<String>>);

impl fmt::Write for LockedWriter {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.0.lock().write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.lock().write_char(c)
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.0.lock().write_fmt(args)
    }
}

pub struct HandlerOpts {
    pub color: ColorConfig,
}

impl HandlerOpts {
    fn to_reporter(c: ColorConfig) -> GraphicalReportHandler {
        match c {
            ColorConfig::Auto => todo!(),
            ColorConfig::Always => GraphicalReportHandler::new(),
            ColorConfig::Never => GraphicalReportHandler::new().with_theme(GraphicalTheme::none()),
        }
    }
}

pub fn try_with_handler<F, Ret>(cm: Arc<SourceMap>, opts: HandlerOpts, op: F) -> Result<Ret>
where
    F: FnOnce(Arc<Handler>) -> Result<Ret>,
{
    let wr = Box::new(LockedWriter::default());

    let emitter = PrettyEmitter::new(
        cm,
        wr.clone(),
        HandlerOpts::to_reporter(opts.color),
        PrettyEmitterConfig::default(),
    );
    let handler = Arc::new(Handler::with_emitter(true, false, Box::new(emitter)));

    let ret = HANDLER.set(&handler, || op(handler.clone()));

    if handler.has_errors() {
        let mut lock = wr.0.lock();
        let msg = take(&mut *lock);

        match ret {
            Ok(_) => Err(anyhow::anyhow!(msg)),
            Err(err) => Err(err.context(msg)),
        }
    } else {
        ret
    }
}
