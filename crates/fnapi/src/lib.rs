mod commands;

use clap::{Parser, Subcommand};

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
        match self.cmd {
            Command::Build(cmd) => cmd.run().await,
            Command::Serve(cmd) => cmd.run().await,
        }
    }
}
