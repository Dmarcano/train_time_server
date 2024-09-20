use gtfs_structures::{Gtfs, Stop};
use std::sync::Arc;

// provide data from reading static gtfs files. Coming in from downloading zipped files from agencies
pub trait StaticGTFSDataProvider {
    fn get_all_stations(&self) -> Vec<Arc<Stop>>;

    fn get_stations_by_name(&self, name_to_match: &str) -> Vec<Arc<Stop>>;
}

impl StaticGTFSDataProvider for Gtfs {
    fn get_all_stations(&self) -> Vec<Arc<Stop>> {
        get_all_stations_query(self)
            .map(|station| station.1.clone())
            .collect()
    }

    fn get_stations_by_name(&self, name_to_match: &str) -> Vec<Arc<Stop>> {
        get_station_match_name(name_to_match, &self)
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
