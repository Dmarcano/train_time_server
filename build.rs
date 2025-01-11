use codegen::{Scope, Variant};
use gtfs_structures::Gtfs;
use phf_codegen::Map;
use std::fs::File;
use std::{
    collections::{HashMap, HashSet},
    env,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/schedules/nyc/google_transit_supplemented.zip";
    let gtfs_schedule = Gtfs::from_path(path)?;
    add_stops_enum(&gtfs_schedule);

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all("src/codegen")?;

    // Set the output directory for generated code
    let out_dir = PathBuf::from("src/codegen");

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir(out_dir)
        .compile(&vec!["gtfs_compiled/gtfs-realtime.proto"], &["./proto"])?;
    Ok(())
}

fn add_stops_enum(gtfs_schedule: &Gtfs) {
    let stop_ids_to_stop_names = gtfs_schedule
        .stops
        .iter()
        .map(|stop| -> (String, Option<String>) { (stop.1.id.clone(), stop.1.name.clone()) })
        .filter(|stop| stop.1.is_some())
        .map(|stop| (stop.0, stop.1.unwrap()))
        .collect::<HashMap<String, String>>();

    let mut stop_name_to_ids = HashMap::new();

    for (k, v) in &stop_ids_to_stop_names {
        stop_name_to_ids.entry(v).or_insert_with(Vec::new).push(k)
    }

    let stop_name_to_ids_slice = stop_name_to_ids
        .iter()
        .map(|(k, v)| (k, v.as_slice()))
        .collect::<HashMap<_, _>>();

    let mut codegen_map = phf_codegen::Map::new();

    for id in stop_name_to_ids {
        codegen_map.entry(id.0, "foo");
    }

    let out_dir = PathBuf::from("src/codegen/assets.rs");
    let mut file = BufWriter::new(File::create(&out_dir).unwrap());

    write!(
        &mut file,
        "static KEYWORDS: phf::Map<&'static str, Keyword> = {}",
        codegen_map.build()
    )
    .unwrap();
}
