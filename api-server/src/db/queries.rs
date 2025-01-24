use crate::db::prisma::{self, PrismaClient};
use crate::service::rpc_service::TxInfoData;
use prisma_client_rust::QueryError;

pub async fn update_tick_info(
    client: &PrismaClient,
    tick: i64,
    duration: i64,
    epoch: i32,
    initial_tick: i64,
) -> Result<prisma::tick_info::Data, QueryError> {
    let old_tick_info = client
        .tick_info()
        .find_first(vec![])
        .order_by(prisma::tick_info::created_at::order(
            prisma_client_rust::Direction::Desc,
        ))
        .exec()
        .await?;

    match old_tick_info {
        Some(existing_tick) => {
            client
                .tick_info()
                .update(
                    prisma::tick_info::id::equals(existing_tick.id),
                    vec![
                        prisma::tick_info::tick::set(tick),
                        prisma::tick_info::duration::set(duration),
                        prisma::tick_info::epoch::set(epoch),
                        prisma::tick_info::initial_tick::set(initial_tick),
                    ],
                )
                .exec()
                .await
        }
        None => {
            client
                .tick_info()
                .create(tick, duration, epoch, initial_tick, vec![])
                .exec()
                .await
        }
    }
}

pub async fn get_tick_info(
    client: &PrismaClient,
) -> Result<Vec<prisma::tick_info::Data>, QueryError> {
    client.tick_info().find_many(vec![]).exec().await
}

pub async fn create_tx_info(
    client: &PrismaClient,
    tx_info: &TxInfoData,
    timestamp: &str,
    money_flew: bool,
) -> Result<prisma::tx_info::Data, QueryError> {
    client
        .tx_info()
        .create(
            tx_info.source_id.to_string(),
            tx_info.dest_id.to_string(),
            tx_info.amount.parse::<i64>().unwrap(),
            tx_info.tick_number,
            tx_info.input_type,
            tx_info.input_size,
            tx_info.input_hex.clone(),
            tx_info.signature_hex.clone(),
            tx_info.tx_id.clone(),
            timestamp.to_string(),
            money_flew,
            vec![],
        )
        .exec()
        .await
}

pub async fn upsert_balance(
    client: &PrismaClient,
    address: &str,
    tick: i64,
    balance: i64,
) -> Result<prisma::balance::Data, QueryError> {
    client
        .balance()
        .upsert(
            prisma::balance::address_tick(address.to_string(), tick),
            prisma::balance::create(address.to_string(), balance, tick, vec![]),
            vec![prisma::balance::balance::set(balance)],
        )
        .exec()
        .await
}
