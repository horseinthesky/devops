use autometrics::{autometrics, prometheus_exporter};
use axum::{routing::get, Json, Router};
use serde::Serialize;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, info, info_span, warn};

mod otlp;

mod devices;
use devices::{get_devices, Device};

mod images;
use images::get_images;

const DEFAULT_PORT: &str = "8000";
const DEFAULT_HOST: &str = "0.0.0.0";

#[derive(Serialize)]
struct Response<'a> {
    status: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'a str>,
}

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    otlp::init_otlp();

    let app = Router::new()
        .route("/api/devices", get(devices))
        .route("/api/images", get(images))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true)),
        )
        .route("/health", get(health))
        .route(
            "/metrics",
            get(|| async { prometheus_exporter::encode_http_response() }),
        );

    let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    let host = std::env::var("PORT").unwrap_or(DEFAULT_HOST.to_string());

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<Response<'static>> {
    Json(Response {
        status: "ok",
        message: None,
    })
}

#[autometrics]
#[tracing::instrument]
async fn devices() -> Json<Vec<Device<'static>>> {
    Json(get_devices())
}

#[autometrics]
#[tracing::instrument]
async fn images() -> Json<Response<'static>> {
    get_images().await;

    Json(Response {
        status: "ok",
        message: Some("saved"),
    })
}
