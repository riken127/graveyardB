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

	// -1 indicates "any version" (or handled as such by logic)
	success, err := c.AppendEvent(ctx, "test-stream", events, -1)
	if err != nil {
		log.Printf("Append failed: %v", err)
	} else {
		fmt.Printf("Append success: %v\n", success)
	}
}
