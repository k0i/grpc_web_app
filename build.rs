use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Please append our protofile here.
    let protos = vec!["proto/kb/v1/news.proto"];

    // google's proto
    tonic_build::configure().build_server(false).compile(
        &["googleapis/google/pubsub/v1/pubsub.proto"],
        &["./googleapis"],
    )?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("kb_descriptor.bin"))
        .compile(&protos, &["proto/kb/v1"])?;
    Ok(())
}
