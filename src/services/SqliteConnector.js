import { Connector } from './Connector';

class SqliteConnector extends Connector {
    getDatabase() {
        return this.opts?.driverOpts?.database ?? null;
    }
    setDatabase(schema) {
        this.opts.driverOpts.database = schema;
    }
    
    async loadDatabases() {
        const [rows] = await this.query('.databases');
        return rows.map(row => Object.values(row)[0]);
    }

    async loadTables() {
        const [rows] = await this.query('.tables');
        return rows.map(row => Object.values(row)[0]);
    }
}

export { SqliteConnector };