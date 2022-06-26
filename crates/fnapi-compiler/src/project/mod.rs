use std::{path::PathBuf, process::Stdio, sync::Arc};

use anyhow::{bail, Context, Result};
use fnapi_core::Env;
use module_storage::modules::Modules;
use tokio::{process::Command, try_join};

use crate::{type_server::TypeServer, target::ServerTarget};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputFiles {
    Files(Vec<PathBuf>),
    TsConfig(PathBuf),
}

impl InputFiles {
    async fn to_files(&self) -> Result<Vec<PathBuf>> {
        match self {
            InputFiles::Files(files) => Ok(files.clone()),
            InputFiles::TsConfig(tsconfig_json) => {
                let mut cmd = Command::new("npx");

                cmd.arg("tsc")
                    .arg("--listFiles")
                    .arg("--noEmit")
                    .arg("--listFilesOnly");
                cmd.arg("-p").arg(tsconfig_json);

                cmd.stderr(Stdio::inherit());
                let output = cmd.output().await.context("`tsc --listFiles` failed")?;

                if !output.status.success() {
                    bail!(
                        "`tsc --listFiles` failed: {}",
                        String::from_utf8_lossy(&output.stdout)
                    );
                }

                let s = String::from_utf8(output.stdout)
                    .context("tsc --listFiles returned non-utf8 output")?;

                Ok(s.lines()
                    .filter(|path| !path.ends_with(".d.ts"))
                    .map(PathBuf::from)
                    .collect())
            }
        }
    }
}

/// This type is cheap to clone.
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub input: Arc<InputFiles>,
}

impl ProjectConfig {
    #[tracing::instrument(name = "ProjectConfig::resolve", skip_all)]
    pub async fn resolve(&self, env: &Env) -> Result<Arc<Project>> {
        let (type_server, files) =
            try_join!(TypeServer::start(&self.input), self.input.to_files())?;
        let files = Arc::new(files);

        let modules = env.with(|| Modules::new(env.cm.clone()));

        Ok(Arc::new(Project {
            modules,
            type_server,
            files,
        }))
    }
}

/// Fully resolved instance of a project.
///
///
/// This type is cheap to clone.
#[derive(Clone)]
pub struct Project {
    pub(crate) type_server: Arc<TypeServer>,

    pub(crate) modules: Arc<Modules>,

    pub files: Arc<Vec<PathBuf>>,

    pub server_target: Arc<dyn ServerTarget>,
}
