import { defineStore } from 'pinia';
import { ref } from 'vue';
import { v4 as uuidv4 } from 'uuid';
import { fetchFavorites, storeFavorites } from '~/services/Favorites.js';
import { makeHappySnack, makeSpicySnack } from '~/components/Snacks.vue';

export default defineStore('favorites', () => {

    const selection = ref({});
    const favorites = ref([]);

    function setDefaultSelection() {
        selection.value = {
            id: null,
            title: '',
            color: 'primary', // update!
            useSsh: false,
            sshOpts: {},
            driverName: null,
            driverOpts: {},
        }
    }

    function setSelection(payload) {
        // const currentConnection = JSON.parse(JSON.stringify(payload));
        // currentConnection.driverOpts = currentConnection.driverOpts ?? {};
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

    async function deleteFavorite(favorite) {
        if (selection.value.id === favorite.id) {
            setDefaultSelection();
        }
        favorites.value.splice(favorites.value.indexOf(favorite), 1);
        await storeFavorites(favorites.value);
    }

    async function dupeFavorite(favorite) {
        const favoriteIndex = favorites.value.indexOf(favorite);
        const newFavorite = { ...favorite, id: uuidv4(), label: `${favorite.label} - Copy` };
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

    return { selection, favorites, loadFavorites, saveFavorite, setDefaultSelection, setSelection, dupeFavorite, deleteFavorite };
});