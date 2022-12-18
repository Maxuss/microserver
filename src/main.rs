use std::net::SocketAddr;

use axum::{routing::get_service, Router};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let router = Router::new().nest_service(
        "/",
        get_service(ServeDir::new("./www").not_found_service(ServeFile::new("./www/404.html")))
            .handle_error(|error| async move { format!("Error: {error}") }),
    );

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 80)))
        .serve(router.into_make_service())
        .await
        .unwrap();
}
