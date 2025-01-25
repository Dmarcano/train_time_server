use core::future::Future;

// use gtfs_structures::{LocationType};

use crate::codegen::assets::NYC_STATION_NAMES_TO_IDS;
use reqwest::{self};
use serde::{Deserialize, Serialize};

use rust_transiter_types::public_api_types::Stop as TransiterStop;
// use crate::codegen::transit_realtime::TransiterStop;

pub trait GtfsRealtimeAPI {
    /***
     * Get all the stations for a transit system
     */
    fn get_outgoing_trips(
        &self,
        stop_name: &str,
    ) -> impl Future<Output = Result<Vec<TransiterStop>, Box<dyn std::error::Error>>>;
}

const TRANSITER_DEMO_URL: &'static str = "https://demo.transiter.dev/";

pub struct TransiterRealTimeAPI {
    server_uri: String,
    agency_url: String,
}

/**
 * Agencies that are available
 */
#[derive(Copy, Clone, Debug)]
pub enum DemogAgencies {
    NycMetro,
}

impl DemogAgencies {
    pub fn get_url(&self) -> &'static str {
        match &self {
            DemogAgencies::NycMetro => "systems/us-ny-subway/",
        }
    }
}

impl TransiterRealTimeAPI {
    pub fn from_example_server(agency: DemogAgencies) -> Self {
        Self {
            server_uri: TRANSITER_DEMO_URL.to_string(),
            agency_url: agency.get_url().to_string(),
        }
    }
}

impl GtfsRealtimeAPI for TransiterRealTimeAPI {
    async fn get_outgoing_trips(
        &self,
        stop_name: &str,
    ) -> Result<Vec<TransiterStop>, Box<dyn std::error::Error>> {
        let stop_base_url = format!("{}{}/stops/", self.server_uri, self.agency_url);

        if let Some(stop_ids) = NYC_STATION_NAMES_TO_IDS.get(stop_name) {
            let mut stops_to_return = Vec::new();
            for stop_id in stop_ids.iter() {
                let stop_name = *stop_id;
                let stop_url = format!("{}{}", stop_base_url, stop_name);

                let response = reqwest::get(stop_url).await?;

                if response.status().is_success() {
                    let stop: TransiterStop = response.json().await?;
                    stops_to_return.push(stop);
                }
            }
            return Result::Ok(stops_to_return);
        }

        Result::Err("Did not find TransiterStop".to_string().into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    pub stops: Vec<TransiterStop>,
    pub next_id: Option<String>,
}
