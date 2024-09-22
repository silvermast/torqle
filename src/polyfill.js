import at from 'array.prototype.at';
Array.prototype.at = at; // fix macos webviews

window.structuredClone = val => JSON.parse(JSON.stringify(val));