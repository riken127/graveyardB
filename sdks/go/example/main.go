package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/riken127/graveyar_db/sdks/go/client"
	pb "github.com/riken127/graveyar_db/sdks/go/proto"
)

func main() {
	cfg := client.DefaultConfig()
	cfg.Address = "localhost:50051"

	c, err := client.NewClient(cfg)
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}
	defer c.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()

	// Append
	events := []*pb.Event{
		{
			Id:        "123",
			EventType: "TestEvent",
			Payload:   []byte("Hello Go SDK"),
			Timestamp: uint64(time.Now().Unix()),
		},
	}

	success, err := c.AppendEvent(ctx, "test-stream", events, -1)
	if err != nil {
		log.Printf("Append failed: %v", err)
	} else {
		fmt.Printf("Append success: %v\n", success)
	}

	// Schema Example
	type User struct {
		Name   string `json:"full_name"`
		Age    int    `graveyard:"min=18"`
		Active bool
	}

	userSchema, err := client.GenerateSchema(User{})
	if err != nil {
		log.Fatalf("Schema generation failed: %v", err)
	}

	upsertResp, err := c.UpsertSchema(ctx, userSchema)
	if err != nil {
		log.Printf("UpsertSchema failed: %v", err)
	} else {
		fmt.Printf("UpsertSchema success: %v\n", upsertResp.Success)
	}
}
