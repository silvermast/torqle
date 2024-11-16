<script setup>
import useFavoritesStore from '~/store/main.js';
import { defineEmits } from 'vue';

const emit = defineEmits('connect');
const store = useFavoritesStore();


/**
 * on doubleclick, connect. on single click, set favorite.
 */
 let lastClickTime = 0;
function setOrConnect(favorite) {
    const elapsed = Date.now() - lastClickTime;
    // possible race condition?
    if (favorite.id && favorite.id === store.selection.id && elapsed < 300) {
        emit('connect');
    } else {
        lastClickTime = Date.now();
        store.setSelection(favorite);
    }
}

store.loadFavorites();

</script>

<template>
    <v-list lines="one" density="compact">
        <v-list-item :active="!store.selection.id" @click="store.setDefaultSelection" prepend-icon="mdi-plus">
            New Connection
        </v-list-item>
        <v-list-item v-for="favorite in store.favorites" :key="favorite.id" :title="store.getFavoriteName(favorite)"
            :active="favorite.id === store.selection.id" @click="setOrConnect(favorite)">
            <template v-slot:prepend>
                <v-icon :color="favorite.color ?? 'gray'" icon="mdi-circle" />
            </template>
            <template v-slot:append>
                <v-menu>
                    <template v-slot:activator="{ props }">
                        <v-icon icon="mdi-dots-vertical" v-bind="props"></v-icon>
                    </template>
                    <v-list>
                        <v-list-item @click="store.dupeFavorite(favorite)">Duplicate</v-list-item>
                        <v-list-item @click="store.deleteFavorite(favorite)">Delete</v-list-item>
                    </v-list>
                </v-menu>
            </template>
        </v-list-item>
    </v-list>
</template>