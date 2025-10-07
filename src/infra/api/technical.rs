use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Health {
    status: String,
}

pub async fn health() -> (StatusCode, Json<Health>) {
    (StatusCode::CREATED, Json(Health { status: "ok".to_string() }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_should_return_ok() {
        let (status, json) = health().await;

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(json.0, Health { status: "ok".to_string() });
    }
}