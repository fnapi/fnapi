use std::{
    process::{Child, Command, Stdio},
    sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use fnapi_api_def::types::Type;
use jsonrpc_client_transports::RawClient;
use jsonrpc_core::{Params, Value};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use tokio::{runtime::Handle, time::sleep};
use tracing::{debug, info, trace};

use crate::project::InputFiles;

#[cfg(test)]
mod tests;

pub(crate) struct TypeServer {
    process: Child,
    client: RawClient,
}

const TYPE_SERVER_CODE: &str = include_str!("../../type-server.js");

impl TypeServer {
    pub async fn start(input: &InputFiles) -> Result<Arc<Self>> {
        let mut cmd = Command::new("node");
        // Stdin
        cmd.arg("-");

        cmd.stdin(Stdio::piped());

        let port = thread_rng().gen_range::<u16, _>(40000..60000);
        cmd.env("PORT", port.to_string());

        match input {
            InputFiles::Files(files) => {
                let val = files
                    .iter()
                    .map(|v| v.display().to_string())
                    .collect::<Vec<_>>()
                    .join(";");

                cmd.env("TS_FILES", val);
            }
            InputFiles::TsConfig(p) => {
                cmd.env("TS_CONFIG_PATH", p);
            }
        }

        info!(port = port, "Starting type server");

        let mut process = cmd.spawn().context("failed to spawn typeserver")?;

        {
            let child_stdin = process.stdin.as_mut().unwrap();
            child_stdin.write_all(TYPE_SERVER_CODE)?;
        }

        let client = jsonrpc_client_transports::transports::http::connect::<RawClient>(&format!(
            "http://localhost:{}",
            port
        ))
        .await
        .map_err(|e| anyhow!("failed to connect to typeserver: {}", e))?;

        let server = Arc::new(Self { process, client });

        sleep(Duration::from_millis(500)).await;

        loop {
            let res = server.check_started().await;
            if res.is_ok() {
                break;
            }

            sleep(Duration::from_millis(300)).await;
        }

        debug!("Type server started");

        Ok(server)
    }

    #[tracing::instrument(name = "TypeServer::check_started", skip_all)]
    async fn check_started(&self) -> Result<()> {
        let _res = self
            .client
            .call_method("checkStarted", Params::None)
            .await
            .map_err(|e| anyhow!("rpc failed: {}", e))?;

        Ok(())
    }

    #[tracing::instrument(name = "TypeServer::query_types_of_method", skip(self, filename))]
    pub async fn query_types_of_method(
        &self,
        filename: &str,
        method_name: &str,
    ) -> Result<MethodTypes> {
        debug!("Sending query for `{}`", method_name);

        let filename_arg = Value::String(filename.into());
        let name_arg = Value::String(method_name.into());
        let res = self
            .client
            .call_method(
                "queryTypesOfMethod",
                Params::Array(vec![filename_arg, name_arg]),
            )
            .await
            .map_err(|e| anyhow!("rpc failed: {}", e))?;

        let s = res.as_str().unwrap();

        trace!("Received response: `{}`", s);

        let body = serde_json::from_str::<MethodTypes>(s)
            .with_context(|| format!("failed to deserialize json: {}", s))?;

        Ok(body)
    }

    pub fn query_return_type_of_method_sync(
        &self,
        filename: &str,
        method_name: &str,
    ) -> Result<MethodTypes> {
        let rt = Handle::current();

        rt.block_on(async { self.query_types_of_method(filename, method_name).await })
    }
}

impl Drop for TypeServer {
    fn drop(&mut self) {
        let res = self.process.kill();
        info!("Killing type server: {:?}", res);
        let res = self.process.wait();
        info!("Waiting for type server: {:?}", res);
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodTypes {
    pub params: Vec<Type>,

    pub return_type: Type,
}
