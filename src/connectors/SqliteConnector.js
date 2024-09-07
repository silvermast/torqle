import { Connector } from './Connector.js';

class SqliteConnector extends Connector {
    getDatabase() {
        return this.options?.driverOpts?.database ?? null;
    }
    setDatabase(schema) {
        this.options.driverOpts.database = schema;
    }
    
    async loadDatabases() {
        try {
            const { rows } = await this.query('PRAGMA database_list');
            return rows?.map(row => Object.values(row)[1] ?? null).filter(v => v);

        } catch (error) {
            console.error(error);
            return [];
        }
    }

    async loadTables() {
        // note: sqlite has changed its internal tablespaces a few times over the years.
        // To ensure compatibility, let's check for all of them.
        // @see https://sqlite.org/schematab.html
        const tables = [
            'sqlite_schema',
            'sqlite_master',
        ];

        try {
            const { rows } = await Promise.any(
                tables.map(async table => {
                    const rsp = await this.query(`SELECT name FROM ${table} WHERE type IN("table", "view") AND name NOT LIKE "sqlite_%"`);
                    if (!rsp?.fields?.length) {
                        throw new Error(`${table} does not exist`);
                    }
                    return rsp;
                })
            );
            return rows.map(row => Object.values(row)[0]);
        } catch (error) {
            console.error(error);
            return [];
        }

    }
}

export { SqliteConnector };