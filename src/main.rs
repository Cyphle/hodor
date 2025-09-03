use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = build_app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn build_app() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
}

#[cfg(not(test))]
async fn get_user(pool: &Pool<Postgres>, id: i64) -> Result<(i64,), sqlx::Error> {
    // Postgres uses positional parameters like $1
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

// For tests, use an in-memory Sqlite database so no external DB is required.
#[cfg(test)]
async fn get_user(pool: &sqlx::SqlitePool, id: i64) -> Result<(i64,), sqlx::Error> {
    // Sqlite uses ?1 style parameters
    let row: (i64,) = sqlx::query_as("SELECT ?1")
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

    #[test]
    fn can_build_app_router() {
        let _app = build_app();
        // If we got here, building the router succeeded.
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