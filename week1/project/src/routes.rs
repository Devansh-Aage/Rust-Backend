use crate::handlers::greet::greet_user;
use crate::handlers::time::get_time;
use crate::{handlers::health::health_check, state::AppState};
use axum::Router;
use axum::routing::get;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/greet/:name", get(greet_user))
        .route("/time", get(get_time))
        .with_state(app_state)
}
