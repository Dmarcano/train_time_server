use codegen::{Scope, Variant};
use gtfs_structures::Gtfs;
use std::{collections::HashSet, env, io::{BufWriter, Write}, path::{Path, PathBuf}};
use tonic_build;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/schedules/nyc/google_transit_supplemented.zip";
    // let gtfs_schedule = Gtfs::from_path(path)?;
    // let mut scope = Scope::new();
    // add_stops_enum(&gtfs_schedule, &mut scope);

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



fn add_stops_enum(gtfs_schedule: &Gtfs, scope: &mut Scope) {
    let stop_ids = gtfs_schedule
        .stops
        .iter()
        .map(|stop| -> Option<String> { (stop.1.name.clone() )})
        .filter_map(|stop| stop)
        .collect::<HashSet<_>>();

    let stops_enum = scope.new_enum("Stops");

    for id in stop_ids {
        stops_enum.push_variant(Variant::new(id));
    }
    let codegen = scope.to_string();
    let bytes = codegen.as_bytes();

    // std::fs::write(out_dir, bytes).unwrap();


    let out_dir = PathBuf::from("src/codegen/assets.rs");

    let mut file = BufWriter::new(File::create(&out_dir).unwrap());

    write!(
        &mut file,
        "static KEYWORDS: phf::Map<&'static str, Keyword> = {}",
        phf_codegen::Map::new()
            .entry("loop", "Keyword::Loop")
            .entry("continue", "Keyword::Continue")
            .entry("break", "Keyword::Break")
            .entry("fn", "Keyword::Fn")
            .entry("extern", "Keyword::Extern")
            .build()
    )
    .unwrap();

}

