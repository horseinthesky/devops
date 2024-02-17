use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions as semconv;
use tracing_subscriber::prelude::*;

pub fn init_otlp() {
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
