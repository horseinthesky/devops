use autometrics::{autometrics, prometheus_exporter};
use axum::{routing::get, Json, Router};
use serde::Serialize;
use serde_json::{json, Value};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

mod otlp;

mod devices;
use devices::get_devices;

mod images;
use images::get_images;

const DEFAULT_PORT: &str = "8000";
const DEFAULT_HOST: &str = "0.0.0.0";

#[derive(Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
enum Status {
    OK,
    ERROR,
}

#[derive(Serialize)]
struct AppResponse<'a> {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
}

impl<'a> AppResponse<'a> {
    fn ok() -> Self {
        Self {
            status: Status::OK,
            message: None,
            result: None,
        }
    }

    fn error() -> Self {
        Self {
            status: Status::ERROR,
            message: None,
            result: None,
        }
    }

    fn with_message(self, message: &'a str) -> Self {
        Self {
            status: self.status,
            message: Some(message),
            result: self.result,
        }
    }

    fn with_result(self, result: Option<Value>) -> Self {
        Self {
            status: self.status,
            message: self.message,
            result,
        }
    }
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

async fn health() -> Json<AppResponse<'static>> {
    Json(AppResponse::ok().with_message("up"))
}

#[autometrics]
#[tracing::instrument]
async fn devices() -> Json<AppResponse<'static>> {
    Json(AppResponse::ok().with_result(Some(json!(get_devices()))))
}

#[autometrics]
#[tracing::instrument]
async fn images() -> Json<AppResponse<'static>> {
    get_images().await;

    Json(AppResponse::ok().with_message("saved"))
}
