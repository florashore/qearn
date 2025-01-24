use crate::service::rpc_service::fetch_tick_info;
use crate::db::queries::upsert_tick_info;
use crate::db::prisma::PrismaClient;
use std::error::Error;

pub async fn cronjob(client: &PrismaClient) -> Result<(), Box<dyn Error>> {
    tracing::info!("Running cronjob");
    match fetch_tick_info().await {
        Ok(tick_info) => {
            tracing::info!("Tick info: {:?}", tick_info);
            upsert_tick_info(&client, tick_info.tick, tick_info.duration, tick_info.epoch, tick_info.initial_tick).await?;
            Ok(())
        }
        Err(err) => {
            tracing::error!("Failed to fetch tick info: {}", err);
            Err(Box::new(err))
        }
    }
}
