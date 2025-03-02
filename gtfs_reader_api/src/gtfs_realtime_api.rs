use crate::codegen::assets::NYC_STATION_NAMES_TO_IDS;
use futures::{stream, StreamExt, TryStreamExt};
use reqwest::{self};
use serde::{Deserialize, Serialize};

use rust_transiter_types::public_api_types::{
    EntrypointReply, GetStopRequest, ListStopsReply, ListStopsRequest, Stop as TransiterStop,
};


pub trait GtfsRealtimeAPI {
    /***
     * Get all the stations for a transit system
     */
    fn get_outgoing_trips(
        &self,
        stop_name: &str,
    ) -> impl core::future::Future<Output =Result<Vec<TransiterStop>, Box<dyn std::error::Error + Sync + Send>>>;
}

const TRANSITER_DEMO_URL: &'static str = "https://demo.transiter.dev/";

pub struct TransiterRealTimeAPI<TransiterWebClient>
where TransiterWebClient : TransiterWebAPI + Sync  + Send{
    transiter_cliet: TransiterWebClient,
    // transiter_cliet: Box<dyn TransiterWebAPI + Sync + Send>,
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

impl TransiterRealTimeAPI<ReqWestTransiterClient> {
    pub fn from_example_server(agency: DemogAgencies) -> Self {
        let client = ReqWestTransiterClient::from_example_server(agency);

        Self {
            transiter_cliet: client,
        }
    }
}


impl<TransiterWebClient> GtfsRealtimeAPI for TransiterRealTimeAPI<TransiterWebClient>  where TransiterWebClient : TransiterWebAPI + Sync  + Send  {
    async fn get_outgoing_trips(
        &self,
        stop_name: &str,
    ) -> Result<Vec<TransiterStop>, Box<dyn std::error::Error + Sync + Send>> {
        if let Some(stop_ids) = NYC_STATION_NAMES_TO_IDS.get(stop_name) {
            let stop_2 = stop_ids
                .iter()
                .map(|ref_str| ref_str.to_string())
                .collect::<Vec<_>>();
            let stream = stream::iter(stop_2.into_iter());
            let out = stream
                .map(|stop_id: String| async move {
                    let request = GetStopRequest {
                        system_id: "none".to_string(),
                        skip_stop_times: false,
                        stop_id: stop_id.clone(),
                        skip_service_maps: true,
                        skip_transfers: true,
                        skip_alerts: false,
                    };

                    let stop = self.transiter_cliet.get_stop(&request).await;
                    println!("stopid: {:#?} stop is ok: {:#?}", stop_id, stop.is_ok());

                    return stop;
                })
                .buffered(10);

            let foo: Vec<_> = out.try_collect().await?;
            return Result::Ok(foo);
        }

        Result::Err("Did not find TransiterStop".to_string().into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    pub stops: Vec<TransiterStop>,
    pub next_id: Option<String>,
}

pub struct ReqWestTransiterClient {
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


impl TransiterWebAPI for ReqWestTransiterClient {
    async fn get_transiter_entrypoint(
        &self,
    ) -> Result<EntrypointReply, Box<dyn core::error::Error + Send + Sync>> {
        let response = reqwest::get(self.get_base_uri()).await?;
        match response.error_for_status() {
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
    ) -> Result<ListStopsReply, Box<dyn core::error::Error + Send + Sync>> {
        todo!()
    }

    async fn get_stop(
        &self,
        request: &GetStopRequest,
    ) -> Result<TransiterStop, Box<dyn core::error::Error + Send + Sync>> {
        let stop_id: &str = request.stop_id.as_ref();
        let stop_url = format!("{}{}/stops/{}", self.server_uri, self.agency_url, stop_id);
        println!("stop_url: {}", stop_url);
        let response = reqwest::get(stop_url).await?;

        match response.error_for_status() {
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


pub trait TransiterWebAPI {
    fn get_transiter_entrypoint(
        &self,
    ) -> impl core::future::Future<Output =  Result<EntrypointReply, Box<dyn core::error::Error + Send + Sync>>>;

    fn list_stops(
        &self,
        request: &ListStopsRequest,
    ) -> impl core::future::Future<Output = Result<ListStopsReply, Box<dyn core::error::Error + Send + Sync>>>;

    fn get_stop(
        &self,
        request: &GetStopRequest,
    ) ->  impl core::future::Future<Output =Result<TransiterStop, Box<dyn core::error::Error + Send + Sync>>>;
}
