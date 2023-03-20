mod config;

use serde::{Deserialize, Serialize};

// https://celestrak.org/satcat/satcat-format.php
#[derive(Serialize, Deserialize, Debug)]
pub struct SatelliteData {
    #[serde(rename(deserialize = "OBJECT_NAME"))]
    pub object_name: String,
    #[serde(rename(deserialize = "OBJECT_ID"))]
    pub object_id: String,
    #[serde(rename(deserialize = "NORAD_CAT_ID"))]
    pub norad_cat_id: Option<u64>,
    #[serde(rename(deserialize = "OBJECT_TYPE"))]
    pub object_type: String,
    #[serde(rename(deserialize = "OPS_STATUS_CODE"))]
    pub ops_status_code: String,
    #[serde(rename(deserialize = "OWNER"))]
    pub owner: String,
    #[serde(rename(deserialize = "LAUNCH_DATE"))]
    pub launch_date: String,
    #[serde(rename(deserialize = "LAUNCH_SITE"))]
    pub launch_site: String,
    #[serde(rename(deserialize = "DECAY_DATE"))]
    pub decay_date: String,
    #[serde(rename(deserialize = "PERIOD"))]
    pub period: Option<f32>,
    #[serde(rename(deserialize = "INCLINATION"))]
    pub inclination: Option<f32>,
    #[serde(rename(deserialize = "APOGEE"))]
    pub apogee: Option<f32>,
    #[serde(rename(deserialize = "PERIGEE"))]
    pub perigee: Option<f32>,
    #[serde(rename(deserialize = "RCS"))]
    pub rcs: Option<f32>,
    #[serde(rename(deserialize = "DATA_STATUS_CODE"))]
    pub data_status_code: String,
    #[serde(rename(deserialize = "ORBIT_CENTER"))]
    pub orbit_center: String,
    #[serde(rename(deserialize = "ORBIT_TYPE"))]
    pub orbit_type: String,
}

#[tokio::main]
async fn main() {
    let _config = config::Config::new();

    let response = ureq::get("https://celestrak.org/satcat/records.php?GROUP=active")
        .call()
        .unwrap();

    let sats: Vec<SatelliteData> = response.into_json().unwrap();

    dbg!(sats);
}
