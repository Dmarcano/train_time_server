use gtfs_structures::Gtfs;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufWriter, Write},
    path::PathBuf,
};
use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "src/schedules/nyc/nyc_subway.zip";

    let gtfs_schedule = gtfs_structures::GtfsReader::default()
        .read_stop_times(false)
        .read_shapes(false)
        .read_from_path(path)?;

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

    for (k, v) in stop_ids_to_stop_names {
        stop_name_to_ids.entry(v).or_insert_with(Vec::new).push(k)
    }

    let stop_name_to_ids_slice = stop_name_to_ids
        .iter()
        .map(|(k, v)| {
            let str_repr = v
                .iter()
                .map(|s| format!("{:?}", s)) // Using debug formatting will properly escape special characters
                .collect::<Vec<_>>()
                .join(", ");

            let array_repr = format!("&[{}]", str_repr);

            (k.clone(), array_repr)
        })
        .collect::<HashMap<String, String>>();

    let mut codegen_map = phf_codegen::Map::new();

    for (key, id) in stop_name_to_ids_slice {
        codegen_map.entry(key, &id);
    }

    let out_dir = PathBuf::from("src/codegen/assets.rs");
    let mut file = BufWriter::new(File::create(&out_dir).unwrap());

    write!(
        &mut file,
        "pub static NYC_STATION_NAMES_TO_IDS: phf::Map<&'static str, &'static [&'static str]> = {};",
        codegen_map.build()
    )
    .unwrap();
}
