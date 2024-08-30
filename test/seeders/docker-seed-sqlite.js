import knex from 'knex';
import { faker } from '@faker-js/faker';

const connection = knex({
    client: 'sqlite3', connection: {
        filename: './.docker/testdb.sqlite',
    },
    useNullAsDefault: true,
});

await connection.schema.dropTableIfExists('users');
await connection.schema.createTable('users', table => {
    table.increments('id').primary();
    table.string('username', 50);
    table.string('email', 100);
    table.text('bio');
    table.timestamp('created_at');
});

for (const i in new Array(10)) {
    await connection('users').insert(
        new Array(100).fill(0).map(() => ({
            username: faker.internet.userName(),
            email: faker.internet.email(),
            bio: faker.person.bio(),
            created_at: faker.date.anytime(),
        }))
    );
}

console.log('Tables created successfully.');

await connection.destroy();
