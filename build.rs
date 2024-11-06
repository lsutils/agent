use std::{error::Error};

fn set_build_info() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-env=AGENT_NAME=deepflow-agent-ce");
    Ok(())
}

fn main() {
    _ = set_build_info();
}