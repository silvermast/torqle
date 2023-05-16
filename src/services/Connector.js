import { invoke } from '@tauri-apps/api/tauri';

class Connector {
    driverName = null;
    driverOpts = null;

    constructor({ driverName, driverOpts }) {
        this.driverName = driverName;
        this.driverOpts = driverOpts;
    }

    async getSchema() { throw Error('getSchema not implemented') }
    async loadSchemas() { throw Error('loadSchemas not implemented') }
    async loadTables() { throw Error('loadTables not implemented') }

    /**
     * Tells the rust-end to create a connection. Internally, this is bound to the window.
     */
    async connect() {
        return await invoke('connect', { driverName: this.driverName, driverOpts: this.driverOpts });
    }

    /**
     * Tells the rust-end connection to disconnect
     */
    async disconnect() {
        return await invoke('disconnect');
    }

    /**
     * Tests the connection info
     */
    async test() {
        return await invoke('test_connection', { driverName: this.driverName, driverOpts: this.driverOpts });
    }

    /**
     * Runs a query
     * @TODO add "cancel" capability
     * @param {String} query
     */
    async query(query) {
        return invoke('query', { query });
    }

    /**
     * Tells the backend to change the schema
     * @param {String} schema 
     * @returns 
     */
    async changeSchema(schema) {
        return await invoke('change_schema', { schema });
    }
}

export { Connector };