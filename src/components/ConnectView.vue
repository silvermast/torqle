<script setup>
import { fetchFavorites, storeFavorites } from '~/services/Favorites.js';
import { v4 as uuidv4 } from 'uuid';
import { ref, computed } from 'vue';

import MysqlFieldset from './fieldsets/MysqlFieldset.vue';

const emit = defineEmits('connect');

async function connect() {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  isConnecting.value = true;
  setTimeout(() => {
    isConnecting.value = false;
    emit('connect', { ...connection.value });
  }, 500);
}

async function loadFavorites() {
  favorites.value = await fetchFavorites();
}

async function dupeFavorite() {
  const incrRegex = new RegExp(' [0-9]+$');
  const newConnection = { ...connection.value, id: uuidv4(), label: `${connection.value.label} - Copy` };
  favorites.value.push(newConnection);
  connection.value = newConnection;
}

async function saveFavorite() {
  if (connection.value.id) {
    favorites.value = favorites.value.map(fav => fav.id === connection.value.id ? { ...connection.value } : fav);
  } else {
    const newFavorite = { ...connection.value, id: uuidv4() };
    favorites.value.push(newFavorite);
    connection.value = newFavorite;
  }

  try {
    await storeFavorites(favorites.value);
  } catch (e) {
    console.error(e);
    snack({ text: e.toString(), color: 'red' });
  }
}

function snack({ text, color }) {
  try {
    snackText.value = text;
    snackColor.value = color;
    snackEnabled.value = true;
  } catch (e) {
    console.error('snack:', e);
  }
}

function testConnection() {
  snackEnabled.value = false;
  snack({ text: 'Failed to connect.', color: 'red' });
}

function setConnection(payload) {
  connection.value = { ...JSON.parse(JSON.stringify(payload)) };
}


const typeOptions = [
  { label: 'MySQL', value: 'mysql' },
  { label: 'PostgreSQL', value: 'postgresql' },
  { label: 'MongoDB', value: 'mongodb' },
  { label: 'SQLite', value: 'sqlite' },
];
// const typeOptions = ['mysql', 'postgresql', 'mongodb', 'sqlite'];

const connection = ref({
  id: null,
  title: '',
  type: null,
});

const snackEnabled = ref(false);
const snackText = ref('');
const snackColor = ref('gray');

const isConnecting = ref(false);
const favorites = ref([]);

const isNew = computed(() => !connection.value.id);

loadFavorites();

</script>

<template>
  <div class="connect-view d-flex">
    <v-sheet width="250" class="pa-3 flex-grow-0">
      <v-list lines="one">
        <v-list-item :active="!connection.id" @click="setConnection({})">
          <v-icon icon="mdi-plus" />
          New Connection
        </v-list-item>
        <v-list-item 
          v-for="favorite in favorites" 
          :key="favorite.id" 
          :title="favorite.label"
          :active="favorite.id && favorite.id === connection.id"
          @click="setConnection(favorite)"
        ></v-list-item>
      </v-list>
    </v-sheet>
    <div class="flex-grow-1 align-self-center">

      <v-snackbar v-model="snackEnabled" :color="snackColor" contained location="top">
        {{ snackText }}

        <template v-slot:actions>
          <v-btn size="small" rounded variant="tonal" @click="snackEnabled = false">
            Dismiss
          </v-btn>
        </template>
      </v-snackbar>

      <v-card max-width="500" class="mx-auto pa-5">
        <v-text-field v-model="connection.label" variant="underlined" label="Connection Label"></v-text-field>
        <v-select v-model="connection.type" :items="typeOptions" item-title="label" item-value="item" variant="underlined" label="Connection Type"></v-select>
        <MysqlFieldset v-model="connection" />

        <v-card-actions class="d-flex">
          <v-btn size="small" class="" rounded @click="saveFavorite" variant="outlined">Save</v-btn>
          <v-btn size="small" class="mr-auto" rounded @click="dupeFavorite" variant="outlined" :disabled="isNew">Duplicate</v-btn>

          <v-progress-circular v-if="isConnecting" indeterminate></v-progress-circular>

          <v-btn size="small" class="mr-2" rounded @click="testConnection" variant="outlined" :disabled="isConnecting">Test</v-btn>
          <v-btn size="small" rounded color="primary" type="submit" variant="elevated" @click="connect" :disabled="isConnecting">Connect</v-btn>
        </v-card-actions>
      </v-card>
    </div>
  </div>
</template>

<style scoped>
.connect-view {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  height: 100vh;
  overflow: hidden;
}
</style>