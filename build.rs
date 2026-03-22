use tonic_prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("PROTOC", "./portfolio-proto/protoc-32.1-win64/bin/protoc");
    }

    tonic_prost_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("src/generated/server/")
        .compile_protos(&[], &["portfolio-proto"])?;

    Ok(())
}
