package main

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

type metrics struct {
	duration *prometheus.SummaryVec
}

func NewMetrics() *metrics {
	return &metrics{
		duration: promauto.NewSummaryVec(prometheus.SummaryOpts{
			Namespace:  "myapp",
			Name:       "request_duration_seconds",
			Help:       "Duration of the request.",
			Objectives: map[float64]float64{0.9: 0.01, 0.99: 0.001},
		}, []string{"operation"}),
	}
}
