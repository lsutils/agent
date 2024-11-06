use std::error::Error;
use std::process::Command;

fn generate_protobuf() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("../src/proto")
        .compile(&["./src/agent.proto", "./src/common.proto"], &["./src"])?;

    Command::new("cargo")
        .args(["fmt", "--", "../src/proto/*.rs"])
        .spawn()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    generate_protobuf()?;
    Ok(())
}
