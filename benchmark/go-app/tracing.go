package main

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracegrpc"
	"go.opentelemetry.io/otel/sdk/resource"
	oteltrace "go.opentelemetry.io/otel/sdk/trace"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.21.0"
	"go.opentelemetry.io/otel/trace"
)

var (
	tracer trace.Tracer
)

func newOTLPExporter(ctx context.Context, otlpEndpoint string) (oteltrace.SpanExporter, error) {
	// Change default HTTPS -> HTTP.
	insecureOpt := otlptracegrpc.WithInsecure()

	// Update default OTLP reciver endpoint.
	endpointOpt := otlptracegrpc.WithEndpoint(otlpEndpoint)

	return otlptracegrpc.New(ctx, insecureOpt, endpointOpt)
}

func setupTraceProvider(ctx context.Context, otlpEndpoint string) (*sdktrace.TracerProvider, error) {
	insecureOpt := otlptracegrpc.WithInsecure()
	endpointOpt := otlptracegrpc.WithEndpoint(otlpEndpoint)

	exporter, err := otlptracegrpc.New(ctx, insecureOpt, endpointOpt)
	if err != nil {
		return nil, err
	}

	r := resource.NewWithAttributes(
		semconv.SchemaURL,
		semconv.ServiceNameKey.String("go-app"),
	)

	tp := sdktrace.NewTracerProvider(sdktrace.WithBatcher(exporter), sdktrace.WithResource(r))
	otel.SetTracerProvider(tp)

	tracer = tp.Tracer("internal")

	return tp, nil
}
