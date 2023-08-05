import { Connector } from './Connector';
import { faker } from '@faker-js/faker';

class TestConnector extends Connector {
    getSchema() {
        return null;
    }
    setSchema() {
        return null;
    }

    async connect() {
        return true;
    }

    async disconnect() {
        return true;
    }

    async setSchema() {
        return true;
    }

    async test() {
        if (0.5 - Math.random()) {
            throw Error('Test Failed');
        }
        return 'Test Passed';
    }

    async loadSchemas() {
        return Array(20).fill(null).map(() => faker.internet.displayName());
    }

    async loadTables() {
        return Array(100).fill(null).map(() => faker.internet.displayName());
    }

    async query(query, database) {
        if (query === 'error') {
            throw new Error(`Mock error message: ${faker.lorem.sentence()}`)
        }
        console.log({ query });
        return new Promise(resolve => setTimeout(() => {
            let i = 1;
            const num_rows = Math.floor(Math.random() * 1000);
            const rows = Array(num_rows).fill(null).map(() => ({
                id: i++,
                userId: faker.string.uuid(),
                username: faker.internet.userName(),
                email: faker.internet.email(),
                avatar: faker.image.avatar(),
                password: faker.internet.password(),
                birthdate: faker.date.birthdate(),
                registeredAt: faker.date.past(),
                secondary_contact_email: faker.internet.email(),
                secondary_contact_avatar: faker.image.avatar(),
                secondary_contact_birthdate: faker.date.birthdate(),
            }));
            resolve({ rows, num_rows, elapsed_ms: Math.round(Math.random() * 100, 2) });
        }, 5_000))
    }
}
export { TestConnector };