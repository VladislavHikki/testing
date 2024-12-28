use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub label: String,
    pub description: Option<String>,
    pub is_example: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
