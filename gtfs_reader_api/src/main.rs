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
            return (id, date_times);
        })
        .collect::<Vec<_>>();
    println!("Got Response {:?} stops", outv2);

    Ok(())
}
