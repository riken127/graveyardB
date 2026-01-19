export interface EventStoreConfig {
    host: string;
    port: number;
    useTls: boolean;
    timeoutMs: number;
}

export const defaultConfig: EventStoreConfig = {
    host: 'localhost',
    port: 50051,
    useTls: false,
    timeoutMs: 5000
};
