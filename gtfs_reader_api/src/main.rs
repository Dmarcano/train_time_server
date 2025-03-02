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
    import { LineEdit, Button } from "std-widgets.slint";

    export component StopTime {
        in property <int> num_hours;
        in property <int> num_minutes;
        in property <int> num_seconds;
        Rectangle {
            Text {
                text: "hours";
            }
        }
    }   

    export component StopView {
        in property <string> stop_name;
        Text {
            text: stop_name;
            color: #0443bf;
        }
    }

    export component HelloWorld inherits Window {
        // width: 512px;
        // height: 512px;
        background: #e3e4e6;

        VerticalLayout {
        LineEdit {}
            StopView {
                stop_name: "R09";
            }
        // if TextInputInterface.text-input-focused: VKB {}


        label := Text {
            text: "Button not clicked";
        }

        Button {
            text: "Click Me";
            clicked => {
                label.text = " Button clicked";
            }
        }

    }

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_BACKTRACE", "1");

    let local_time = chrono::Utc::now();

    // Make the GET request to the Transitter demo API
    let api = TransiterRealTimeAPI::from_example_server(DemogAgencies::NycMetro);
    let outgoing_trips = api.get_outgoing_trips("Queensboro Plaza").await?;

    let outv2 = outgoing_trips
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
                                maybe_date_time.map(|utc_time| utc_time - local_time).map(
                                    |date_time| {
                                        format!(
                                            "{:02}:{:02}:{:02}",
                                            date_time.num_hours(),
                                            date_time.num_minutes(),
                                            date_time.num_seconds()
                                        )
                                    },
                                )
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
