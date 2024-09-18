import { invoke } from '@tauri-apps/api/core';
import QueryResult from '~/services/QueryResult';

/**
 * @typedef QueryResult
 * @property elapsed_ms String,
 * @property num_rows String,
 * @property fields Array<String>,
 * @property rows Array<Object>,
 */

class Connector {
    options = {};
    color = '#2196f3';

    constructor({ color, ...options }) {
        if (color) {
            this.color = color;
        }
        this.options = options;
    }

    get dbHost() {
        return this.options?.driverOpts?.host;
    }

    get type() {
        return this.options?.driverName;
    }

    get connectOpts() {
        return {
            ...this.options,
            driverOpts: {
                driver: this.options.driverName,
                ...this.options.driverOpts,
                host: this.options.driverOpts?.host ?? '',
                port: Number(this.options.driverOpts?.port ?? 0),
                user: this.options.driverOpts?.user ?? '',
                password: this.options.driverOpts?.password ?? '',
                filepath: this.options.driverOpts?.filepath ?? '',
            },
            sshOpts: !this.options.useSsh ? null : {
                host: this.options.sshOpts?.host ?? '',
                port: Number(this.options.sshOpts?.port ?? 22),
                user: this.options.sshOpts?.user ?? '',
                password: this.options.sshOpts?.password ?? '',
                keyfile: this.options.sshOpts?.keyfile ?? undefined,
            }
        }
    }

    setDatabase() { throw Error('setDatabase not implemented') }
    getDatabase() { throw Error('getDatabase not implemented') }
    async loadDatabases() { throw Error('loadDatabases not implemented') }
    async loadTables() { throw Error('loadTables not implemented') }

    /**
     * Tells the rust-end to create a connection. Internally, this is bound to the window.
     */
    async connect() {
        return await invoke('adapter_connect', this.connectOpts);
    }

    /**
     * Tells the rust-end connection to disconnect
     */
    async disconnect() {
        return await invoke('adapter_disconnect');
    }

    /**
     * Attempts to reconnect
     */
    async reconnect() {
        try {
            await this.disconnect();
        } catch (e) {
            console.warn(e);
        }
        await this.connect();
    }

    /**
     * Tests the connection info
     */
    async test() {
        const options = structuredClone(this.connectOpts);
        console.log('adapter_test', options);
        return await invoke('adapter_test', options);
    }

    /**
     * Runs a query
     * @TODO add "cancel" capability
     * @param {String} query
     * @param {String} database -- optional
     */
    async query(query) {
        const database = this.getDatabase();
        const response = await invoke('adapter_query', { query, database });
        console.log('invoke adapter_query', { query, database, response });
        return new QueryResult(response);
    }
}

export { Connector };