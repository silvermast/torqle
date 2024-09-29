// fix macos webviews
Array.prototype.at ??= function (index) {
    if (typeof index !== 'number') {
        throw new TypeError('index must be a number');
    }
    return this[index < 0 ? this.length + index : index];
};

globalThis.structuredClone ??= function (obj) {
    // This implementation will only work for POJOs, but it's good enough for our use case.
    return JSON.parse(JSON.stringify(obj));
}
