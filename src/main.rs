mod config;
mod tools;

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::config::app::configure_routes;
use crate::tools::capitalize::capitalize;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let application_config = config::config::AppConfig::new().unwrap();
    config::logger::config(capitalize(&application_config.logging.level));
    start().await?;



    // ===========> BELOW ARE SOME EXAMPLES
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;

    let row: (i64,) = get_user(&pool, 150_i64).await?;

    assert_eq!(row.0, 150);

    // run migrations at startup
    sqlx::migrate!("./migrations").run(&pool).await?;


    Ok(())
}

async fn start() -> Result<(), Error> {
    let app = configure_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_user(pool: &Pool<Postgres>, id: i64) -> Result<(i64,), sqlx::Error> {
    // Postgres uses positional parameters like $1
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_user(pool: &sqlx::SqlitePool, id: i64) -> Result<(i64,), sqlx::Error> {
        // Sqlite uses ?1 style parameters
        let row: (i64,) = sqlx::query_as("SELECT ?1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(row)
    }

    #[tokio::test]
    async fn root_returns_hello_world() {
        let resp = root().await;
        assert_eq!(resp, "Hello, World!");
    }

    #[tokio::test]
    async fn create_user_returns_201_and_payload_username() {
        let payload = CreateUser { username: "alice".to_string() };
        let (status, Json(user)) = create_user(Json(payload)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(user.id, 1337);
        assert_eq!(user.username, "alice");
    }

    // This test uses sqlx::test which provides an in-memory Sqlite pool.
    // It does not require any external database.
    #[sqlx::test]
    async fn get_user_returns_bound_value(pool: sqlx::SqlitePool) {
        let val = 150_i64;
        let row = get_user(&pool, val).await.expect("get_user failed");
        assert_eq!(row.0, val);
    }
}