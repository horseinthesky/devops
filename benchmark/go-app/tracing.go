package main

import (
	"context"
	"os"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracegrpc"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.21.0"
	"go.opentelemetry.io/otel/trace"
)

const (
	defaultOtlpEndpoint = "localhost:4317"
)

var (
	tracer trace.Tracer
)

func setupTraceProvider(ctx context.Context) (*sdktrace.TracerProvider, error) {
	otlpEndpoint := os.Getenv("OTLP_ENDPOINT")
	if otlpEndpoint == "" {
		otlpEndpoint = defaultOtlpEndpoint
	}

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
