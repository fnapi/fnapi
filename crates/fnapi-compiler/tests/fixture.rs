#![deny(warnings)]

use std::{path::PathBuf, sync::Arc};

use fnapi_api_def::ApiFile;
use fnapi_compiler::{
    project::{InputFiles, ProjectConfig},
    ServerApiFile,
};
use fnapi_core::Env;
use fnapi_testing::{run_async_test, swc_handler::HandlerOpts};
use swc_common::{errors::ColorConfig, SourceMap};
use swc_ecmascript::{ast::Module, codegen::text_writer::JsWriter};
use testing::NormalizedOutput;

#[testing::fixture("tests/fixture/**/input.ts")]
fn compile(input: PathBuf) {
    let output_path = input.with_file_name("output.mjs");
    let api_def_output = input.with_file_name("apiDef.json");
    let node_client = input.with_file_name("client.node.mjs");
    let web_client = input.with_file_name("client.web.mjs");

    let (code, api_def) = run_async_test(
        HandlerOpts {
            color: ColorConfig::Always,
        },
        |env| async move {
            let project = ProjectConfig {
                input: Arc::new(InputFiles::Files(vec![input.clone()])),
                type_server_script_path: "src/type_server/index.js".into(),
            }
            .resolve(&env)
            .await?;

            let m = ServerApiFile::from_file(input).unwrap();

            let (output, api_def) = m.process(&env, project).await?;
            let code = print(env.cm.clone(), &output);

            {
                // Test client generation
                test_client_codegen(
                    &env,
                    &api_def,
                    fnapi_client_gen::JsClientConfig {
                        target_env: fnapi_client_gen::JsTargetEnv::NodeJs,
                    },
                )
                .compare_to_file(&node_client)
                .unwrap();
                test_client_codegen(
                    &env,
                    &api_def,
                    fnapi_client_gen::JsClientConfig {
                        target_env: fnapi_client_gen::JsTargetEnv::Web,
                    },
                )
                .compare_to_file(&web_client)
                .unwrap();
            }

            Ok((code, api_def))
        },
    )
    .unwrap();

    NormalizedOutput::from(code)
        .compare_to_file(&output_path)
        .unwrap();

    {
        // Test api definition
        NormalizedOutput::from(serde_json::to_string_pretty(&api_def).unwrap())
            .compare_to_file(&api_def_output)
            .unwrap();
    }
}

fn test_client_codegen(
    env: &Env,
    api: &Arc<ApiFile>,
    config: fnapi_client_gen::JsClientConfig,
) -> NormalizedOutput {
    let output = config
        .generate_file(env, api)
        .expect("failed to generate client");

    print(Default::default(), &output).into()
}

#[testing::fixture("tests/errors/**/input.ts")]
fn errors(input: PathBuf) {
    let output_path = input.with_extension("stderr");

    let err = run_async_test(
        HandlerOpts {
            color: ColorConfig::Never,
        },
        |env| async move {
            let project = ProjectConfig {
                input: Arc::new(InputFiles::Files(vec![input.clone()])),
                type_server_script_path: "src/type_server/index.js".into(),
            }
            .resolve(&env)
            .await?;

            let m = ServerApiFile::from_file(input).unwrap();
            m.process(&env, project).await
        },
    )
    .unwrap_err();

    let err = err.to_string();

    NormalizedOutput::from(err)
        .compare_to_file(&output_path)
        .unwrap();
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

#[testing::fixture("tests/projects/**/tsconfig.json")]
fn project(_ts_config: PathBuf) {
    run_async_test(
        HandlerOpts {
            color: ColorConfig::Always,
        },
        |_cm| async move {
            // TODO
            Ok(())
        },
    )
    .unwrap();
}
