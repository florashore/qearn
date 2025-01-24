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
    let current_tick = match fetch_tick_info().await {
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
            tick_info.tick
        }
        Err(err) => {
            tracing::error!("Failed to fetch tick info: {}", err);
            return Err(Box::new(err));
        }
    };

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
    let mut start_tick = match latest_tx {
        Some(tx) => tx.tick,
        None => EPOCH_138_START_TICK,
    };

    loop {
        // Don't fetch more than current tick
        let end_tick = (start_tick + 20).min(current_tick);

        // Only proceed if there are new ticks to process
        if start_tick > end_tick {
            break;
        }

        // 4. Fetch transaction history
        let tx_history = fetch_tx_history(QEARN_ADDRESS, start_tick + 1, end_tick).await?;

        if !tx_history.transactions.is_empty() {
            // 5. Process each transaction
            for tx_item in tx_history.transactions {
                for tx in tx_item.transactions {
                    // Get current timestamp
                    let timestamp = chrono::Utc::now().to_rfc3339();

                    // Create transaction record
                    create_tx_info(&client, &tx.transaction, &timestamp, tx.money_flew).await?;

                    // Update balances if money actually transferred
                    if tx.money_flew {
                        // Get current source balance
                        let source_balance = client
                            .balance()
                            .find_first(vec![
                                prisma::balance::address::equals(tx.transaction.source_id.clone()),
                                prisma::balance::tick::lte(tx.transaction.tick_number),
                            ])
                            .order_by(prisma::balance::tick::order(
                                prisma_client_rust::Direction::Desc,
                            ))
                            .exec()
                            .await?
                            .map(|b| b.balance)
                            .unwrap_or(0);

                        // Get current destination balance
                        let dest_balance = client
                            .balance()
                            .find_first(vec![
                                prisma::balance::address::equals(tx.transaction.dest_id.clone()),
                                prisma::balance::tick::lte(tx.transaction.tick_number),
                            ])
                            .order_by(prisma::balance::tick::order(
                                prisma_client_rust::Direction::Desc,
                            ))
                            .exec()
                            .await?
                            .map(|b| b.balance)
                            .unwrap_or(0);

                        let amount = tx.transaction.amount.parse::<i64>()?;

                        // Update source balance
                        upsert_balance(
                            &client,
                            &tx.transaction.source_id,
                            tx.transaction.tick_number,
                            source_balance - amount,
                        )
                        .await?;

                        // Update destination balance
                        upsert_balance(
                            &client,
                            &tx.transaction.dest_id,
                            tx.transaction.tick_number,
                            dest_balance + amount,
                        )
                        .await?;
                    }
                }
            }
            // Update start_tick to end_tick for the next iteration
            start_tick = end_tick;
        } else {
            // If no transactions found, fetch the next 20 ticks
            tracing::info!(
                "No transactions found between ticks {} and {}. Fetching next 20 ticks.",
                start_tick + 1,
                end_tick
            );

            // Get current balance
            let current_balance = client
                .balance()
                .find_first(vec![
                    prisma::balance::address::equals(QEARN_ADDRESS.to_string()),
                    prisma::balance::tick::lte(end_tick),
                ])
                .order_by(prisma::balance::tick::order(
                    prisma_client_rust::Direction::Desc,
                ))
                .exec()
                .await?
                .map(|b| b.balance)
                .unwrap_or(0);

            // Update balance with same amount to mark tick as processed
            upsert_balance(&client, QEARN_ADDRESS, end_tick, current_balance).await?;

            // Move to the next tick window
            start_tick = end_tick;
        }
    }

    // 6. Wait for next iteration
    sleep(Duration::from_secs(10)).await;
    Ok(())
}
