use crate::{create_user, root};
use axum::routing::{get, post};
use axum::Router;
use crate::infra::api::technical::health;

pub fn configure_routes() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        // TECHNICAL
        .route("/health", get(health))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_app_router() {
        let _app = configure_routes();
        // If we got here, building the router succeeded.
    }
}