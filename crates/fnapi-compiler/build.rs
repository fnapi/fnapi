use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=rollup.config.js");
    println!("cargo:rerun-if-changed=src/**/*.ts");

    if !cfg!(target_os = "windows") && env::var("CI").as_deref() == Ok("1") {
        Command::new("chmod")
            .arg("-R")
            .arg("777")
            .arg(env::var("OUT_DIR").unwrap())
            .status()
            .unwrap();
    }

    let mut c = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C").arg("npx");
        c
    } else {
        Command::new("npx")
    };

    let status = c.arg("rollup").arg("-c").status().unwrap();
    assert!(status.success());
}
