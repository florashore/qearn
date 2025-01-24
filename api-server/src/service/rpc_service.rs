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

#[derive(Debug, Deserialize)]
pub struct Balance {
    #[serde(rename = "balance")]
    pub balance: BalanceData,
}

#[derive(Debug, Deserialize)]
pub struct BalanceData {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "validForTick")]
    pub valid_for_tick: i64,
    #[serde(rename = "latestIncomingTransferTick")]
    pub latest_incoming_transfer_tick: i64,
    #[serde(rename = "latestOutgoingTransferTick")]
    pub latest_outgoing_transfer_tick: i64,
    #[serde(rename = "incomingAmount")]
    pub incoming_amount: String,
    #[serde(rename = "outgoingAmount")]
    pub outgoing_amount: String,
    #[serde(rename = "numberOfIncomingTransfers")]
    pub number_of_incoming_transfers: i64,
    #[serde(rename = "numberOfOutgoingTransfers")]
    pub number_of_outgoing_transfers: i64,
}

pub async fn fetch_balance(address: &str) -> Result<BalanceData, Error> {
    let response: reqwest::Response = reqwest::get(format!("{}/v1/balance/{}", RPC_URL, address)).await?;
    let result: Balance = response.json::<Balance>().await?;
    Ok(result.balance)
}

#[derive(Debug, Deserialize)]
pub struct TxInfo {
    #[serde(rename = "transaction")]
    pub transaction: TxInfoData,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "moneyFlew")]
    pub money_flew: bool,
}

#[derive(Debug, Deserialize)]
pub struct TxInfoData {
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "destId")]
    pub dest_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "tickNumber")]
    pub tick_number: i64,
    #[serde(rename = "inputType")]
    pub input_type: i32,
    #[serde(rename = "inputSize")]
    pub input_size: i32,
    #[serde(rename = "inputHex")]
    pub input_hex: String,
    #[serde(rename = "signatureHex")]
    pub signature_hex: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
}

pub async fn fetch_tx_info(tx_id: &str) -> Result<TxInfo, Error> {
    let response: reqwest::Response = reqwest::get(format!("{}/v2/transactions/{}", RPC_URL, tx_id)).await?;
    let result: TxInfo = response.json::<TxInfo>().await?;
    Ok(result)
}


#[derive(Debug, Deserialize)]
pub struct TxHistoryResponse {
    pub transactions: Vec<TxHistoryItem>,
}

#[derive(Debug, Deserialize)]
pub struct TxHistoryItem {
    #[serde(rename = "tickNumber")]
    pub tick_number: i64,
    pub identity: String,
    pub transactions: Vec<TxHistoryTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct TxHistoryTransaction {
    pub transaction: TxInfoData,
    pub timestamp: String,
    #[serde(rename = "moneyFlew")]
    pub money_flew: bool,
}

pub async fn fetch_tx_history(address: &str, start_tick: i64, end_tick: i64) -> Result<TxHistoryResponse, Error> {
    let response: reqwest::Response = reqwest::get(format!("{}/v2/identities/{}/transfers?startTick={}&endTick={}", RPC_URL, address, start_tick, end_tick)).await?;
    let result: TxHistoryResponse = response.json::<TxHistoryResponse>().await?;
    Ok(result)
}

pub async fn fetch_richlist(page: i64, page_size: i64) -> Result<TxInfo, Error> {
    let response: reqwest::Response = reqwest::get(format!("{}/v1/richlist?page={}&pageSize={}", RPC_URL, page, page_size)).await?;
    let result: TxInfo = response.json::<TxInfo>().await?;
    Ok(result)
}
