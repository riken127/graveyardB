package com.eventstore.client;

import com.eventstore.client.model.AppendEventRequest;
import com.eventstore.client.model.AppendEventResponse;
import com.eventstore.client.model.Event;
import com.eventstore.client.model.EventStoreGrpc;
import com.eventstore.client.model.GetEventsRequest;
import com.eventstore.client.model.UpsertSchemaRequest;
import com.eventstore.client.model.UpsertSchemaResponse;
import com.eventstore.client.schema.SchemaGenerator;
import com.eventstore.client.annotations.GraveyardEntity;
import com.eventstore.client.config.EventStoreConfig;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import org.springframework.stereotype.Service;

import java.util.Iterator;
import java.util.List;
import java.util.concurrent.TimeUnit;

/**
 * Client for interacting with the EventStore gRPC API.
 * Provides synchronous and asynchronous methods to append and read events.
 */
@Service
public class EventStoreClient {

    private final EventStoreGrpc.EventStoreBlockingStub blockingStub;
    private final EventStoreGrpc.EventStoreFutureStub futureStub;
    private final EventStoreConfig config;

    public EventStoreClient(ManagedChannel channel, EventStoreConfig config) {
        this.config = config;
        this.blockingStub = EventStoreGrpc.newBlockingStub(channel);
        this.futureStub = EventStoreGrpc.newFutureStub(channel);
    }

    /**
     * Appends events to a stream without optimistic concurrency control.
     * Equivalent to {@code appendEvent(streamId, events, -1)}.
     *
     * @param streamId The ID of the stream to append to.
     * @param events   The list of events to append.
     * @return true if successful, false otherwise.
     */
    public boolean appendEvent(String streamId, List<Event> events) {
        return appendEvent(streamId, events, -1);
    }

    /**
     * Appends events to a stream with optimistic concurrency control.
     *
     * @param streamId        The ID of the stream.
     * @param events          The events to append.
     * @param expectedVersion The expected version of the stream. Use -1 to disable check.
     * @return true if successful.
     */
    public boolean appendEvent(String streamId, List<Event> events, long expectedVersion) {
        AppendEventRequest request = AppendEventRequest.newBuilder()
                .setStreamId(streamId)
                .addAllEvents(events)
                .setExpectedVersion(expectedVersion)
                .build();
        
        AppendEventResponse response = blockingStub
                .withDeadlineAfter(config.getTimeoutMs(), TimeUnit.MILLISECONDS)
                .appendEvent(request);
        return response.getSuccess();
    }

    /**
     * Asynchronously appends events to a stream.
     *
     * @param streamId        The ID of the stream.
     * @param events          The events to append.
     * @param expectedVersion The expected version (-1 for any).
     * @return A Future containing the response.
     */
    public com.google.common.util.concurrent.ListenableFuture<AppendEventResponse> appendEventAsync(String streamId, List<Event> events, long expectedVersion) {
        AppendEventRequest request = AppendEventRequest.newBuilder()
                .setStreamId(streamId)
                .addAllEvents(events)
                .setExpectedVersion(expectedVersion)
                .build();
        
        return futureStub
                .withDeadlineAfter(config.getTimeoutMs(), TimeUnit.MILLISECONDS)
                .appendEvent(request);
    }

    /**
     * Reads events from a stream.
     *
     * @param streamId The stream ID to read from.
     * @return An iterator of events.
     */
    public Iterator<Event> getEvents(String streamId) {
        GetEventsRequest request = GetEventsRequest.newBuilder()
                .setStreamId(streamId)
                .build();
        
        return blockingStub
                .withDeadlineAfter(config.getTimeoutMs(), TimeUnit.MILLISECONDS)
                .getEvents(request);
    }

    /**
     * registers or updates the schema for the given entity class.
     * The class must be annotated with @GraveyardEntity.
     *
     * @param entityClass The class of the entity to register.
     * @return The response from the server.
     */
    public UpsertSchemaResponse upsertSchema(Class<?> entityClass) {
        if (!entityClass.isAnnotationPresent(GraveyardEntity.class)) {
            throw new IllegalArgumentException("Class " + entityClass.getName() + " is not annotated with @GraveyardEntity");
        }

        com.eventstore.client.model.Schema schema = SchemaGenerator.generate(entityClass);
        
        UpsertSchemaRequest request = UpsertSchemaRequest.newBuilder()
                .setSchema(schema)
                .build();

        return blockingStub
                .withDeadlineAfter(config.getTimeoutMs(), TimeUnit.MILLISECONDS)
                .upsertSchema(request);
    }
}
