import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { v4 as uuidv4 } from 'uuid';

/**
 * Tells the rust-end to create a connection. Internally, this is bound to the window.
 */
async function connect(connectionInfo) {
    return await invoke('connect', { ...connectionInfo });
}

/**
 * Tells the rust-end connection to disconnect
 */
async function disconnect() {
    return await invoke('disconnect');
}

/**
 * Tests the connection info
 */
async function test(connectionInfo) {
    return await invoke('test_connection', { ...connectionInfo });
}

/**
 * @TODO add "cancel" capability
 * 
 * Events:
 * query:request
 * query:response:<uuid>
 */
async function query(query) {
    return invoke('query', { query });
}

/**
 * Tells the backend to change the schema
 * @param {String} schema 
 * @returns 
 */
async function changeSchema(schema) {
    return await invoke('change_schema', { schema });
}

export default {
    connect,
    disconnect,
    test,
    query,
    changeSchema,
};