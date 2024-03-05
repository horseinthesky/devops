package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"slices"
	"time"

	"github.com/gofiber/contrib/otelfiber/v2"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/adaptor"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

const (
	defaultPort = "8000"
)

type handler struct {
	metrics *metrics
}

func (h *handler) getHealth(c *fiber.Ctx) error {
	return c.JSON(&fiber.Map{
		"status":  "ok",
		"message": "up",
	})
}

func (h *handler) getDevices(c *fiber.Ctx) error {
	return c.JSON(&fiber.Map{
		"status": "ok",
		"result": get_devices(),
	})
}

func (h *handler) getImage(c *fiber.Ctx) error {
	_, span := tracer.Start(c.UserContext(), "download")

	now := time.Now()
	err := download("thumbnail.png")

	// Record download duration.
	h.metrics.duration.With(prometheus.Labels{"operation": "s3"}).Observe(time.Since(now).Seconds())

	span.AddEvent("downladed")
	span.End()

	if err != nil {
		log.Printf("download failed: %v", err)
		return c.Status(http.StatusInternalServerError).JSON(&fiber.Map{
			"status":  "error",
			"message": "internal error",
		})
	}

	image := NewImage()
	_, span = tracer.Start(c.UserContext(), "save")
	defer span.End()

	now = time.Now()
	save(image)

	// Record save duration.
	h.metrics.duration.With(prometheus.Labels{"operation": "db"}).Observe(time.Since(now).Seconds())

	span.AddEvent("saved")

	return c.JSON(&fiber.Map{
		"status":  "ok",
		"message": "saved",
	})
}

func main() {
	// Setup telemetry
	ctx := context.Background()
	tp, err := setupTraceProvider(ctx)
	if err != nil {
		panic(err)
	}

	// Handle shutdown properly so nothing leaks.
	defer func() { _ = tp.Shutdown(ctx) }()

	// Initialize handler.
	h := handler{metrics: NewMetrics()}

	app := fiber.New()

	// OTLP middleware
	endpointsToSkip := []string{"/health", "/metrics"}
	app.Use(otelfiber.Middleware(otelfiber.WithNext(func(c *fiber.Ctx) bool {
		return slices.Index(endpointsToSkip, c.Path()) != -1
	})))

	// Routes
	app.Get("/api/devices", h.getDevices)
	app.Get("/api/images", h.getImage)
	app.Get("/health", h.getHealth)
	// Attach prometheus /metrics endpoint to Fiber router.
	// Use adaptor for http.HandlerFunc -> fiber.Handler
	app.Get("/metrics", adaptor.HTTPHandler(promhttp.Handler()))

	// Start HTTP server.
	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}
	app.Listen(fmt.Sprintf(":%v", port))
}
