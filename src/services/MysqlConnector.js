import { Connector } from './Connector.js';

class MysqlConnector extends Connector {
    getSchema() {
        return this.driverOpts.database ?? null;
    }

    async disconnect() {
        return true;
    }

    async loadSchemas() {
        const result = await this.query('SHOW DATABASES;');
        console.log('loadSchemas', schemas);
        return result?.rows?.map(row => Object.values(row)[0]);
    }

    async loadTables() {
        const result = await this.query('SHOW TABLES;');
        console.log('loadTables', result);
        return result?.rows?.map(row => Object.values(row)[0]);
    }
}
export { MysqlConnector };