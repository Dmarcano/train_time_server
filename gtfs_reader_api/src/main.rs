pub mod gtfs_realtime_api; //

use tokio;

use chrono::{DateTime, FixedOffset, NaiveDate, Timelike};
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

slint::slint! {
    export component MyTile inherits Rectangle {
        width: 64px;
        height: 64px;
        background:rgb(48, 141, 121);
    }

    export component HelloWorld inherits Window {
        width: 512px;
        height: 512px;
        background: #3960D5;
        Text {
            text: "hello world";
            color: green;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_BACKTRACE", "1");
    // HelloWorld::new().unwrap().run().unwrap();

    // Make the GET request to the Transitter demo API
    let api = TransiterRealTimeAPI::from_example_server(DemogAgencies::NycMetro);
    let out = api.get_outgoing_trips("Queensboro Plaza").await?;
    let timezone = FixedOffset::west_opt(5 * 3600).unwrap();

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
                            .map(|timestamp| DateTime::from_timestamp(timestamp as i64, 0))
                            .map(|maybe_date_time| {
                                maybe_date_time
                                    .map(|utc_time| utc_time.with_timezone(&timezone))
                                    .map(|date_time| {
                                        format!(
                                            " {:02}:{:02}:{:02}",
                                            date_time.hour(),
                                            date_time.minute(),
                                            date_time.second()
                                        )
                                    })
                            });
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
