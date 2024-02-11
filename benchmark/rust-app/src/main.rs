use axum::{routing::get, Json, Router};
use serde::Serialize;

mod devices;
use devices::{Device, get_devices};

#[derive(Serialize)]
struct Response<'a> {
    status: &'a str,
    message: &'a str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/devices", get(devices))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Response<'static>> {
    Json(Response {
        status: "ok",
        message: "up",
    })
}

async fn devices() -> Json<Vec<Device<'static>>> {
    Json(get_devices())
}
