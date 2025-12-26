mod config;
mod tools;
mod infra;

use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use crate::config::app::configure_routes;
use crate::config::database::connect;
use crate::tools::capitalize::capitalize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let application_config = config::config::AppConfig::new()?;
    config::logger::init_logger(capitalize(&application_config.logging.level));

    let db_connection = connect(&application_config.database).await;

    // run migrations at startup
    sqlx::migrate!("./migrations").run(&db_connection).await?;

    start(db_connection).await?;


    Ok(())
}

async fn start(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    let app = configure_routes(pool);
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

async fn db_check(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<i64>, (StatusCode, String)> {
    let val: i64 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(val))
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
