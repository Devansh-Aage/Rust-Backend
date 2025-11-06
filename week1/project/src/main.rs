use tokio::net::TcpListener;
use tracing_subscriber::fmt;

use crate::{routes::create_router, shutdown::graceful_shutdown, state::AppState};

mod routes;
mod handlers {
    pub mod greet;
    pub mod health;
    pub mod time;
}
mod shutdown;
mod state;

#[tokio::main]
async fn main() {
    fmt().pretty().init();

    let app_state = AppState::new();

    let app = create_router(app_state.clone());

    let addr = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running at http://{}", addr.local_addr().unwrap());

    axum::serve(addr, app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}
