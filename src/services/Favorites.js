import { 
    BaseDirectory,
    exists,
    readTextFile,
    writeTextFile,
    createDir,
} from '@tauri-apps/plugin-fs';
import { encrypt, decrypt, fetchKey, isEncrypted } from './Crypto.js';

const fileOpts = { dir: BaseDirectory.AppLocalData };

/**
 * @returns {Array<Object>}
 */
async function fetchFavorites() {
    if (await exists('data/favorites.json', fileOpts)) {
        const payload = await readTextFile('data/favorites.json', fileOpts);

        if (isEncrypted(payload)) {
            const appKey = await fetchKey();
            const decrypted = decrypt(payload, appKey);
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
