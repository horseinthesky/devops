package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"slices"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"go.opentelemetry.io/contrib/instrumentation/github.com/gin-gonic/gin/otelgin"
)

func init() {
	gin.SetMode(gin.ReleaseMode)
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
