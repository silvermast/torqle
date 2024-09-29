import { toNumber } from '~/helpers.js';

/**
 * @typedef QueryResult
 * @property elapsedMs Number,
 * @property numRows Number,
 * @property fields Array<String>,
 * @property rows Array<Object>,
 */
class QueryResult {
    numRows = null;
    elapsedMs = null;
    rows = null;
    fields = null;

    constructor({ rows, fields, num_rows, elapsed_ms }) {
        this.elapsedMs = toNumber(elapsed_ms);
        this.numRows = toNumber(num_rows);
        this.fields = fields ?? [];
        this.rows = rows ?? [];
    }

    get num_rows() {
        return this.numRows;
    }

    get elapsed_ms() {
        return this.elapsedMs;
    }
}

export default QueryResult;