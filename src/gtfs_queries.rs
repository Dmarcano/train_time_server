use std::sync::Arc;

use gtfs_structures::{Gtfs, Stop};

#[derive(Clone, Debug)]
pub struct StationName {
    pub name: String,
    pub id: String,
}

impl From<(String, String)> for StationName {
    fn from(station_name: (String, String)) -> Self {
        Self {
            id: station_name.0,
            name: station_name.1,
        }
    }
}

impl From<Stop> for StationName {
    fn from(stop: Stop) -> Self {
        Self {
            id: stop.id,
            name: stop.name.unwrap(),
        }
    }
}

fn get_all_stations_query<'input>(
    gtfs: &'input Gtfs,
) -> impl Iterator<Item = (String, Arc<Stop>)> + 'input {
    gtfs.stops
        .iter()
        .filter(|station| station.1.name.is_some())
        .map(|station| (station.0.clone(), station.1.clone()))
}

pub fn get_parent_station_names(gtfs: &Gtfs) -> Vec<Arc<Stop>> {
    get_all_stations_query(gtfs)
        .map(|station| station.1.clone())
        .filter(|station| station.parent_station.is_none())
        .map(|arc| arc.clone())
        .collect()
}

pub fn get_station_match_name(name_to_match: &str, gtfs: &Gtfs) -> Vec<Arc<Stop>> {
    let lowercase = name_to_match.to_lowercase();
    get_all_stations_query(gtfs)
        .map(|station| station.1.clone())
        .filter(|station| {
            station
                .name
                .as_ref()
                .is_some_and(|station_name| station_name.to_lowercase().contains(&lowercase))
        })
        .collect()
}
