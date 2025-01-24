use reqwest::Error;
use serde::Deserialize;
use serde_json;

use crate::constants::RPC_URL;

#[derive(Debug, Deserialize)]
pub struct TickInfo {
    #[serde(rename = "tickInfo")]
    tick_info: TickInfoData,
}

#[derive(Debug, Deserialize)]
pub struct TickInfoData {
    #[serde(rename = "tick")]
    tick: u64,
    #[serde(rename = "duration")]
    duration: u64,
    #[serde(rename = "epoch")]
    epoch: u32,
    #[serde(rename = "initialTick")]
    initial_tick: u64,
}

pub async fn fetch_tick_info() -> Result<TickInfoData, Error> {
    let response: reqwest::Response = reqwest::get(format!("{}/v1/tick-info", RPC_URL))
        .await?;
    let result: TickInfo = response.json::<TickInfo>().await?;
    Ok(result.tick_info)
}
