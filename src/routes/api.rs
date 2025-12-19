use crate::controllers::controller::{add_user, get_all_users};
use crate::db::DbPool;
use axum::{
    Router,
    routing::{get, post},
};

pub fn api_routes() -> Router<DbPool> {
    Router::new()
        .route("/users", post(add_user))
        .route("/users", get(get_all_users))
}
