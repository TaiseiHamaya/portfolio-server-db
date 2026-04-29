use std::fs;

use tonic_prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 出力ファイル・ディレクトリの作成
    fs::create_dir_all("src/generated/server/")?;

    // サーバーコードの生成
    tonic_prost_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("src/generated/server/")
        .type_attribute("SessionId", "#[derive(PartialOrd, Ord)]")
        .compile_protos(
            &[
                "process/db/record/service.proto",
                "process/db/user/service.proto",
            ],
            &["portfolio-proto"],
        )?;

    Ok(())
}
