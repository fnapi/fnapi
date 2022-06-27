use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=rollup.config.js");
    println!("cargo:rerun-if-changed=src/**/*.ts");

    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let out_dir = out_dir
        .canonicalize()
        .expect("failed to canonicalize output directory");

    if !cfg!(target_os = "windows")
        && (env::var("CI").as_deref() == Ok("1") || env::var("CI").as_deref() == Ok("true"))
    {
        let status = Command::new("chmod")
            .arg("-R")
            .arg("777")
            .arg(&out_dir)
            .status()
            .unwrap();
        assert!(status.success(), "chmod failed");

        eprintln!("chmod -R 777 $OUT_DIR");
    }

    eprintln!("Output (env): {}", env::var("OUT_DIR").unwrap());
    eprintln!("Output: {}", out_dir.display());

    let mut c = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C").arg("npx");
        c
    } else {
        Command::new("npx")
    };

    let status = c.arg("rollup").arg("-c").status().unwrap();
    assert!(status.success(), "rollup failed");
}
