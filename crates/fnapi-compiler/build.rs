use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/**/*.ts");

    let status = Command::new("npx").arg("tsc").status().unwrap();

    assert!(status.success());
}
