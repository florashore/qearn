use crate::constants::{EPOCH_138_START_TICK, QEARN_ADDRESS};
use crate::db::prisma::{self, PrismaClient};
use crate::db::queries::{create_tx_info, update_tick_info, upsert_balance};
use crate::service::rpc_service::{fetch_tick_info, fetch_tx_history};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

pub async fn cronjob(client: &PrismaClient) -> Result<(), Box<dyn Error>> {
    tracing::info!("Running cronjob");

    // 1. Fetch and update tick info for monitoring purposes
    match fetch_tick_info().await {
        Ok(tick_info) => {
            tracing::info!("Tick info: {:?}", tick_info);
            update_tick_info(
                &client,
                tick_info.tick,
                tick_info.duration,
                tick_info.epoch,
                tick_info.initial_tick,
            )
            .await?;
        }
        Err(err) => {
            tracing::error!("Failed to fetch tick info: {}", err);
            return Err(Box::new(err));
        }
    }

    // 2. Query latest processed tick from tx_info table
    let latest_tx = client
        .tx_info()
        .find_first(vec![])
        .order_by(prisma::tx_info::tick::order(
            prisma_client_rust::Direction::Desc,
        ))
        .exec()
        .await?;

    // 3. Determine start tick for fetching transaction history
    let start_tick = match latest_tx {
        Some(tx) => tx.tick + 1,
        None => EPOCH_138_START_TICK,
    };

    // 4. Fetch transaction history (20 ticks at a time)
    let tx_history = fetch_tx_history(QEARN_ADDRESS, start_tick, start_tick + 20).await?;

    // 5. Process each transaction
    for tx_item in tx_history.transactions {
        for tx in tx_item.transactions {
            // Get current timestamp
            let timestamp = chrono::Utc::now().to_rfc3339();

            // Create transaction record
            create_tx_info(&client, &tx.transaction, &timestamp, tx.money_flew).await?;

            // Update balances if money actually transferred
            if tx.money_flew {
                // Subtract from source
                upsert_balance(
                    &client,
                    &tx.transaction.source_id,
                    tx.transaction.tick_number,
                    -tx.transaction.amount.parse::<i64>()?,
                )
                .await?;

                // Add to destination
                upsert_balance(
                    &client,
                    &tx.transaction.dest_id,
                    tx.transaction.tick_number,
                    tx.transaction.amount.parse::<i64>()?,
                )
                .await?;
            }
        }
    }

    // 6. Wait for next iteration
    sleep(Duration::from_secs(10)).await;
    Ok(())
}
