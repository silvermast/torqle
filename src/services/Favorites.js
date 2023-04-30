import { 
    BaseDirectory,
    exists,
    readTextFile,
    writeTextFile,
    createDir,
} from '@tauri-apps/api/fs';

const fileOpts = { dir: BaseDirectory.AppLocalData };

async function fetchFavorites() {
    if (await exists('data/favorites.json', fileOpts)) {
        const favoritesTxt = await readTextFile('data/favorites.json', fileOpts);
        return JSON.parse(favoritesTxt);
    } else {
        return [];
    }
}

async function storeFavorites(favorites) {
    if (!await exists('data', fileOpts)) {
        await createDir('data', { ...fileOpts, recursive: true });
    }

    return await writeTextFile('data/favorites.json', JSON.stringify(favorites), fileOpts);
}

export { fetchFavorites, storeFavorites };
