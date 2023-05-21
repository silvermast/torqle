import crypto from 'crypto';
import { invoke } from '@tauri-apps/api/tauri';

let key;
/**
 * Generates an AES key and stores it in keytar. Use this to encrypt/decrypt the entire favorites file.
 * Use AES CBC-256 with hmac
 * @returns Buffer
 */
async function fetchKey() {
    if (!key) {
        const result = await invoke('fetch_key');
        if (!/^[a-f0-9]{32}$/.match(result)) {
            throw new Error(result); // bubble up keytar error
        }
        key = Buffer.from(result, 'hex');
    }
    return key;
}

function isEncrypted(payload) {
    return /^{"iv":"[a-z0-9]+","hmac":"[a-z0-9]+","ciphertext":"[a-z0-9]+"}$/.test(payload);
}

/**
 * Encryption function using AES-CBC with PKCS7 padding and HMAC
 * @param {String} payload 
 * @param {Buffer} key
 * @returns {String} JSON payload
 */
function encrypt(payload, key) {
    const iv = crypto.randomBytes(16); // 128-bit IV for AES-CBC
    const cipher = crypto.createCipheriv('aes-256-cbc', key, iv);

    // Enable PKCS7 padding
    cipher.setAutoPadding(true);

    // Encrypt with padding
    const encrypted = Buffer.concat([cipher.update(payload, 'utf8'), cipher.final()]);

    // Generate HMAC for integrity and authenticity
    const hmac = crypto.createHmac('sha256', key).update(encrypted).digest();

    return JSON.stringify({
        iv: iv.toString('hex'),
        hmac: hmac.toString('hex'),
        ciphertext: encrypted.toString('hex'),
    });
}

/**
 * Decryption function using AES-CBC with PKCS7 padding and HMAC validation
 * @param {String} encryptedJson 
 * @param {Buffer} key
 * @returns 
 */
async function decrypt(encryptedJson, key) {
    const encryptedObj = JSON.parse(encryptedJson);
    const ciphertext = Buffer.from(encryptedObj.ciphertext, 'hex');
    const iv = Buffer.from(encryptedObj.iv, 'hex');
    const hmac = Buffer.from(encryptedObj.hmac, 'hex');

    // Verify HMAC for integrity and authenticity
    const localHmac = crypto.createHmac('sha256', key).update(ciphertext).digest();

    // Compare the received HMAC with the calculated HMAC
    if (!crypto.timingSafeEqual(localHmac, hmac)) {
        throw new Error('HMAC validation failed. The encrypted data may have been tampered with.');
    }

    const decipher = crypto.createDecipheriv('aes-256-cbc', key, iv);

    // Enable PKCS7 padding
    decipher.setAutoPadding(true);

    const decrypted = Buffer.concat([decipher.update(ciphertext), decipher.final()]);

    return decrypted.toString('utf8');
}

export { encrypt, decrypt, fetchKey, isEncrypted };