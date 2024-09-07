import { Connector } from './Connector.js';

class MysqlConnector extends Connector {
    getDatabase() {
        return this.options?.driverOpts?.database ?? null;
    }
    setDatabase(schema) {
        this.options.driverOpts.database = schema;
    }

    async disconnect() {
        return true;
    }

    async loadDatabases() {
        const result = await this.query('SHOW DATABASES;');
        console.log('loadSchemas', result);
        return result?.rows?.map(row => Object.values(row)[0]);
    }

    async loadTables() {
        const schema = this.getDatabase();
        if (schema) {
            const result = await this.query(`SHOW TABLES IN ${schema};`);
            console.log('loadTables', result);
            return result?.rows?.map(row => Object.values(row)[0]);
        }
    }
}
export { MysqlConnector };