package main

import (
	"time"

	"github.com/google/uuid"
)

type Image struct {
	ImageUUID string
	LastModified time.Time
}

func NewImage() *Image {
	id := uuid.New().String()
	lastModified := time.Now()

	return &Image{
		ImageUUID:    id,
		LastModified: lastModified,
	}
}

func download(key string) error {
	time.Sleep(time.Millisecond * 5)
	return nil
}

func save(c *Image) error {
	time.Sleep(time.Millisecond * 2)
	return nil
}
