use crate::codegen::assets::NYC_STATION_NAMES_TO_IDS;
use async_trait::async_trait;
use reqwest::{self};
use serde::{Deserialize, Serialize};

use rust_transiter_types::public_api_types::{
    EntrypointReply, GetStopRequest, ListStopsReply, ListStopsRequest, Stop as TransiterStop,
};

#[async_trait]
pub trait GtfsRealtimeAPI {
    /***
     * Get all the stations for a transit system
     */
    async fn get_outgoing_trips(
        &self,
        stop_name: &str,
    ) ->  Result<Vec<TransiterStop>, Box<dyn std::error::Error>>;
}

const TRANSITER_DEMO_URL: &'static str = "https://demo.transiter.dev/";

pub struct TransiterRealTimeAPI {
    transiter_cliet: Box<dyn TransiterWebAPI + Sync + Send>,
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
        let client = ReqWestTransiterClient::from_example_server(agency);
        Self {
            transiter_cliet: Box::new(client),
        }
    }
}

#[async_trait]
impl GtfsRealtimeAPI for TransiterRealTimeAPI {
    
    async fn get_outgoing_trips(
        &self,
        stop_name: &str,
    ) -> Result<Vec<TransiterStop>, Box<dyn std::error::Error>> {

        if let Some(stop_ids) = NYC_STATION_NAMES_TO_IDS.get(stop_name) {
            let mut stops_to_return: Vec<TransiterStop> = Vec::new();
            for stop_id in stop_ids.iter() {
                let stop_name = *stop_id;

                let request = GetStopRequest {
                    system_id : "none".to_string(),
                    skip_stop_times : false, 
                    stop_id : stop_name.to_string(),
                    skip_service_maps : true, 
                    skip_transfers: true, 
                    skip_alerts : false, 
                };

                let stop = self.transiter_cliet.get_stop(&request).await?;
                stops_to_return.push(stop);
                
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

struct ReqWestTransiterClient {
    server_uri: String,
    agency_url: String,
}

impl ReqWestTransiterClient {
    pub fn from_example_server(agency: DemogAgencies) -> Self {
        Self {
            server_uri: TRANSITER_DEMO_URL.to_string(),
            agency_url: agency.get_url().to_string(),
        }
    }

    fn get_base_uri(&self) -> String {
        format!("{}{}/stops/", self.server_uri, self.agency_url)
    }
}

#[async_trait]
impl TransiterWebAPI for ReqWestTransiterClient {
    async fn get_transiter_entrypoint(
        &self,
    ) -> Result<EntrypointReply, Box<dyn core::error::Error>> {
        let response = reqwest::get(self.get_base_uri()).await?;
        match  response.error_for_status() {
            Ok(response) => {
                let reply: EntrypointReply = response.json().await?;
                return Result::Ok(reply);
            }
            Err(error) => {
                return Err(error.into()); 
            }
        }
    }

    async fn list_stops(
        &self,
        _request: &ListStopsRequest,
    ) -> Result<ListStopsReply, Box<dyn core::error::Error>> {
        todo!()
    }

    async fn get_stop(
        &self,
        request: &GetStopRequest,
    ) -> Result<TransiterStop, Box<dyn core::error::Error>> {
        let stop_id: &str  = request.stop_id.as_ref();
        let stop_url = format!("{}{}/stops/{}",  self.server_uri, self.agency_url, stop_id);

        let response = reqwest::get(stop_url).await?;

        match  response.error_for_status() {
            Ok(response) => {
                let stop: TransiterStop = response.json().await?;
                return Result::Ok(stop);
            }
            Err(error) => {
                return Err(error.into()); 
            }
        }
    }
}

#[async_trait]
pub trait TransiterWebAPI {
    async fn get_transiter_entrypoint(
        &self,
    ) ->  Result<EntrypointReply, Box<dyn core::error::Error>>;


    async fn list_stops(
        &self,
        request: &ListStopsRequest,
    ) -> Result<ListStopsReply, Box<dyn core::error::Error>>;

    async fn get_stop(
        &self,
        request: &GetStopRequest,
    ) -> Result<TransiterStop, Box<dyn core::error::Error>>;
}
