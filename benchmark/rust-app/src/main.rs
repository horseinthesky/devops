use autometrics::{autometrics, prometheus_exporter};
use axum::{http::Request, routing::get, Json, Router};
use serde::Serialize;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, info, info_span, warn};

mod otlp;

mod devices;
use devices::{get_devices, Device};

mod images;
use images::get_images;

#[derive(Serialize)]
struct Response<'a> {
    status: &'a str,
    message: &'a str,
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
    // .route(
    //     "/metrics",
    //     get(|| async { "Hello, World!" }),
    // );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<Response<'static>> {
    Json(Response {
        status: "ok",
        message: "up",
    })
}

#[tracing::instrument]
#[autometrics]
async fn devices() -> Json<Vec<Device<'static>>> {
    info_span!("internal").in_scope(|| {
        warn!("do stuff inside internal");
    });

    Json(get_devices())
}

#[tracing::instrument]
async fn images() -> Json<Response<'static>> {
    get_images().await;

    Json(Response {
        status: "ok",
        message: "saved",
    })
}
