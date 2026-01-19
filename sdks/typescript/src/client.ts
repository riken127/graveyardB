import * as grpc from '@grpc/grpc-js';
import { EventStoreClient as GrpcClient } from './proto/eventstore';
import { EventStoreConfig, defaultConfig } from './config';
import { Event, AppendEventRequest, GetEventsRequest, UpsertSchemaRequest, UpsertSchemaResponse, AppendEventResponse } from './proto/eventstore';
import { SchemaGenerator } from './schema/generator';

export class EventStoreClient {
    private client: GrpcClient;
    private config: EventStoreConfig;

    constructor(config: Partial<EventStoreConfig> = {}) {
        this.config = { ...defaultConfig, ...config };

        const address = `${this.config.host}:${this.config.port}`;
        const credentials = this.config.useTls
            ? grpc.credentials.createSsl()
            : grpc.credentials.createInsecure();

        this.client = new GrpcClient(address, credentials);
    }

    private getDeadline(): Date {
        return new Date(Date.now() + this.config.timeoutMs);
    }

    async appendEvent(streamId: string, events: Event[], expectedVersion: number = -1): Promise<boolean> {
        const req: AppendEventRequest = {
            streamId,
            events,
            expectedVersion
        };

        return new Promise((resolve, reject) => {
            const metadata = new grpc.Metadata();
            this.client.appendEvent(req, metadata, { deadline: this.getDeadline() }, (err, response) => {
                if (err) return reject(err);
                if (!response) return reject(new Error('No response received'));
                resolve(response.success);
            });
        });
    }

    async getEvents(streamId: string): Promise<Event[]> {
        const req: GetEventsRequest = { streamId };
        const events: Event[] = [];

        return new Promise((resolve, reject) => {
            const stream = this.client.getEvents(req, { deadline: this.getDeadline() });

            stream.on('data', (event: Event) => {
                events.push(event);
            });

            stream.on('end', () => {
                resolve(events);
            });

            stream.on('error', (err) => {
                reject(err);
            });
        });
    }

    async upsertSchema(entityClass: Function): Promise<UpsertSchemaResponse> {
        const schema = SchemaGenerator.generate(entityClass);
        const req: UpsertSchemaRequest = { schema };

        return new Promise((resolve, reject) => {
            const metadata = new grpc.Metadata();
            this.client.upsertSchema(req, metadata, { deadline: this.getDeadline() }, (err, response) => {
                if (err) return reject(err);
                if (!response) return reject(new Error('No response received'));
                resolve(response);
            });
        });
    }

    close() {
        this.client.close();
    }
}
