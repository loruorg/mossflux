fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/mossflux.proto");
    let fds = protox::compile(["proto/mossflux.proto"], ["proto"])?;
    tonic_prost_build::configure().compile_fds(fds)?;
    Ok(())
}
