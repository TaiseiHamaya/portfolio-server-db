use std::fs;

use tonic_prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("PROTOC", "./portfolio-proto/protoc-32.1-win64/bin/protoc");
    }

    // 出力ファイル・ディレクトリの作成
    fs::create_dir_all("src/generated/server/")?;

    // サーバーコードの生成
    tonic_prost_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("src/generated/server/")
        .compile_protos(
            &[
                "process/db/record/service.proto",
                "process/db/session/service.proto",
            ],
            &["portfolio-proto"],
        )?;

    Ok(())
}
