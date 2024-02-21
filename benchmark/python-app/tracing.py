import os

from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import SERVICE_NAME, Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor

DEFAULT_OTLP_ENDPOINT = "localhost:4317"
OTLP_ENDPOINT = os.getenv("OTLP_ENDPOINT", DEFAULT_OTLP_ENDPOINT)

# Start configuring OpenTelemetry.
resource = Resource(attributes={SERVICE_NAME: "python-app"})
provider = TracerProvider(resource=resource)

# OpenTelemetry Protocol Exporter (OTLP) Exporter
processor = BatchSpanProcessor(
    OTLPSpanExporter(
        # Change default HTTPS -> HTTP.
        insecure=True,
        # Update default OTLP reciver endpoint.
        endpoint=OTLP_ENDPOINT,
    )
)
provider.add_span_processor(processor)

# Sets the global default tracer provider.
trace.set_tracer_provider(provider)

# Creates a tracer from the global tracer provider.
tracer = trace.get_tracer("internal")
