import { defineStore } from 'pinia';
import { ref } from 'vue';
import { v4 as uuidv4 } from 'uuid';
import { fetchFavorites, storeFavorites } from '~/services/Favorites.js';
import { makeHappySnack, makeSpicySnack } from '~/components/Snacks.vue';

export const defaultColor = '#4caf50';

export default defineStore('favorites', () => {

    const selection = ref({});
    const favorites = ref([]);

    function setDefaultSelection() {
        selection.value = {
            id: null,
            name: '',
            color: defaultColor,
            canSsh: true,
            useSsh: false,
            sshOpts: {
                user: '',
                host: '',
                port: 22,
                password: '',
                keyfile: null,
            },
            driverName: null,
            driverOpts: {},
        }
    }

    function canSsh() {
        return !/(sqlite|test)/i.test(selection.value.driverName);
    }

    function setSelection(payload) {
        payload.canSsh = payload.driverName === 'SQLite' ? false : payload.canSsh;
        selection.value = payload;
    }

    async function loadFavorites() {
        try {
            favorites.value = await fetchFavorites();
        } catch (e) {
            makeSpicySnack(e);
            console.error(e);
        }
    }

    function getFavoriteName(favorite) {
        return favorite.name ?? favorite.label ?? favorite.title;
    }

    async function deleteFavorite(favorite) {
        if (selection.value.id === favorite.id) {
            setDefaultSelection();
        }
        favorites.value.splice(favorites.value.indexOf(favorite), 1);
        await storeFavorites(favorites.value);
    }

    async function dupeFavorite(favorite) {
        const favoriteIndex = favorites.value.indexOf(favorite);
        const newFavorite = { ...favorite, id: uuidv4(), name: `${favorite.name} - Copy` };
        favorites.value.splice(favoriteIndex + 1, 0, newFavorite);
        await storeFavorites(favorites.value);
    }

    async function saveFavorite(favorite) {
        if (favorite.id) {
            favorites.value = favorites.value.map(fav => fav.id === favorite.id ? { ...favorite } : fav);
        } else {
            const newFavorite = { ...favorite, id: uuidv4() };
            favorites.value.push(newFavorite);
            selection.value = newFavorite;
        }

        try {
            await storeFavorites(favorites.value);
            makeHappySnack('Saved');
        } catch (e) {
            console.error(e);
            makeSpicySnack(e.toString());
        }
    }

    setDefaultSelection();

    return { selection, favorites, loadFavorites, saveFavorite, setDefaultSelection, setSelection, dupeFavorite, deleteFavorite, getFavoriteName };
});