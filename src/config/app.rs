use crate::{create_user, db_check, root};
use axum::routing::{get, post};
use axum::{Extension, Router};
use crate::infra::api::technical::health;
use sqlx::{Pool, Postgres};

pub fn configure_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/db-check", get(db_check))
        // TECHNICAL
        .route("/health", get(health))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_app_router() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:password@localhost/test")
            .expect("failed to build pg pool");
        let _app = configure_routes(pool);
        // If we got here, building the router succeeded.
    }
}
