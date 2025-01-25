pub mod gtfs_realtime_api; //

// use axum::extract::Request;
use gtfs_structures::LocationType;
use serde::{Deserialize, Serialize};
use tokio;

use chrono::{DateTime, NaiveDate};
use reqwest::{self};
use std::env;

pub mod codegen;

use gtfs_realtime_api::{DemogAgencies, GtfsRealtimeAPI, TransiterRealTimeAPI};

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
    pub next_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    // Make the GET request to the Transitter demo API
    let api = TransiterRealTimeAPI::from_example_server(DemogAgencies::NycMetro);

    let out = api.get_outgoing_trips("Queensboro Plaza").await?;

    let outv2 = out
        .iter()
        .map(|x| (x.id.clone(), x.stop_times.clone()))
        .map(|(id, stop_times)| {
            let date_times = stop_times
                .iter()
                .map(|time| time.arrival.clone())
                .filter_map(|time| {
                    let foo = time.map(|time| {
                        return time
                            .time
                            .map(|timestamp| DateTime::from_timestamp(timestamp as i64, 0));
                    });
                    return foo;
                })
                .flatten()
                .filter_map(|f| f)
                .collect::<Vec<_>>();
            // date_times.iter.map(|datetime| {
            //     datetime.
            // });
            return (id, date_times);
        })
        .collect::<Vec<_>>();
    println!("Got Response {:?} stops", outv2);

    Ok(())
}
