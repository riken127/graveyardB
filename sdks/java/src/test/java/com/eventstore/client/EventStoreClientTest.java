package com.eventstore.client;

import com.eventstore.client.annotations.GraveyardEntity;
import com.eventstore.client.annotations.GraveyardField;
import com.eventstore.client.config.EventStoreConfig;
import com.eventstore.client.model.AppendEventRequest;
import com.eventstore.client.model.AppendEventResponse;
import com.eventstore.client.model.Event;
import com.eventstore.client.model.EventStoreGrpc;
import com.eventstore.client.model.GetEventsRequest;
import com.eventstore.client.model.UpsertSchemaResponse;
import com.google.common.util.concurrent.Futures;
import com.google.common.util.concurrent.ListenableFuture;
import io.grpc.ManagedChannel;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.ArgumentCaptor;
import org.mockito.Mock;
import org.mockito.MockedStatic;
import org.mockito.Mockito;
import org.mockito.junit.jupiter.MockitoExtension;

import java.util.Collections;
import java.util.Iterator;
import java.util.List;
import java.util.concurrent.TimeUnit;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.*;

@ExtendWith(MockitoExtension.class)
class EventStoreClientTest {

    @Mock
    private ManagedChannel managedChannel;

    @Mock
    private EventStoreConfig eventStoreConfig;

    @Mock
    private EventStoreGrpc.EventStoreBlockingStub blockingStub;

    @Mock
    private EventStoreGrpc.EventStoreFutureStub futureStub;

    private EventStoreClient eventStoreClient;

    @BeforeEach
    void setUp() {
        // Mock static factory methods for stubs
        try (MockedStatic<EventStoreGrpc> mockedStatic = Mockito.mockStatic(EventStoreGrpc.class)) {
            mockedStatic.when(() -> EventStoreGrpc.newBlockingStub(managedChannel)).thenReturn(blockingStub);
            mockedStatic.when(() -> EventStoreGrpc.newFutureStub(managedChannel)).thenReturn(futureStub);
            
            // Mock config
            when(eventStoreConfig.getTimeoutMs()).thenReturn(5000L);

            // Instantiate client (this triggers the static mocks)
            eventStoreClient = new EventStoreClient(managedChannel, eventStoreConfig);
        }
    }

    @Test
    void appendEvent_Success() {
        // Arrange
        String streamId = "test-stream";
        List<Event> events = Collections.singletonList(Event.newBuilder().setId("1").build());
        long expectedVersion = 10L;

        AppendEventResponse response = AppendEventResponse.newBuilder().setSuccess(true).build();
        
        // Mock withDeadlineAfter chain
        when(blockingStub.withDeadlineAfter(anyLong(), any(TimeUnit.class))).thenReturn(blockingStub);
        when(blockingStub.appendEvent(any(AppendEventRequest.class))).thenReturn(response);

        // Act
        boolean result = eventStoreClient.appendEvent(streamId, events, expectedVersion);

        // Assert
        assertTrue(result);
        
        // Verify arguments
        ArgumentCaptor<AppendEventRequest> captor = ArgumentCaptor.forClass(AppendEventRequest.class);
        verify(blockingStub).appendEvent(captor.capture());
        AppendEventRequest request = captor.getValue();
        
        assertEquals(streamId, request.getStreamId());
        assertEquals(expectedVersion, request.getExpectedVersion());
        assertEquals(1, request.getEventsCount());
        
        // Verify timeout
        verify(blockingStub).withDeadlineAfter(5000L, TimeUnit.MILLISECONDS);
    }

    @Test
    void appendEvent_Failure() {
        // Arrange
        AppendEventResponse response = AppendEventResponse.newBuilder().setSuccess(false).build();
        when(blockingStub.withDeadlineAfter(anyLong(), any(TimeUnit.class))).thenReturn(blockingStub);
        when(blockingStub.appendEvent(any(AppendEventRequest.class))).thenReturn(response);

        // Act
        boolean result = eventStoreClient.appendEvent("stream", Collections.emptyList());

        // Assert
        assertFalse(result);
    }

