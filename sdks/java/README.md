# EventStore Java SDK

This is a production-ready Java Client library for `graveyar_db`. It provides a simple, strongly-typed interface to interact with the EventStore gRPC API, with built-in support for Spring Boot, Timeouts, Async operations, and Optimistic Concurrency Control.

## Features

- **Protocol**: gRPC with Protobuf.
- **Concurrency**: Optimistic locking via `expected_version`.
- **Resilience**: Configurable Timeouts.
- **Performance**: Async API (`ListenableFuture`) and Sync API.
- **Integration**: Spring Boot `@Service` and `@Configuration`.
- **Environment**: Easy toggle between Plaintext (Dev) and TLS (Prod).

## Installation

Add the following to your `pom.xml` (assuming local install):

```xml
<dependency>
    <groupId>com.eventstore</groupId>
    <artifactId>eventstore-client</artifactId>
    <version>0.0.1-SNAPSHOT</version>
</dependency>
```

## Configuration

Configure the client in your `application.properties` or `application.yml`:

| Property | Default | Description |
|----------|---------|-------------|
| `eventstore.host` | `localhost` | Hostname of the EventStore server. |
| `eventstore.port` | `50051` | gRPC port. |
| `eventstore.use-tls` | `false` | Set `true` for Production to check certificates. |
| `eventstore.timeout-ms` | `5000` | Timeout for requests in milliseconds. |

## Usage

Inject the client into your service:

```java
@Autowired
private EventStoreClient client;
```

### Append Sync

```java
List<Event> events = List.of(Event.newBuilder()...build());
// -1 for no version check
boolean success = client.appendEvent("stream-1", events, -1);
```

### Append Async

```java
ListenableFuture<AppendEventResponse> future = client.appendEventAsync("stream-1", events, 10);
Futures.addCallback(future, new FutureCallback<>() {
    public void onSuccess(AppendEventResponse r) { ... }
    public void onFailure(Throwable t) { ... }
}, executor);
```

### Read Stream

```java
Iterator<Event> events = client.getEvents("stream-1");
while (events.hasNext()) {
    Event e = events.next();
    System.out.println(e.getPayload().toStringUtf8());
}
```

## Development

Build and run tests:

```bash
mvn clean install
```
