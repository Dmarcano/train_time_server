use std::sync::Arc;

use gtfs_structures::{Gtfs, Stop};

fn get_all_stations_query<'input>(
    gtfs: &'input Gtfs,
) -> impl Iterator<Item = (String, Arc<Stop>)> + 'input {
    gtfs.stops
        .iter()
        .filter(|station| station.1.name.is_some())
        .map(|station| (station.0.clone(), station.1.clone()))
}

pub fn get_parent_station_names(gtfs: &Gtfs) -> Vec<(String, String)> {
    get_all_stations_query(gtfs)
        .map(|station| station.1.clone())
        .filter(|station| station.parent_station.is_none())
        .map(|arc| arc.clone())
        .map(|stop| (stop.id.clone(), stop.name.clone().unwrap()))
        .collect()
}
