from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import SERVICE_NAME, Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor

from config import Config

# Load app config from yaml file.
config = Config.from_yaml("config.yaml")

# Start configuring OpenTelemetry.
resource = Resource(attributes={SERVICE_NAME: "python-app"})
provider = TracerProvider(resource=resource)

# OpenTelemetry Protocol Exporter (OTLP) Exporter
processor = BatchSpanProcessor(
    OTLPSpanExporter(
        # Change default HTTPS -> HTTP.
        insecure=True,
        # Update default OTLP reciver endpoint.
        endpoint=config.otlp_endpoint,
    )
)
provider.add_span_processor(processor)

# Sets the global default tracer provider.
trace.set_tracer_provider(provider)

# Creates a tracer from the global tracer provider.
tracer = trace.get_tracer("internal")
