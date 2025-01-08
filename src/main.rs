pub mod gtfs_queries;

// use axum::extract::Request;
use gtfs_structures::{Availability, Gtfs, Id, LocationType, Pathway, Stop, StopTransfer};
use serde::{Deserialize, Serialize};
use tokio;

use reqwest::{self, Error, Response};
use std::{collections::HashMap, env, sync::Arc};

pub mod transit_realtime {
    tonic::include_proto!("transit_realtime");
}

use transit_realtime::{
    trip_update::{StopTimeEvent, StopTimeUpdate, TripProperties},
    Alert, FeedEntity, TripDescriptor, TripUpdate, VehiclePosition,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MyStop {
    /// Unique technical identifier (not for the traveller) of the stop
    pub id: String,
    /// Short text or a number that identifies the location for riders
    pub code: Option<String>,
    ///Name of the location. Use a name that people will understand in the local and tourist vernacular
    pub name: Option<String>,
    /// Description of the location that provides useful, quality information
    pub description: Option<String>,
    /// Type of the location
    #[serde(default)]
    pub location_type: LocationType,
    /// Defines hierarchy between the different locations
    pub parent_station: Option<String>,
    /// Identifies the fare zone for a stop
    pub zone_id: Option<String>,
    /// URL of a web page about the location
    pub url: Option<String>,
    /// Longitude of the stop
    pub longitude: Option<f64>,
    /// Latitude of the stop
    pub latitude: Option<f64>,
    /// Timezone of the location
    pub timezone: Option<String>,
    /// Level of the location. The same level can be used by multiple unlinked stations
    pub level_id: Option<String>,
    /// Platform identifier for a platform stop (a stop belonging to a station)
    pub platform_code: Option<String>,
    /// Transfers from this Stop
    /// Text to speech readable version of the stop_name
    pub tts_name: Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    pub stops: Vec<MyStop>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    let client = reqwest::Client::new();

    // Make the GET request to the Transitter demo API
    let response = reqwest::get("https://demo.transiter.dev/systems/us-ny-subway/stops").await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response into a vector of Station structs
        let api_response: ApiResponse = response.json().await?;
        let stations = api_response.stops;

        // Print out the station information
        println!("Found {} NYC subway stations:", stations.len());
        for station in stations {
            println!(
                "Name: {:?}\nLine: {:?}\nLocation: ({:?}, {:?})\nID: {}\n",
                station.name, station.code, station.latitude, station.longitude, station.id
            );
        }
    } else {
        println!("Error: {} - {}", response.status(), response.text().await?);
    }

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
