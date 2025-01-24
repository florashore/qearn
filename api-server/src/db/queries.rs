use crate::db::prisma::{self, PrismaClient};
use prisma_client_rust::QueryError;

pub async fn update_tick_info(client: &PrismaClient, tick: i64, duration: i64, epoch: i32, initial_tick: i64) -> Result<prisma::tick_info::Data, QueryError> {
    let old_tick_info = client.tick_info().find_first(vec![])
        .order_by(prisma::tick_info::created_at::order(prisma_client_rust::Direction::Desc))
        .exec()
        .await?;

    match old_tick_info {
        Some(existing_tick) => {
            client.tick_info().update(
                prisma::tick_info::id::equals(existing_tick.id),
                vec![
                    prisma::tick_info::tick::set(tick),
                    prisma::tick_info::duration::set(duration),
                    prisma::tick_info::epoch::set(epoch),
                    prisma::tick_info::initial_tick::set(initial_tick),
                ],
            ).exec().await
        },
        None => {
            client.tick_info().create(
                tick,
                duration,
                epoch,
                initial_tick,
                vec![],
            ).exec().await
        }
    }
}

pub async fn get_tick_info(client: &PrismaClient) -> Result<Vec<prisma::tick_info::Data>, QueryError> {
    client.tick_info().find_many(vec![])
        .exec()
        .await
}
