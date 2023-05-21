<script setup>
import { v4 as uuidv4 } from 'uuid';
import { computed, ref, watch } from 'vue';
import { mdiDotsVertical } from "@mdi/js";
import { Connector } from '~/services/Connector.js';
import { fetchFavorites, storeFavorites } from '~/services/Favorites.js';
import { hideSnack, makeHappySnack, makeSpicySnack } from '~/components/Snacks.vue';
import Password from '~/components/Password.vue';

import { TestConnector } from '~/services/TestConnector.js';
import { MysqlConnector } from '~/services/MysqlConnector.js';
import { SqliteConnector } from '~/services/SqliteConnector.js';

import TestFieldset from '~/components/fieldsets/TestFieldset.vue';
import SqliteFieldset from '~/components/fieldsets/SqliteFieldset.vue';
import MysqlFieldset from '~/components/fieldsets/MysqlFieldset.vue';

const drivers = [
  { label: 'Test', connector: TestConnector, fieldset: TestFieldset },
  { label: 'MySQL', connector: MysqlConnector, fieldset: MysqlFieldset },
  // { label: 'PostgreSQL', value: 'Postgresql' },
  // { label: 'MongoDB', value: 'Mongodb' },
  { label: 'SQLite', connector: SqliteConnector, fieldset: SqliteFieldset },
];

const connection = ref({
  id: null,
  title: '',
  color: 'primary', // update!
  useSsh: false,
  sshOpts: {},
  driverName: null,
  driverOpts: {},
});

const isConnecting = ref(false);
const favorites = ref([]);

const driver = computed(() => drivers.find(d => d.label === connection.value.driverName));

const emit = defineEmits(['connect']);

async function loadFavorites() {
  favorites.value = await fetchFavorites();
}

function deleteFavorite(favorite) {
  if (connection.value.id === favorite.id) {
    setConnection({});
  }
  favorites.value.splice(favorites.value.indexOf(favorite), 1);
}
function dupeFavorite(favorite) {
  const favoriteIndex = favorites.value.indexOf(favorite);
  const newFavorite = { ...favorite, id: uuidv4(), label: `${favorite.label} - Copy` };
  favorites.value.splice(favoriteIndex + 1, 0, newFavorite);
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
    makeSpicySnack(e.toString());
  }
}

async function testConnection() {
  hideSnack();
  try {
    const connector = new driver.value.connector({ ...connection.value });
    const rsp = await connector.test();
    makeHappySnack(rsp);
  } catch (e) {
    makeSpicySnack(e);
  }
}

async function connect() {
  try {
    const connector = new driver.value.connector({ ...connection.value });
    await connector.connect();
    document.title = connection.value.label;
    emit('connect', connector);
  } catch (e) {
    makeSpicySnack(e);
  }
}

function setConnection(payload) {
  connection.value = { ...JSON.parse(JSON.stringify(payload)) };
}

loadFavorites();

document.title = 'New Connection';

</script>

<template>
  <div class="connect-view d-flex">
    <v-sheet width="250" class="flex-grow-0">
      <v-list lines="one" density="compact">
        <v-list-item :active="!connection.id" @click="setConnection({})" prepend-icon="mdi-plus">
          New Connection
        </v-list-item>
        <v-list-item v-for="favorite in favorites" :key="favorite.id" :title="favorite.label"
          :active="favorite.id && favorite.id === connection.id" @click="setConnection(favorite)">
          <template v-slot:prepend>
            <v-icon :color="favorite.color ?? 'gray'" icon="mdi-circle" />
          </template>
          <template v-slot:append>
            <v-menu>
              <template v-slot:activator="{ props }">
                <v-icon icon="mdi-dots-vertical" v-bind="props"></v-icon>
              </template>
              <v-list>
                <v-list-item @click="dupeFavorite(favorite)">Duplicate</v-list-item>
                <v-list-item @click="deleteFavorite(favorite)">Delete</v-list-item>
              </v-list>
            </v-menu>
          </template>
        </v-list-item>
      </v-list>
    </v-sheet>
    <div class="flex-grow-1 overflow-y-auto">
      <v-card max-width="700" class="mx-auto pa-5 my-5">
        <v-row>
          <v-col cols="2">
            <v-menu>
              <template v-slot:activator="{ props }">
                <v-btn :color="connection.color" v-bind="props" rounded>Color</v-btn>
              </template>
              <v-color-picker show-swatches hide-inputs hide-sliders hide-canvas v-model="connection.color" />
            </v-menu>
          </v-col>
          <v-col cols="6">
            <v-text-field density="compact" v-model="connection.label" variant="outlined"
              label="Connection Label"></v-text-field>
          </v-col>
          <v-col cols="4">
            <v-select density="compact" v-model="connection.driverName" :items="drivers" item-title="label"
              item-value="label" variant="outlined" label="Connection Type"></v-select>
          </v-col>
        </v-row>
        <v-switch density="compact" v-model="connection.useSsh" label="SSH Tunnel"></v-switch>
        <div v-if="connection.useSsh">
          <v-text-field density="compact" v-model="connection.sshOpts.host" variant="outlined" label="SSH Host" />
          <v-text-field density="compact" v-model="connection.sshOpts.port" variant="outlined" label="SSH Port" />
          <v-text-field density="compact" v-model="connection.sshOpts.user" variant="outlined" label="SSH User" />
          <Password density="compact" v-model="connection.sshOpts.password" variant="outlined" label="SSH Password" />
        </div>

        <v-divider class="mb-5" />

        <component v-if="driver" :is="driver.fieldset" v-model="connection.driverOpts" />

        <v-card-actions class="d-flex">
          <v-btn class="mr-auto" rounded @click="saveFavorite" variant="outlined">Save</v-btn>

          <v-progress-circular v-if="isConnecting" indeterminate></v-progress-circular>

          <v-btn class="mr-2" rounded @click="testConnection" variant="outlined" :disabled="isConnecting">Test</v-btn>
          <v-btn rounded color="primary" type="submit" variant="elevated" @click="connect"
            :disabled="isConnecting">Connect</v-btn>
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