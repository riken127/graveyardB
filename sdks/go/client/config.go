package client

import "time"

// Config holds the configuration for the EventStore client.
type Config struct {
	// Address is the target address of the EventStore server (e.g., "localhost:50051").
	Address string

	// Timeout is the default timeout for gRPC calls.
	// If zero, no timeout is applied by default (though context deadline still applies).
	Timeout time.Duration
}

// DefaultConfig returns a default configuration with:
// Address: "localhost:50051"
// Timeout: 5 seconds
func DefaultConfig() Config {
	return Config{
		Address: "localhost:50051",
		Timeout: 5 * time.Second,
	}
}
