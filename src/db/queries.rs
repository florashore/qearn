use sqlx::query_as;
use crate::db::models::{User, Project, TickInfo};

pub async fn create_user(pool: &sqlx::PgPool, name: &str, email: &str) -> sqlx::Result<User> {
    query_as!(
        User,
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email, created_at
        "#,
        name,
        email
    )
    .fetch_one(pool)
    .await
}

pub async fn create_project(pool: &sqlx::PgPool, title: &str, user_id: i32) -> sqlx::Result<Project> {
    query_as!(
        Project,
        r#"
        INSERT INTO projects (title, user_id)
        VALUES ($1, $2)
        RETURNING id, title, user_id, created_at
        "#,
        title,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_id(pool: &sqlx::PgPool, user_id: i32) -> sqlx::Result<User> {
    query_as!(
        User,
        r#"
        SELECT id, name, email, created_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_tick_info(pool: &sqlx::PgPool, tick: i64, duration: i64, epoch: i32, initial_tick: i64) -> sqlx::Result<TickInfo> {
    query_as!(
        TickInfo,
        r#"INSERT INTO tick_info (tick, duration, epoch, initial_tick) VALUES ($1, $2, $3, $4) RETURNING tick, duration, epoch, initial_tick"#,
        tick, duration, epoch, initial_tick
    )
    .fetch_one(pool)
    .await
}

pub async fn get_tick_info(pool: &sqlx::PgPool) -> sqlx::Result<TickInfo> {
    query_as!(
        TickInfo,
        r#"SELECT tick, duration, epoch, initial_tick FROM tick_info"#
    )
    .fetch_one(pool)
    .await
}

