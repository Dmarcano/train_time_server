use codegen::{Scope, Variant};
use gtfs_structures::Gtfs;
use std::path::Path;
use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("gtfs_compiled/gtfs-realtime.proto")?;
    // tonic_build::compile_protos("gtfs_compiled/nyc-gtfs-realtime.proto")?;

    // compile_gtfs()?;
    Ok(())
}

fn compile_gtfs() -> Result<(), Box<dyn std::error::Error>> {
    let mut scope = Scope::new();
    let path = "src/schedules/nyc/google_transit_supplemented.zip";
    let gtfs_schedule = Gtfs::from_path(path)?;

    let out = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out).join("assets.rs");
    add_stops_enum(&gtfs_schedule, &mut scope);

    std::fs::write(out, scope.to_string()).unwrap();
    Ok(())
}

fn add_stops_enum(gtfs_schedule: &Gtfs, scope: &mut Scope) {
    let stop_ids = gtfs_schedule
        .stops
        .iter()
        .map(|stop| -> Option<String> { stop.1.name.clone() })
        .filter_map(|stop| stop)
        .collect::<Vec<_>>();

    let stops_enum = scope.new_enum("Stops");

    for id in stop_ids {
        stops_enum.push_variant(Variant::new(id));
    }
}
