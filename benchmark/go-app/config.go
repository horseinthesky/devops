package main

import (
	"log"
	"os"

	"gopkg.in/yaml.v2"
)

// Config represents configuration for the app.
type Config struct {
	// Port to run the http server.
	Port int `yaml:"port"`

	// OTLP Endpoint to send traces.
	OTLPEndpoint string `yaml:"otlp_endpoint"`
}

// loadConfig loads app config from YAML file.
func (c *Config) loadConfig(path string) {

	// Read the config file from the disk.
	f, err := os.ReadFile(path)
	if err != nil {
		log.Fatalf("os.ReadFile failed: %v", err)
	}

	// Convert the YAML config into a Go struct.
	err = yaml.Unmarshal(f, c)
	if err != nil {
		log.Fatalf("yaml.Unmarshal failed: %v", err)
	}
}
