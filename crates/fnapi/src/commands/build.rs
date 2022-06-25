use std::{fs::create_dir_all, path::PathBuf, sync::Arc};

use anyhow::{Context, Error, Result};
use clap::{ArgEnum, Parser};
use fnapi_compiler::{
    project::{InputFiles, ProjectConfig},
    ServerApiFile,
};
use fnapi_core::Env;
use futures::future::join_all;
use tokio::{spawn, task::yield_now};

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

    /// FnApi directory for printing the api definition. Defaults to `.fnapi`
    #[clap(long)]
    fnapi_dir: Option<PathBuf>,
}

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClientType {
    /// Typescript
    Web,
}

impl BuildCommand {
    pub async fn run(self, env: &Env) -> Result<()> {
        let fnapi_dir = self.fnapi_dir.unwrap_or_else(|| PathBuf::from(".fnapi"));

        create_dir_all(&fnapi_dir).context("failed to create fnapi directory")?;

        let project = ProjectConfig {
            input: Arc::new(InputFiles::TsConfig("tsconfig.json".into())),
        }
        .resolve(env)
        .await
        .context("failed to resolve project")?;

        let mut handles = vec![];

        for file in project.files.iter().cloned() {
            let project = project.clone();
            let env = env.clone();

            handles.push(spawn(async move {
                let m = ServerApiFile::from_file(file.clone())?;
                let (module, api_file) = m.process(&env, project).await?;

                Ok::<_, Error>((file, module, api_file))
            }));
        }

        yield_now().await;

        let files = join_all(handles)
            .await
            .into_iter()
            .collect::<Result<Result<Vec<_>>, _>>()??;

        Ok(())
    }
}
