use axum::{routing::get, Json, Router};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions as semconv;
use serde::Serialize;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, info, info_span, warn};
use tracing_subscriber::prelude::*;

mod devices;
use devices::{get_devices, Device};

mod images;
use images::get_images;

#[derive(Serialize)]
struct Response<'a> {
    status: &'a str,
    message: &'a str,
}

fn init_otlp() {
    // Create OTLP gRPC exporter
    let exporter = opentelemetry_otlp::new_exporter()
        .grpcio()
        .with_timeout(std::time::Duration::from_secs(3))
        .with_endpoint("localhost:4317");

    // Create a resource
    let resource = Resource::new([
        KeyValue::new(
            semconv::resource::SERVICE_NAME,
            "rust-app",
        ),
        KeyValue::new(
            semconv::resource::SERVICE_VERSION,
            "0.1.0",
        ),
    ]);

    // Create tracer
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            opentelemetry_sdk::trace::config().with_resource(resource),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("should create a tracer");

    // Create an opentelemetry layer
    let otlp_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Create a subscriber
    let subscriber = tracing_subscriber::Registry::default().with(otlp_layer);

    // Set the global subscriber for the app
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    init_otlp();

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
            get(|| async { "Hello, World!" }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

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
async fn devices() -> Json<Vec<Device<'static>>> {
    // info_span!("internal").in_scope(|| {
    //     warn!("do stuff inside internal");
    // });

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
