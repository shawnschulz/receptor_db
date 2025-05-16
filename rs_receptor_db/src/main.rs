use axum::{
        routing::{get, post},
        Router,
        Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(homepage))
        .route("/get_var", get(get_var));
    let addr = SocketAddr::from(([192, 168, 4, 41], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn homepage() -> &'static str {
    "This is the homepage for receptor_db"
}
async fn get_var() -> &'static str {
    "This is the page for get_var"
}
