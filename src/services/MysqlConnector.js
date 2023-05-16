import { Connector } from './Connector.js';

class MysqlConnector extends Connector {
    getSchema() {
        return this.driverOpts.database ?? null;
    }

    async loadSchemas() {
        const [rows] = await this.query('SHOW DATABASES;');
        return rows.map(row => Object.values(row)[0]);
    }

    async loadTables() {
        const [rows] = await this.query('SHOW TABLES;');
        return rows.map(row => Object.values(row)[0]);
    }
}
export { MysqlConnector };