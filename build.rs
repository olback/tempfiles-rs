use std::{fs, process::Command};

fn main() {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap()
        .stdout;

    fs::write("version.txt", out).unwrap();
}
