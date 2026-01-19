package com.eventstore.client.config;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class EventStoreConfig {

    @Value("${eventstore.host:localhost}")
    private String host;

    @Value("${eventstore.port:50051}")
    private int port;

    @Value("${eventstore.use-tls:false}")
    private boolean useTls;

    @Value("${eventstore.timeout-ms:5000}")
    private long timeoutMs;

    public long getTimeoutMs() {
        return timeoutMs;
    }

    public void setTimeoutMs(long timeoutMs) {
        this.timeoutMs = timeoutMs;
    }

    @Bean
    public ManagedChannel eventStoreChannel() {
        ManagedChannelBuilder<?> builder = ManagedChannelBuilder.forAddress(host, port);
        
        if (useTls) {
            builder.useTransportSecurity();
        } else {
            builder.usePlaintext();
        }
        
        return builder.build();
    }
}
