use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub user_id: i64, // Foreign key to User
    pub created_at: DateTime<Utc>
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TickInfo {
    pub tick: i64,
    pub duration: i64,
    pub epoch: i32,
    pub initial_tick: i64,
}

