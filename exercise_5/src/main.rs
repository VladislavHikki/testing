use axum::{routing::get, Router};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{mysql::MySqlPool, FromRow};
use std::env;
mod model;
use model::task::Task;

async fn get_tasks() -> String {
    "Hello World222122".to_string()
}

async fn add_task() -> String {
    "Add task".to_string()
}

async fn edit_task() -> String {
    "Edit task".to_string()
}

async fn del_task() -> String {
    "Delete task".to_string()
}

async fn change_example() -> String {
    "Change Example".to_string()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the db");

    let app = Router::new()
        .route("/tasks", get(|| async { get_tasks().await }))
        .route("/add/task", get(|| async { add_task().await }))
        .route("/edit/task", get(|| async { edit_task().await }))
        .route("/del/task", get(|| async { del_task().await }))
        .route("/change-example", get(|| async { change_example().await }))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
