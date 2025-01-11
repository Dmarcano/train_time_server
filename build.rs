use codegen::{Scope, Variant};
use gtfs_structures::Gtfs;
use std::{env, path::{Path, PathBuf}};
use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all("src/codegen")?;
    
    // Set the output directory for generated code
    let out_dir = PathBuf::from("src/codegen");

    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        ).out_dir(out_dir)
        .compile(&vec!["gtfs_compiled/gtfs-realtime.proto"], &["./proto"])?; 
    //::compile_protos("gtfs_compiled/gtfs-realtime.proto")?;
    // tonic_build::compile_protos("gtfs_compiled/nyc-gtfs-realtime.proto")?;
    Ok(())
}

