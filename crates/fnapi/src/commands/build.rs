use std::path::PathBuf;

use anyhow::Result;
use clap::{ArgEnum, Parser};

/// Build functions as a server and generate client sdk.
#[derive(Parser, Debug)]
pub(crate) struct BuildCommand {
    /// The functions to build. This is a list of regular
    /// expressions.
    #[clap(long, name = "pattern")]
    only: Vec<String>,

    /// Client types to generate.
    #[clap(arg_enum, long, short = 't')]
    client_types: Vec<ClientType>,

    /// Directory to use for generated client api.
    #[clap(long, short = 'd')]
    client_target_dir: Option<PathBuf>,
}

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClientType {
    /// Typescript
    Web,
}

impl BuildCommand {
    pub async fn run(self) -> Result<()> {
        todo!()
    }
}
