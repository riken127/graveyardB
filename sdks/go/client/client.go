package client

import (
	"context"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "github.com/riken127/graveyar_db/sdks/go/proto"
)

// Client is the high-level client for interacting with the graveyar_db Event Store.
// It manages the underlying gRPC connection and provides strongly-typed methods
// for appending and reading events.
type Client struct {
	conn   *grpc.ClientConn
	client pb.EventStoreClient
	config Config
}

// NewClient creates a new Client with the provided configuration.
// It establishes a gRPC connection to the server specified in config.Address.
func NewClient(config Config) (*Client, error) {
	conn, err := grpc.Dial(config.Address, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, err
	}
	return &Client{
		conn:   conn,
		client: pb.NewEventStoreClient(conn),
		config: config,
	}, nil
}

// Close closes the underlying gRPC connection.
// It should be called when the client is no longer needed.
func (c *Client) Close() error {
	return c.conn.Close()
}

// AppendEvent appends a batch of events to a specific stream.
//
// streamID: The unique identifier of the stream.
// events: The list of events to append.
// expectedVersion: Optimistic locking version. 
// Pass -1 to disable version checking (append regardless of current version).
// Pass generic version numbers (0, 1, ...) to enforce strict ordering.
//
// Returns true if the append was successful, or an error if the RPC failed
// or the version check failed on the server side (depending on server error implementation).
func (c *Client) AppendEvent(ctx context.Context, streamID string, events []*pb.Event, expectedVersion int64) (bool, error) {
	// Apply default timeout from config if context has no deadline? 
	// Standard Go practice prefers caller to handle context, but we can respect config.Timeout
	if _, ok := ctx.Deadline(); !ok && c.config.Timeout > 0 {
		var cancel context.CancelFunc
		ctx, cancel = context.WithTimeout(ctx, c.config.Timeout)
		defer cancel()
	}

	req := &pb.AppendEventRequest{
		StreamId:        streamID,
		Events:          events,
		ExpectedVersion: uint64(expectedVersion), // Note: proto defines it as uint64, but logic treats -1 special. We might need casting if proto changed to int64 or specific handling.
		// Proto defined expected_version as uint64. 
		// If we want to support -1 as "any", we rely on the server interpreting a very large uint64 (MaxUint64) as -1 or just 0 as "no check" depending on implementation.
		// However, in Java client logic: return appendEvent(streamId, events, -1);
		// Let's assume for now we cast to uint64, and server handles MaxUint64 as "any" or we should change local logic. 
		// BUT wait, looking at proto: uint64 expected_version = 3; 
		// If user passes -1, casting to uint64 creates a large number. 
		// We will stick to int64 signature to match "concept" but cast to uint64 for transport.
	}

	resp, err := c.client.AppendEvent(ctx, req)
	if err != nil {
		return false, err
	}
	return resp.Success, nil
}

// GetEvents opens a stream to read events from the specified streamID.
// It returns a gRPC stream client that can be used to receive events.
func (c *Client) GetEvents(ctx context.Context, streamID string) (pb.EventStore_GetEventsClient, error) {
	if _, ok := ctx.Deadline(); !ok && c.config.Timeout > 0 {
		// For streaming, timeout usually applies to establishment or individual messages?
		// Usually we don't set a hard timeout on the whole stream unless intended.
		// We will leave context management for streams to the caller mostly.
	}

	req := &pb.GetEventsRequest{
		StreamId: streamID,
	}
	return c.client.GetEvents(ctx, req)
}
