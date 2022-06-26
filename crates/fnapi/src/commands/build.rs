use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
    sync::Arc,
};

use anyhow::{bail, Context, Error, Result};
use clap::{ArgEnum, Parser};
use fnapi_api_def::ProjectApis;
use fnapi_client_gen::JsTargetEnv;
use fnapi_compiler::{
    project::{InputFiles, ProjectConfig},
    target::{AwsLambda, Native, NextJs, ServerTarget, ServerlessService},
    ServerApiFile,
};
use fnapi_core::Env;
use futures::future::join_all;
use rayon::prelude::*;
use tokio::{spawn, task::yield_now};

/// Build functions as a server and generate client sdk.
#[derive(Parser, Debug)]
pub(crate) struct BuildCommand {
    /// Directories or files to build
    inputs: Vec<PathBuf>,

    /// Option to deploy fnapi server to external providers.
    #[clap(arg_enum, long, short = 't', default_value = "fnapi")]
    server_target: Target,

    /// Client types to generate.
    #[clap(arg_enum, long, short = 'c')]
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

#[derive(ArgEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    #[clap(name = "fnapi")]
    Native,

    #[clap(name = "nextjs")]
    NextJs,

    #[clap(name = "lambda")]
    AwsLambda,
}

impl Default for Target {
    fn default() -> Self {
        Self::Native
    }
}

impl BuildCommand {
    pub async fn run(self, env: &Env) -> Result<()> {
        let inputs = self
            .inputs
            .into_par_iter()
            .map(expand_dir)
            .collect::<Result<Vec<_>>>()
            .context("failed to expand inputs")?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if inputs.is_empty() {
            bail!("no inputs found");
        }

        let fnapi_dir = self.fnapi_dir.unwrap_or_else(|| PathBuf::from(".fnapi"));

        create_dir_all(&fnapi_dir).context("failed to create fnapi directory")?;

        let server_target: Arc<dyn ServerTarget> = match self.server_target {
            Target::Native => Arc::new(Native {}),
            Target::NextJs => Arc::new(ServerlessService::new(NextJs {})),
            Target::AwsLambda => Arc::new(ServerlessService::new(AwsLambda {})),
        };

        let project = ProjectConfig {
            input: Arc::new(InputFiles::Files(inputs)),
        }
        .resolve(env, server_target)
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

        let file_apis = files.into_iter().map(|v| v.2.clone()).collect::<Vec<_>>();

        let project_apis = ProjectApis { files: file_apis };

        {
            let node_client = print(
                env.cm.clone(),
                fnapi_client_gen::JsClientConfig {
                    target_env: JsTargetEnv::NodeJs,
                }
                .generate(&env, &project_apis)?,
            );
            write(&fnapi_dir.join("client.node.mjs"), node_client.as_bytes())
                .context("failed to write node client")?;
        }

        {
            let web_client = print(
                env.cm.clone(),
                fnapi_client_gen::JsClientConfig {
                    target_env: JsTargetEnv::Web,
                }
                .generate(&env, &project_apis)?,
            );
            write(&fnapi_dir.join("client.web.mjs"), web_client.as_bytes())
                .context("failed to write web client")?;
        }
        Ok(())
    }
}

fn expand_dir(p: PathBuf) -> Result<Vec<PathBuf>> {
    let stat = p
        .metadata()
        .with_context(|| format!("failed to get metadata of `{}`", p.display()))?;

    if stat.is_dir() {
        let read = p
            .read_dir()
            .with_context(|| format!("failed to read directory `{}`", p.display()))?;

        Ok(read
            .par_bridge()
            .map(|v| expand_dir(v?.path()))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect())
    } else {
        Ok(vec![p])
    }
}

fn print(cm: Arc<SourceMap>, m: &Module) -> String {
    let mut buf = vec![];

    {
        let mut emitter = swc_ecmascript::codegen::Emitter {
            cfg: Default::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };

        emitter.emit_module(m).unwrap();
    }

    String::from_utf8(buf).unwrap()
}
