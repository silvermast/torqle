import { Connector } from './Connector.js';
import { faker } from '@faker-js/faker';
import QueryResult from '~/services/QueryResult.js';
import { sleep } from '~/helpers.mjs';

class TestConnector extends Connector {
    database = null;

    getDatabase() {
        return this.database;
    }
    setDatabase(database) {
        return this.database = database;
    }

    async connect() {
        await sleep(2_000);
        return true;
    }

    async disconnect() {
        return true;
    }

    async reconnect() {
        await sleep(2_000);
        return true;
    }

    async test() {
        await sleep(1_000);
        if (0.5 - Math.random()) {
            throw Error('Test Failed');
        }
        return 'Test Passed';
    }

    async loadDatabases() {
        await sleep(100);
        return Array(20).fill(null).map(() => faker.internet.displayName());
    }

    async loadTables() {
        await sleep(100);
        return Array(100).fill(null).map(() => faker.internet.displayName());
    }

    async query(query, database) {
        if (query === 'error') {
            throw new Error(`Mock error message: ${faker.lorem.sentence()}`)
        }
        
        const elapsed_ms = 1000 * Math.random();

        await sleep(elapsed_ms);

        console.log({ query });
        const num_rows = Math.floor(Math.random() * 1000);
        const rows = Array(num_rows).fill(null).map((v, id) => ({
            id,
            userId: faker.string.uuid(),
            username: faker.internet.userName(),
            email: faker.internet.email(),
            bio: faker.person.bio(),
            paragraph: faker.lorem.paragraphs(),
            avatar: faker.image.avatar(),
            password: faker.internet.password(),
            birthdate: faker.date.birthdate().toJSON(),
            registeredAt: faker.date.past().toJSON(),
            secondary_contact_email: faker.internet.email(),
            secondary_contact_avatar: faker.image.avatar(),
            secondary_contact_birthdate: faker.date.birthdate().toJSON(),
        }));
        const queryResult = new QueryResult({
            rows,
            fields: Object.keys(rows[0]),
            num_rows,
            elapsed_ms,
        });
        console.log('QueryResult:', queryResult);
        return queryResult;
    }
}
export { TestConnector };