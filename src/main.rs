use prost::Message;
// use axum::extract::Request;
use gtfs_structures::Gtfs;
use tokio;

use reqwest::{self};
use std::{borrow::Borrow, env};

pub mod num_conversion;

pub mod transit_realtime {
    tonic::include_proto!("transit_realtime");
}

use transit_realtime::{FeedEntity, TripUpdate, VehiclePosition};

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

    let entity: &transit_realtime::FeedEntity =
        feed_message.entity.iter().take(1).next().take().unwrap();

    println!("{:#?}", entity);
    println!("\n====== header ====== \n {:#?}", header);

    let path = "src/schedules/nyc/google_transit_supplemented.zip";
    let gtfs_schedule = Gtfs::from_path(path)?;

    let a = gtfs_schedule.borrow();
    println!("\n====== header ====== \n {:#?}", a.read_duration);

    Ok(())
}

fn convert_entity_to_row(entity: &FeedEntity) -> Result<(), String> {
    Ok(())
}

fn convert_trip_update(update: &TripUpdate) -> Result<(), String> {
    Ok(())
}

fn convert_vehicle_pos(vehicle_pos: &VehiclePosition) -> Result<(), String> {
    Ok(())
}
