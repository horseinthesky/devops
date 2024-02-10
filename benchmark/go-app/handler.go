package main

import (
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
)

type handler struct {
	metrics *metrics
	config  *Config
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

func (h *handler) getHealth(c *gin.Context) {
	c.JSON(200, gin.H{"status": "ok", "message": "up"})
}
