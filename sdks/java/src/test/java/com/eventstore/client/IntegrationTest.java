package com.eventstore.client;

import com.eventstore.client.annotations.GraveyardEntity;
import com.eventstore.client.annotations.GraveyardField;
import com.eventstore.client.config.EventStoreConfig;
import com.eventstore.client.model.UpsertSchemaResponse;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Disabled;

import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.assertEquals;

// Enable this test manually or via profile, normally getting ignored in unit test phase if strict
public class IntegrationTest {

    private static ManagedChannel channel;
    private static EventStoreClient client;

    @BeforeAll
    static void setUp() {
        // Assume backend is running on localhost:50051
        channel = ManagedChannelBuilder.forAddress("localhost", 50051)
                .usePlaintext()
                .build();
        
        EventStoreConfig config = new EventStoreConfig();
        config.setTimeoutMs(5000L);
        
        client = new EventStoreClient(channel, config);
    }

    @AfterAll
    static void tearDown() {
        if (channel != null) {
            channel.shutdown();
        }
    }

    @Test
    void testUpsertSchema() {
        UpsertSchemaResponse response = client.upsertSchema(IntegrationUser.class);
        assertTrue(response.getSuccess(), "Schema upsert should succeed");
        System.out.println("Upsert Schema Response: " + response.getMessage());
    }

    @GraveyardEntity("integration_user")
    static class IntegrationUser {
        @GraveyardField(minLength = 3)
        String username;
        
        @GraveyardField(min = 18)
        int age;
    }
}
