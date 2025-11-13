use crate::routes::create_router;
use axum;
use tokio::net::TcpListener;

mod routes;
mod handlers {
    pub mod body;
    pub mod query;
}
mod error;

#[tokio::main]
async fn main() {
    let addr = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let app = create_router();

    axum::serve(addr, app.into_make_service()).await.unwrap();
}
