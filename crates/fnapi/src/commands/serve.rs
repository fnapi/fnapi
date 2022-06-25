use anyhow::Result;
use clap::Parser;
use fnapi_core::Env;

/// Start a development server
#[derive(Parser, Debug)]
pub(crate) struct ServeCommand {
    #[clap(long, short = 'p', default_value = "4321")]
    port: usize,
}

impl ServeCommand {
    pub async fn run(self, env: &Env) -> Result<()> {
        todo!()
    }
}
