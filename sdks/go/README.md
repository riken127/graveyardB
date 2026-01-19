# EventStore Go SDK

Go Client library for `graveyar_db` using gRPC.

## Features

- **Protocol**: gRPC with Protobuf.
- **Concurrency**: Optimistic Cocurrency Control via `ExpectedVersion`.
- **Schema**: Struct-tag based schema generation (`graveyard:"min=5"`).
- **Transport**: Configurable TLS and timeouts.

## Installation

```bash
go get github.com/riken127/graveyardB/sdks/go
```

## Usage

### Client Setup

```go
cfg := client.NewDefaultConfig()
cfg.Address = "localhost:50051"

c, err := client.NewClient(cfg)
if err != nil {
    log.Fatal(err)
}
defer c.Close()
```

### Schema Registration

```go
type User struct {
    Username string `graveyard:"min_len=3"`
    Age      int    `graveyard:"min=18"`
}

err := c.UpsertSchema("user", User{})
```

### Append & Read

```go
events := []*Event{...}
success, err := c.AppendEvent("stream-1", events, -1)

iter, err := c.GetEvents("stream-1")
for iter.Next() {
    log.Println(iter.Event())
}
```
