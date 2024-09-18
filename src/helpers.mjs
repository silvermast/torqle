function isNull(value) {
    return value === null || value === undefined;
}

async function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

window.isNull = isNull;
window.sleep = sleep;
export { isNull, sleep }