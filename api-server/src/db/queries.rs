use crate::db::prisma::{self, PrismaClient};
use prisma_client_rust::QueryError;

pub async fn create_user(client: &PrismaClient, name: &str, email: &str) -> Result<prisma::user::Data, QueryError> {
    client.user().create(
        name.to_string(),
        email.to_string(),
        vec![],
    ).exec().await
}

pub async fn create_project(client: &PrismaClient, title: &str, user_id: i32) -> Result<prisma::project::Data, QueryError> {
    client.project().create(
        title.to_string(),
        prisma::user::id::equals(user_id),
        vec![],
    ).exec().await
}

pub async fn get_user_by_id(client: &PrismaClient, user_id: i32) -> Result<Option<prisma::user::Data>, QueryError> {
    client.user().find_unique(
        prisma::user::id::equals(user_id)
    ).exec().await
}

pub async fn upsert_tick_info(client: &PrismaClient, tick: i64, duration: i64, epoch: i32, initial_tick: i64) -> Result<prisma::tick_info::Data, QueryError> {
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
