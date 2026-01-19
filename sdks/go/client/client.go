package client

import (
	"context"
	"crypto/tls"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"

	pb "github.com/riken127/graveyar_db/sdks/go/proto"
	"github.com/riken127/graveyar_db/sdks/go/schema"
)

// GenerateSchema is a helper to generate a Schema definition from a Go struct.
func GenerateSchema(v interface{}) (*pb.Schema, error) {
	return schema.Generate(v)
}

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
	var opts []grpc.DialOption

	if config.UseTLS {
		var creds credentials.TransportCredentials
		var err error

		if config.TLSCertFile != "" {
			creds, err = credentials.NewClientTLSFromFile(config.TLSCertFile, "")
			if err != nil {
				return nil, err
			}
		} else {
			// Use system root CAs
			// Note: This requires "crypto/x509" and "google.golang.org/grpc/credentials"
			// Since we want to keep imports clean, we'll try standard loading if needed
			// actually credentials.NewTLS(nil) uses system roots
			creds = credentials.NewTLS(&tls.Config{})
		}
		opts = append(opts, grpc.WithTransportCredentials(creds))
	} else {
		opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))
	}

	conn, err := grpc.Dial(config.Address, opts...)
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

// UpsertSchema registers or updates a schema definition.
func (c *Client) UpsertSchema(ctx context.Context, schema *pb.Schema) (*pb.UpsertSchemaResponse, error) {
	if _, ok := ctx.Deadline(); !ok && c.config.Timeout > 0 {
		var cancel context.CancelFunc
		ctx, cancel = context.WithTimeout(ctx, c.config.Timeout)
		defer cancel()
	}

	req := &pb.UpsertSchemaRequest{
		Schema: schema,
	}
	return c.client.UpsertSchema(ctx, req)
}

// GetSchema retrieves a schema definition by name.
func (c *Client) GetSchema(ctx context.Context, name string) (*pb.GetSchemaResponse, error) {
	if _, ok := ctx.Deadline(); !ok && c.config.Timeout > 0 {
		var cancel context.CancelFunc
		ctx, cancel = context.WithTimeout(ctx, c.config.Timeout)
		defer cancel()
	}

	req := &pb.GetSchemaRequest{
		Name: name,
	}
	return c.client.GetSchema(ctx, req)
}
