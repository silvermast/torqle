import { 
    BaseDirectory,
    exists,
    readTextFile,
    writeTextFile,
    createDir,
} from '@tauri-apps/api/fs';
import { encrypt, decrypt, fetchKey, isEncrypted } from './Crypto.js';

const fileOpts = { dir: BaseDirectory.AppLocalData };

/**
 * @TODO BEFORE LIVE: implement keytar to securely encrypt passwords -- https://docs.rs/keytar/0.1.3/keytar/
 * macos keychain max length: 16MB
 * win cred manager length: 512B (name & pw combined)
 * linux libsecret length: 64KB
 * 
 * IDEA: generate an AES key and store it in keytar. Use this to encrypt/decrypt the entire favorites file.
 * Use AES CBC256 or GCM
 */

/**
 * @returns {Array<Object>}
 */
async function fetchFavorites() {
    if (await exists('data/favorites.json', fileOpts)) {
        const payload = await readTextFile('data/favorites.json', fileOpts);

        if (isEncrypted(payload)) {
            const appKey = await fetchKey();
            return JSON.parse(decrypt(payload, appKey));
        }

        // encrypt if not encrypted
        const favorites = JSON.parse(payload);
        await storeFavorites(favorites);
        return favorites;

    } else {
        return [];
    }
}

async function storeFavorites(favorites) {
    if (!await exists('data', fileOpts)) {
        await createDir('data', { ...fileOpts, recursive: true });
    }

    const key = await fetchKey();
    const payload = encrypt(JSON.stringify(favorites), key);

    return await writeTextFile('data/favorites.json', payload, fileOpts);
}

export { fetchFavorites, storeFavorites };
