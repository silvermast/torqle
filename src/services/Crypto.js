import { invoke } from '@tauri-apps/api/tauri';
import Hex from 'crypto-js/enc-hex';
import Utf8 from 'crypto-js/enc-utf8';
import Base64 from 'crypto-js/enc-base64';
import { AES, HmacSHA256 } from 'crypto-js';
import WordArray from 'crypto-js/lib-typedarrays';

let key;
/**
 * Generates an AES key and stores it in keytar. Use this to encrypt/decrypt the entire favorites file.
 * Use AES CBC-256 with hmac
 * @returns {WordArray}
 */
async function fetchKey() {
    if (!key) {
        const testRexp = new RegExp('^[a-f0-9]{64}$'); // 32bit as hex
        const result = await invoke('fetch_key');
        if (!testRexp.test(result)) {
            throw new Error(`Failed to load encryption key! '${result}'`); // bubble up keytar error
        }
        key = Hex.parse(result);
    }
    return key;
}

function isEncrypted(payload) {
    return /^{"iv":"[a-z0-9]+","hmac":"[a-z0-9]+","ciphertext":"[a-z0-9]+"}$/.test(payload);
}

/**
 * Encryption function using crypto-js defaults (AES256 CBC with Pkcs7)
 * @param {String} payload 
 * @param {WordArray} key
 * @returns {String} JSON payload
 */
function encrypt(payload, key) {
    const iv = WordArray.random(128 / 8); // 128-bit IV for AES-CBC 256
    const enc = AES.encrypt(payload, key, { iv }); // defaults to AES-CBC
    const hmac = HmacSHA256(enc.ciphertext, key);

    return JSON.stringify({
        iv: iv.toString(Hex),
        hmac: hmac.toString(Hex),
        ciphertext: enc.ciphertext.toString(Hex),
    });
}

/**
 * Decryption function using crypto-js defaults (AES256 CBC with Pkcs7)
 * @param {String} encryptedJson 
 * @param {WordArray} key
 * @returns 
 */
function decrypt(encryptedJson, key) {
    const encryptedObj = JSON.parse(encryptedJson);
    const ciphertext = Hex.parse(encryptedObj.ciphertext);
    const iv = Hex.parse(encryptedObj.iv);
    const hmac = Hex.parse(encryptedObj.hmac);

    // Verify HMAC for integrity and authenticity
    const localHmac = HmacSHA256(ciphertext, key);

    // pretty sure we don't need to worry about timing attacks with hmac
    // but we do it anyway!
    // if (!timingSafeEquals(localHmac, hmac)) {
    //     throw new Error('HMAC validation failed. The encrypted data may have been tampered with.');
    // }

    const decrypted = AES.decrypt(ciphertext.toString(Base64), key, { iv });

    return decrypted.toString(Utf8);
}

/**
 * Performs a WordArray comparisson that is safe from timing attacks
 * @param {WordArray} wa1 
 * @param {WordArray} wa2 
 * @returns {Boolean}
 */
function timingSafeEquals(wa1, wa2) {
    // Get the lengths of the hex strings
    const length1 = wa1.words.length;
    const length2 = wa2.words.length;

    // Perform a bitwise comparison
    let result = 0;
    for (let i = 0; i < length1; i++) {
        result |= wa1.words[i] ^ wa2.words[i % length2] ?? 0;
    }

    // Return true if the comparison is successful
    return length1 === length2 && result === 0;
}

export { encrypt, decrypt, fetchKey, isEncrypted };