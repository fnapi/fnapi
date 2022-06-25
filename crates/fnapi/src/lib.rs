#![feature(box_syntax)]

mod commands;

use std::{
    fmt,
    io::{stderr, Write},
    sync::Arc,
};

use clap::{Parser, Subcommand};
use fnapi_core::Env;
use swc_common::{errors::Handler, SourceMap};
use swc_error_reporters::{GraphicalReportHandler, PrettyEmitter, PrettyEmitterConfig};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Build(commands::BuildCommand),
    Serve(commands::ServeCommand),
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let cm = Arc::new(SourceMap::default());

        let env = Env {
            cm: cm.clone(),
            globals: Default::default(),
            handler: Arc::new(Handler::with_emitter(
                true,
                false,
                box PrettyEmitter::new(
                    cm,
                    box StderrWriter,
                    GraphicalReportHandler::new(),
                    PrettyEmitterConfig {
                        skip_filename: false,
                    },
                ),
            )),
        };

        match self.cmd {
            Command::Build(cmd) => cmd.run(&env).await,
            Command::Serve(cmd) => cmd.run(&env).await,
        }
    }
}

struct StderrWriter;

impl fmt::Write for StderrWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        stderr()
            .write_all(s.as_bytes())
            .map(|_| ())
            .map_err(|_| fmt::Error)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut buf = [0; 4];

        c.encode_utf8(&mut buf);
        stderr().write_all(&buf).map(|_| ()).map_err(|_| fmt::Error)
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        stderr().write_fmt(args).map(|_| ()).map_err(|_| fmt::Error)
    }
}
