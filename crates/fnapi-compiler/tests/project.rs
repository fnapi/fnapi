use std::{env, path::PathBuf, sync::Arc};

use fnapi_compiler::{
    project::{InputFiles, ProjectConfig},
    target::Native,
    ServerApiFile,
};
use fnapi_testing::{run_async_test, swc_handler::HandlerOpts};
use once_cell::sync::Lazy;
use swc_common::{errors::ColorConfig, SourceMap};
use swc_ecmascript::{
    ast::{ImportDecl, Module},
    codegen::text_writer::JsWriter,
    visit::{VisitMut, VisitMutWith},
};
use tokio::{
    fs::{read_dir, write},
    process::Command,
};
use tracing::info;

#[testing::fixture("tests/projects/**/tsconfig.json")]
fn exec(tsconfig_json: PathBuf) {
    let project_dir = tsconfig_json.parent().unwrap();
    let src_dir = project_dir.join("src");
    let test_dir = project_dir.join("test");

    run_async_test(
        HandlerOpts {
            color: ColorConfig::Always,
        },
        |env| async move {
            let project = ProjectConfig {
                input: Arc::new(InputFiles::TsConfig(tsconfig_json)),
            }
            .resolve(&env, Arc::new(Native {}))
            .await?;

            {
                // Create index.server.js
                let js_path = src_dir.join("index.server.js");
                write(
                    &js_path,
                    format!(
                        "export * as rt from '{}/rt/index.js';",
                        api_pkg_dir().display(),
                    ),
                )
                .await?;
            }

            // Compile all api files in `src` directory
            for path in project.files.iter() {
                if path.starts_with(&src_dir) {
                    info!("Compiling {}", path.display());

                    let f = ServerApiFile::from_file(path.to_path_buf())?;

                    let (mut processed, _) = f.process(&env, project.clone()).await?;
                    processed.visit_mut_with(&mut ImportReplacer);

                    let code = print(env.cm.clone(), &processed);

                    let js_path = path.with_extension("server.js");
                    write(&js_path, code).await?;
                }
            }

            // Run tests

            let mut dir = read_dir(&test_dir).await?;
            while let Some(test_file) = dir.next_entry().await? {
                info!("Running {}", test_file.path().display());

                let mut cmd = Command::new("yarn");
                cmd.arg("mocha").arg(test_file.path());
                let status = cmd.status().await?;

                assert!(status.success());
            }

            Ok(())
        },
    )
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

/// Compiles `@fnapi/api` package and returns the path to it.
fn api_pkg_dir() -> Arc<PathBuf> {
    static DIR: Lazy<Arc<PathBuf>> = Lazy::new(|| {
        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("packages")
            .join("api");

        let mut cmd = std::process::Command::new("yarn");
        cmd.current_dir(&dir).arg("run").arg("build:tsc");
        let _status = cmd.status().expect("failed to compile api package");
        // assert!(status.success());

        Arc::new(dir)
    });

    DIR.clone()
}

struct ImportReplacer;

impl VisitMut for ImportReplacer {
    fn visit_mut_import_decl(&mut self, i: &mut ImportDecl) {
        i.visit_mut_children_with(self);

        if i.src.value.starts_with("@fnapi/api") {
            i.src.raw = None;

            let api_dir = api_pkg_dir();

            if &*i.src.value == "@fnapi/api" {
                i.src.value = format!("{}/index.js", api_dir.display()).into();
            } else {
                i.src.value = i
                    .src
                    .value
                    .replace("@fnapi/api", &api_dir.display().to_string())
                    .into();
            }
        }
    }
}
