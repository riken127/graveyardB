import { EventStoreClient } from '../src/client';
import { GraveyardEntity } from '../src/decorators/entity';
import { GraveyardField } from '../src/decorators/field';
import { SchemaGenerator } from '../src/schema/generator';

@GraveyardEntity("user_test")
class UserTest {
    @GraveyardField({ minLength: 3 })
    username!: string;

    @GraveyardField({ min: 18 })
    age!: number;
}

describe('SchemaGenerator', () => {
    it('should generate schema with constraints', () => {
        const schema = SchemaGenerator.generate(UserTest);
        expect(schema.name).toBe("user_test");
        expect(schema.fields['username']).toBeDefined();
        expect(schema.fields['username'].constraints?.minLength).toBe(3);
        expect(schema.fields['age']).toBeDefined();
        expect(schema.fields['age'].constraints?.minValue).toBe(18);
    });
});

describe('EventStoreClient', () => {
    // Note: To fully mock grpc-js in jest requires more setup or separate integration test
    // For now, we are verifying compilation and basic class structure.
    // In a real scenario, we would mock the GrpcClient prototype.

    let client: EventStoreClient;

    beforeEach(() => {
        client = new EventStoreClient({ host: 'localhost', port: 50051 });
    });

    afterEach(() => {
        client.close();
    });

    it('should instantiate', () => {
        expect(client).toBeDefined();
    });
});
