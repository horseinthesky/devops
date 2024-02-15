package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"slices"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"go.opentelemetry.io/contrib/instrumentation/github.com/gin-gonic/gin/otelgin"
)

func init() {
	gin.SetMode(gin.ReleaseMode)
}

type handler struct {
	metrics *metrics
	config  *Config
}

func (h *handler) getHealth(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{"status": "ok", "message": "up"})
}

func (h *handler) getDevices(c *gin.Context) {
	c.JSON(http.StatusOK, get_devices())
}

func (h *handler) getImage(c *gin.Context) {
	_, span := tracer.Start(c.Request.Context(), "DOWNLOAD IMAGE")

	now := time.Now()
	err := download("thumbnail.png")
	if err != nil {
		log.Printf("download failed: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "internal error"})
		return
	}

	// Record download duration.
	h.metrics.duration.With(prometheus.Labels{"operation": "download"}).Observe(time.Since(now).Seconds())

	span.AddEvent("downladed")
	span.End()

	image := NewImage()
	_, span = tracer.Start(c.Request.Context(), "SAVE IMAGE")
	defer span.End()

	now = time.Now()
	save(image)

	// Record save duration.
	h.metrics.duration.With(prometheus.Labels{"operation": "save"}).Observe(time.Since(now).Seconds())

	span.AddEvent("saved")

	c.JSON(http.StatusOK, gin.H{"status": "ok", "message": "saved"})
}

func main() {
	var c Config
	c.loadConfig("config.yaml")

	// Setup telemetry
	ctx := context.Background()
	tp, err := setupTraceProvider(ctx, c.OTLPEndpoint)
	if err != nil {
		panic(err)
	}

	// Handle shutdown properly so nothing leaks.
	defer func() { _ = tp.Shutdown(ctx) }()

	// Initialize Gin handler.
	h := handler{config: &c, metrics: NewMetrics()}

	r := gin.New()
	r.Use(otelgin.Middleware("go-app", otelgin.WithFilter(func(req *http.Request) bool {
		notToLogEndpoints := []string{"/health", "/metrics"}
		return slices.Index(notToLogEndpoints, req.URL.Path) == -1
	})))

	// Define handler functions for each endpoint.
	r.GET("/api/devices", h.getDevices)
	r.GET("/api/images", h.getImage)
	r.GET("/health", h.getHealth)
	// Attach prometheus /metrics endpoint to Gin router.
	r.GET("/metrics", func(c *gin.Context) {
		promhttp.Handler().ServeHTTP(c.Writer, c.Request)
	})

	// Start the main Gin HTTP server.
	log.Printf("Starting App on port %d", c.Port)
	r.Run(fmt.Sprintf(":%d", c.Port))
}
