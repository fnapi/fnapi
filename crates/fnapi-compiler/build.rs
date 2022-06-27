use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=rollup.config.js");
    println!("cargo:rerun-if-changed=src/**/*.ts");

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
