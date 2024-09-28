/**
 * @param {*} value
 * @returns boolean True if the value is null or undefined, false otherwise
 */
export function isNull(value) {
    return value === null || typeof value === 'undefined';
}

/**
 * @param {*} value
 * @returns boolean True if the value is a number or numeric string, false otherwise
 */
export function isNumeric(value) {
    return !isNaN(parseFloat(value)) && isFinite(value);
}

/**
 * @param {*} value
 * @returns number|null The number value of the input, or null if the input is not numeric
 */
export function toNumber(value) {
    return isNumeric(value) ? Number(value) : null;
}

/**
 * Awaitable function to asyncronously wait for a specified number of milliseconds
 * @param {number} ms The number of milliseconds to wait
 */
export function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
