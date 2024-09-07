import { MongoClient } from 'mongodb';
import { faker } from '@faker-js/faker';

const client = new MongoClient('localhost');
await client.connect();
const db = client.db('testdb');

const users = db.collection('users');

await users.insertMany(
    new Array(1000).fill(0).map(() => ({
        username: faker.internet.userName(),
        email: faker.internet.email(),
        bio: faker.person.bio(),
        created_at: faker.date.anytime(),
    }))
);

console.log('Records created successfully.');
await client.close();
