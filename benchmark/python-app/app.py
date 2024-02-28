from fastapi import FastAPI
from fastapi.responses import JSONResponse
from opentelemetry.instrumentation.fastapi import FastAPIInstrumentor
from prometheus_client import Summary
from prometheus_fastapi_instrumentator import Instrumentator as PromInstrumentator

from devices import devices
from images import Image, download, save
from tracing import tracer

app = FastAPI()
FastAPIInstrumentor.instrument_app(app, excluded_urls="/health,/metrics")
PromInstrumentator().instrument(app).expose(app)

request_duration = Summary(
    name="request_duration_seconds",
    namespace="myapp",
    documentation="Duration of the request",
    labelnames=["operation"],
)


@app.get("/api/devices")
def get_devices() -> JSONResponse:
    return JSONResponse(
        {
            "status": "ok",
            "result": devices,
        }
    )


@app.get("/api/images")
async def get_image() -> JSONResponse:
    # We get trace id and root span id from instrumentator
    with request_duration.labels("s3").time(), tracer.start_as_current_span(
        "DOWNLOAD IMAGE"
    ) as span:
        await download()
        span.add_event("downloaded")

    image = Image()

    with request_duration.labels("db").time(), tracer.start_as_current_span(
        "SAVE IMAGE"
    ) as span:
        await save(image)
        span.add_event("saved")

    return JSONResponse(
        {
            "status": "ok",
            "result": "saved",
        }
    )


@app.get("/health")
def get_health():
    return JSONResponse(
        {
            "status": "ok",
            "result": "up",
        }
    )
