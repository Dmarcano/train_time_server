use prost::Message;
// use axum::extract::Request;
use gtfs_structures::{Gtfs, Stop};
use tokio;

use reqwest::{self};
use std::{borrow::Borrow, env, sync::Arc};

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
    let alerts = feed_message
        .entity
        .iter()
        .filter_map(|entity| entity.alert.clone())
        .collect::<Vec<_>>();
    let shapes = feed_message
        .entity
        .iter()
        .filter_map(|entity| entity.shape.clone())
        .collect::<Vec<_>>();
    let stops = feed_message
        .entity
        .iter()
        .filter_map(|entity| entity.stop.clone())
        .collect::<Vec<_>>();

    let seven_trips = trip_updates
        .iter()
        .filter(|trip_update| {
            trip_update.trip.route_id() == "7" || trip_update.trip.route_id() == "7x"
        })
        .cloned()
        .collect::<Vec<_>>();

    // we need to find the

    // 7 train trip id
    // queensboro plaza stop ID
    // from there add all the 7 trains that are stopping there

    println!("\n====== header ====== \n {:#?}", header);

    let path = "src/schedules/nyc/google_transit_supplemented.zip";
    let gtfs_schedule = Gtfs::from_path(path)?;

    // let names = get_parent_station_names(gtfs_schedule.borrow());
    let out  = get_station_match_name("Queensboro", gtfs_schedule.borrow());
    println!("\n====== names ====== \n {:#?}", out);
    // get_children_stations(gtfs_schedule.borrow());

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
