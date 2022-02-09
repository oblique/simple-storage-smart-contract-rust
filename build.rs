use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn compile_sol(src_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    let src_path = src_path.as_ref();
    let out_dir = out_dir.as_ref();

    println!("cargo:rerun-if-changed={}", src_path.display());

    let status = Command::new("solc")
        .arg(src_path)
        .arg("--overwrite")
        .arg("--abi")
        .arg("--bin")
        .arg("--optimize")
        .arg("--optimize-runs")
        .arg("2000")
        .arg("-o")
        .arg(out_dir)
        .spawn()
        .expect("solc not installed")
        .wait()
        .unwrap();

    if !status.success() {
        panic!("Failed to compile {}", src_path.display());
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    compile_sol("contracts/SimpleStorage.sol", &out_dir);
}
