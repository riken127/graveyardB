# EventStore TypeScript SDK

Production-ready TypeScript Client for `graveyar_db`. Adheres to the standardization spec with high-performance gRPC, strict Schema Constraints, and Async/Await support.

## Features

- **Protocol**: gRPC over HTTP/2 (via `@grpc/grpc-js`).
- **ODM**: TypeScript Decorators (`@GraveyardEntity`, `@GraveyardField`) for robust schema definitions.
- **Data Integrity**: Enforced via Schema Constraints (Min/Max, Regex) mapped to Proto.
- **Reliability**: Configurable Timeouts and TLS support.

## Installation

```bash
npm install @eventstore/client
```

## Configuration

```typescript
import { EventStoreClient } from '@eventstore/client';

const client = new EventStoreClient({
    host: 'localhost',
    port: 50051,
    useTls: process.env.NODE_ENV === 'production',
    timeoutMs: 2000
});
```

## Usage

### Define Entities

```typescript
import { GraveyardEntity, GraveyardField } from '@eventstore/client/decorators';

@GraveyardEntity("user_profile")
class UserProfile {
    @GraveyardField({ minLength: 3, regex: "^[a-z]+$" })
    username: string;

    @GraveyardField({ min: 18, max: 150 })
    age: number;
}
```

### Register Schema

```typescript
await client.upsertSchema(UserProfile);
```

### Append Events

```typescript
const result = await client.appendEvent("stream-1", [
    { id: '1', eventType: 'Created', payload: Buffer.from('...'), timestamp: Date.now() }
]);
```

## Development

```bash
npm install
npm run proto:gen
npm test
```
