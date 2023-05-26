import { assert, describe, expect, it } from 'vitest';
import { encrypt, decrypt, isEncrypted } from './Crypto';
import WordArray from 'crypto-js/lib-typedarrays';

describe('Crypto.js', () => {
    it('can encrypt and decrypt', async () => {
        const key = WordArray.random(32 / 8);

        const payload = JSON.stringify({value: 'this is a test'});

        const encrypted = encrypt(payload, key);
        assert(isEncrypted(encrypted));

        const decrypted = decrypt(encrypted, key);
        assert(decrypted === payload);
    });
})