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
import org.springframework.stereotype.Service;

import java.util.Iterator;
import java.util.List;
import java.util.concurrent.TimeUnit;

/**
 * Client for interacting with the Graveyar_DB EventStore gRPC API.
 * <p>
 * This client provides synchronous and asynchronous methods to:
 * <ul>
 *     <li>Append events to streams with Optimistic Concurrency Control (OCC).</li>
 *     <li>Read events from streams.</li>
 *     <li>Manage schemas for event validation.</li>
 *     <li>Store and retrieve snapshots for stream state.</li>
 * </ul>
 */
@Service
public class EventStoreClient {

    private final EventStoreGrpc.EventStoreBlockingStub blockingStub;
    private final EventStoreGrpc.EventStoreFutureStub futureStub;
    private final EventStoreConfig config;

    /**
     * Creates a new client instance.
     *
     * @param channel The gRPC managed channel to the Graveyar_DB server.
     * @param config  Configuration for timeouts and connection settings.
     */
    public EventStoreClient(ManagedChannel channel, EventStoreConfig config) {
        this.config = config;
        this.blockingStub = EventStoreGrpc.newBlockingStub(channel);
        this.futureStub = EventStoreGrpc.newFutureStub(channel);
    }

    /**
     * Appends a batch of events to a stream without requesting optimistic concurrency checks.
     * This is equivalent to calling {@link #appendEvent(String, List, long)} with {@code expectedVersion = -1}.
     *
     * @param streamId The distinct ID of the stream (e.g., "order-123").
     * @param events   The list of {@link Event} objects to append.
     * @return {@code true} if the events were successfully appended; {@code false} otherwise.
     */
    public boolean appendEvent(String streamId, List<Event> events) {
        return appendEvent(streamId, events, -1);
    }

    /**
     * Appends a batch of events to a stream, enforcing Optimistic Concurrency Control (OCC).
     *
     * @param streamId        The distinct ID of the stream.
     * @param events          The list of {@link Event} objects to append.
     * @param expectedVersion The expected version of the stream prior to this append.
     *                        Use {@code -1} (or {@code 0} depending on server impl) to disable the check.
     *                        If the server's current version does not match, the append fails.
     * @return {@code true} if successful; {@code false} if a concurrency conflict or other error occurred.
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
     * @param streamId        The stream ID.
     * @param events          The events to append.
     * @param expectedVersion The expected version for OCC.
     * @return A {@link com.google.common.util.concurrent.ListenableFuture} representing the pending response.
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
     * <p>
     * Returns an iterator that streams events from the server.
     *
     * @param streamId The stream ID to read from.
     * @return An {@link Iterator} of {@link Event}s.
     * @throws io.grpc.StatusRuntimeException If the stream is not found or communication fails.
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
     * Registers or updates a schema for the domain entity.
     * <p>
     * The provided class must be annotated with {@link GraveyardEntity}.
     * The schema is generated by reflecting on fields annotated with {@link com.eventstore.client.annotations.GraveyardField}.
     *
     * @param entityClass The Java class representing the entity.
     * @return The response from the server indicating success or failure.
     * @throws IllegalArgumentException If the class is missing the {@code @GraveyardEntity} annotation.
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

    /**
     * Saves a snapshot of a stream's state at a specific version.
     *
     * @param streamId  The stream ID.
     * @param version   The version of the stream this snapshot corresponds to.
     * @param payload   The serialized state payload (e.g., JSON bytes).
     * @param timestamp The timestamp of the snapshot.
     * @return {@code true} if the snapshot was successfully saved.
     */
    public boolean saveSnapshot(String streamId, long version, byte[] payload, long timestamp) {
        com.eventstore.client.model.Snapshot snapshot = com.eventstore.client.model.Snapshot.newBuilder()
                .setStreamId(streamId)
                .setVersion(version)
                .setPayload(com.google.protobuf.ByteString.copyFrom(payload))
                .setTimestamp(timestamp)
                .build();

        com.eventstore.client.model.SaveSnapshotRequest request = com.eventstore.client.model.SaveSnapshotRequest.newBuilder()
                .setSnapshot(snapshot)
                .build();

        com.eventstore.client.model.SaveSnapshotResponse response = blockingStub
                .withDeadlineAfter(config.getTimeoutMs(), TimeUnit.MILLISECONDS)
                .saveSnapshot(request);
        
        return response.getSuccess();
    }

    /**
     * Retrieves the latest snapshot for a stream.
     *
     * @param streamId The stream ID.
     * @return The {@link com.eventstore.client.model.Snapshot} if found, or {@code null} if no snapshot exists.
     */
    public com.eventstore.client.model.Snapshot getSnapshot(String streamId) {
        com.eventstore.client.model.GetSnapshotRequest request = com.eventstore.client.model.GetSnapshotRequest.newBuilder()
                .setStreamId(streamId)
                .build();

        com.eventstore.client.model.GetSnapshotResponse response = blockingStub
                .withDeadlineAfter(config.getTimeoutMs(), TimeUnit.MILLISECONDS)
                .getSnapshot(request);
        
        if (response.getFound()) {
            return response.getSnapshot();
        } else {
            return null;
        }
    }
}
