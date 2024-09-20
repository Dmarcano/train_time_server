use prost::Message;
// use axum::extract::Request;
use gtfs_structures::{Gtfs, Id, Stop};
use tokio;

use reqwest::{self};
use std::{collections::HashSet, env, sync::Arc};

pub mod gtfs_queries;

use gtfs_queries::{get_parent_station_names, get_station_match_name};

pub mod transit_realtime {
    tonic::include_proto!("transit_realtime");
}

use transit_realtime::{
    trip_update::{StopTimeEvent, StopTimeUpdate, TripProperties},
    Alert, FeedEntity, TripDescriptor, TripUpdate, VehiclePosition,
};

#[derive(Copy, Clone, Debug)]
pub enum NYCTrains {
    E,
    A,
    C,
    G,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl NYCTrains {
    pub fn get_api_endpoints_str(&self) -> &'static str {
        match &self {
            Self::A | Self::C | Self::E => {
                "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-ace"
            }
            Self::G => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-g",
            Self::One
            | Self::Two
            | Self::Three
            | Self::Four
            | Self::Five
            | Self::Six
            | Self::Seven => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs",
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let a_train = NYCTrains::Seven;

    let uri = a_train.get_api_endpoints_str();
    let response = reqwest::get(uri).await?;

    let response_bytes = response.bytes().await?;

    let feed_message = transit_realtime::FeedMessage::decode(response_bytes.clone())?;
    let header = feed_message.clone().header;

    let trip_updates = feed_message
        .entity
        .iter()
        .filter_map(|entity| entity.trip_update.clone())
        .collect::<Vec<_>>();
    let vehicles = feed_message
        .entity
        .iter()
        .filter_map(|entity| entity.vehicle.clone())
        .collect::<Vec<_>>();
    // let alerts = feed_message
    //     .entity
    //     .iter()
    //     .filter_map(|entity| entity.alert.clone())
    //     .collect::<Vec<_>>();
    // let shapes = feed_message
    //     .entity
    //     .iter()
    //     .filter_map(|entity| entity.shape.clone())
    //     .collect::<Vec<_>>();
    // let stops = feed_message
    //     .entity
    //     .iter()
    //     .filter_map(|entity| entity.stop.clone())
    //     .collect::<Vec<_>>();

    let seven_train_positions = vehicles
        .iter()
        .filter(|position| {
            position
                .trip
                .as_ref()
                .map(|trip| trip.route_id())
                .map(|id| id == "7" || id == "7x")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let seven_trips = trip_updates
        .iter()
        .filter(|trip_update| {
            trip_update.trip.route_id() == "7" || trip_update.trip.route_id() == "7x"
        })
        .cloned()
        .collect::<Vec<_>>();

    println!("\n====== header ====== \n {:#?}", header);

    // let path = "src/schedules/nyc/google_transit_supplemented.zip";
    // let gtfs_schedule = Gtfs::from_path(path)?;

    // let queensboro_plaza_stops = get_station_match_name("Queensboro", gtfs_schedule.borrow());

    let qbp_stop_ids: HashSet<&'static str> =
        // HashSet::from(["718", "718S", "718N", "R09N", "R09S", "R09"]);
        HashSet::from([ "718N",]);

    // let qbp_stop_ids = queensboro_plaza_stops
    //     .iter()
    //     .map(|stop| stop.id())
    //     .collect::<HashSet<_>>();

    let mut seven_trip_updates_into_queensboro_plaza = seven_trips
        .iter()
        .flat_map(|trip_update| {
            let stop_time_updates_into_station = trip_update
                .stop_time_update
                .iter()
                .filter(|trip| qbp_stop_ids.contains(trip.stop_id()));

            return stop_time_updates_into_station;
        })
        .collect::<Vec<_>>();

    seven_trip_updates_into_queensboro_plaza.sort_by(|stop_time, other| {
        stop_time
            .arrival
            .as_ref()
            .unwrap()
            .time()
            .cmp(&other.arrival.as_ref().unwrap().time())
    });

    // let seven_trip_updates_into_queensboro_plaza = seven_trips
    //     .iter()
    //     .filter(|trip_update| {
    //         let comparison_stop_id = trip_update
    //             .stop_time_update
    //             .first()
    //             .and_then(|foo| Some(foo.stop_id()));
    //         comparison_stop_id.is_some_and(|stop_id| qbp_stop_ids.contains(stop_id))
    //     })
    //     .collect::<Vec<_>>();

    let seven_positions_into_queensboro_plaza = seven_train_positions
        .iter()
        .filter(|position| {
            position
                .stop_id
                .as_ref()
                .map(|stop_id| qbp_stop_ids.contains(stop_id.as_str()))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    println!(
        "\n====== seven trip updates into queensboro plaza ====== \n {:#?}",
        seven_trip_updates_into_queensboro_plaza
    );

    // println!(
    //     "\n====== seven trains into queensboro plaza ====== \n {:#?}",
    //     seven_train_positions
    // );

    // println!(
    //     "\n====== seven trains into queensboro plaza ====== \n {:#?}",
    //     seven_positions_into_queensboro_plaza
    // );
    Ok(())
}

pub fn get_parent_stations(gtfs: &Gtfs) -> Vec<Arc<Stop>> {
    gtfs.stops
        .iter()
        .map(|station| station.1.clone())
        .filter(|station| station.parent_station.is_none())
        .map(|arc| arc.clone())
        .collect::<Vec<_>>()
}

pub fn get_children_stations(gtfs: &Gtfs) {
    let out = gtfs
        .stops
        .iter()
        .filter(|station| station.1.parent_station.is_some())
        .collect::<Vec<_>>();
    print!("{:#?}", out);
}
