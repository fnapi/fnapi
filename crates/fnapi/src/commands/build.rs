use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::{ArgEnum, Parser};
use fnapi_compiler::project::{ProjectConfig, InputFiles};
use fnapi_core::Env;

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
    pub async fn run(self, env: &Env) -> Result<()> {
        let project = ProjectConfig {
            input: Arc::new(InputFiles::Files(vec![input.clone()])),
            type_server_script_path: "src/type_server/index.js".into(),
        }
        .resolve(&env)
        .await?;

        Ok(())
    }
}
