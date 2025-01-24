use reqwest::Error;
use serde::Deserialize;

use crate::constants::RPC_URL;

#[derive(Debug, Deserialize)]
pub struct TickInfo {
    #[serde(rename = "tickInfo")]
    pub tick_info: TickInfoData,
}

#[derive(Debug, Deserialize)]
pub struct TickInfoData {
    #[serde(rename = "tick")]
    pub tick: i64,
    #[serde(rename = "duration")]
    pub duration: i64,
    #[serde(rename = "epoch")]
    pub epoch: i32,
    #[serde(rename = "initialTick")]
    pub initial_tick: i64,
}

pub async fn fetch_tick_info() -> Result<TickInfoData, Error> {
    let response: reqwest::Response = reqwest::get(format!("{}/v1/tick-info", RPC_URL))
        .await?;
    let result: TickInfo = response.json::<TickInfo>().await?;
    Ok(result.tick_info)
}
