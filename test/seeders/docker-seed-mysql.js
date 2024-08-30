import knex from 'knex';
import { faker } from '@faker-js/faker';

const mysql = knex({
    client: 'mysql2', connection: {
        host: 'localhost',
        user: 'root',
        password: 'mypassword',
        database: 'testdb',
    }
});

await mysql.schema.dropTableIfExists('users');
await mysql.schema.createTable('users', table => {
    table.increments('id').primary();
    table.string('username', 50);
    table.string('email', 100);
    table.text('bio');
    table.timestamp('created_at');
});

await mysql('users').insert(
    new Array(1000).fill(0).map(() => ({
        username: faker.internet.userName(),
        email: faker.internet.email(),
        bio: faker.person.bio(),
        created_at: faker.date.anytime(),
    }))
);

console.log('Tables created successfully.');

await mysql.destroy();