    @Test
    void getEvents_Success() {
        // Arrange
        String streamId = "read-stream";
        Iterator<Event> mockIterator = mock(Iterator.class);
        
        when(blockingStub.withDeadlineAfter(anyLong(), any(TimeUnit.class))).thenReturn(blockingStub);
        when(blockingStub.getEvents(any(GetEventsRequest.class))).thenReturn(mockIterator);

        // Act
        Iterator<Event> result = eventStoreClient.getEvents(streamId);

        // Assert
        assertNotNull(result);
        assertSame(mockIterator, result);
        
        verify(blockingStub).withDeadlineAfter(5000L, TimeUnit.MILLISECONDS);
    }

    @Test
    void appendEventAsync_Success() {
        // Arrange
        String streamId = "async-stream";
        List<Event> events = Collections.emptyList();
        AppendEventResponse response = AppendEventResponse.newBuilder().setSuccess(true).build();
        ListenableFuture<AppendEventResponse> futureResponse = Futures.immediateFuture(response);

        when(futureStub.withDeadlineAfter(anyLong(), any(TimeUnit.class))).thenReturn(futureStub);
        when(futureStub.appendEvent(any(AppendEventRequest.class))).thenReturn(futureResponse);

        // Act
        ListenableFuture<AppendEventResponse> result = eventStoreClient.appendEventAsync(streamId, events, -1);

        // Assert
        assertNotNull(result);
        verify(futureStub).withDeadlineAfter(5000L, TimeUnit.MILLISECONDS);
    }

    @Test
    void upsertSchema_Success() {
        // Arrange
        UpsertSchemaResponse response = UpsertSchemaResponse.newBuilder().setSuccess(true).build();
        when(blockingStub.withDeadlineAfter(anyLong(), any(TimeUnit.class))).thenReturn(blockingStub);
        when(blockingStub.upsertSchema(any(com.eventstore.client.model.UpsertSchemaRequest.class))).thenReturn(response);

        // Act
        UpsertSchemaResponse result = eventStoreClient.upsertSchema(TestEntity.class);

        // Assert
        assertTrue(result.getSuccess());
        
        ArgumentCaptor<com.eventstore.client.model.UpsertSchemaRequest> captor = ArgumentCaptor.forClass(com.eventstore.client.model.UpsertSchemaRequest.class);
        verify(blockingStub).upsertSchema(captor.capture());
        
        com.eventstore.client.model.Schema schema = captor.getValue().getSchema();
        assertEquals("test_entity", schema.getName());
        assertTrue(schema.getFieldsMap().containsKey("name"));
        assertTrue(schema.getFieldsMap().containsKey("age"));
    }

    @Test
    void upsertSchema_WithConstraints() {
        // Arrange
        UpsertSchemaResponse response = UpsertSchemaResponse.newBuilder().setSuccess(true).build();
        when(blockingStub.withDeadlineAfter(anyLong(), any(TimeUnit.class))).thenReturn(blockingStub);
        when(blockingStub.upsertSchema(any(com.eventstore.client.model.UpsertSchemaRequest.class))).thenReturn(response);

        // Act
        UpsertSchemaResponse result = eventStoreClient.upsertSchema(ConstrainedEntity.class);

        // Assert
        assertTrue(result.getSuccess());
        
        ArgumentCaptor<com.eventstore.client.model.UpsertSchemaRequest> captor = ArgumentCaptor.forClass(com.eventstore.client.model.UpsertSchemaRequest.class);
        verify(blockingStub).upsertSchema(captor.capture());
        
        com.eventstore.client.model.Schema schema = captor.getValue().getSchema();
        assertTrue(schema.getFieldsMap().containsKey("age"));
        
        com.eventstore.client.model.Field ageField = schema.getFieldsMap().get("age");
        assertTrue(ageField.hasConstraints());
        assertEquals(0.0, ageField.getConstraints().getMinValue());
        assertEquals(150.0, ageField.getConstraints().getMaxValue());
        
        com.eventstore.client.model.Field usernameField = schema.getFieldsMap().get("username");
        assertTrue(usernameField.hasConstraints());
        assertEquals(3, usernameField.getConstraints().getMinLength());
        assertEquals("^[a-z]+$", usernameField.getConstraints().getRegex());
    }

    @GraveyardEntity("test_entity")
    static class TestEntity {
        @GraveyardField(nullable = false)
        String name;
        
        int age;
    }

    @GraveyardEntity("constrained_entity")
    static class ConstrainedEntity {
        @GraveyardField(min = 0, max = 150)
        int age;
        
        @GraveyardField(minLength = 3, regex = "^[a-z]+$")
        String username;
    }
}
