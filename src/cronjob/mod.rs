use crate::service::rpc_service::fetch_tick_info;
use crate::db::queries::create_tick_info;
use crate::Db;
use std::error::Error;

pub async fn cronjob(db: &Db) -> Result<(), Box<dyn Error>> {
    tracing::info!("Running cronjob");
    match fetch_tick_info().await {
        Ok(tick_info) => {
            tracing::info!("Tick info: {:?}", tick_info);
            create_tick_info(&db.pool, tick_info.tick, tick_info.duration, tick_info.epoch, tick_info.initial_tick).await?;
            Ok(())
        }
        Err(err) => {
            tracing::error!("Failed to fetch tick info: {}", err);
            Err(Box::new(err))
        }
    }
}
