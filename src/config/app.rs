use axum::Router;
use axum::routing::{get, post};
use crate::{create_user, root};

pub fn configure_routes() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
}