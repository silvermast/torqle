<script setup>
import { computed, ref, watch } from 'vue';
import { hideSnack, makeHappySnack, makeSpicySnack } from '~/components/Snacks.vue';
import Password from '~/components/Password.vue';
import FavoritesList from '~/components/FavoritesList.vue';
import { open } from '@tauri-apps/api/dialog';

import useFavoritesStore from '~/store/main.js';

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

const emit = defineEmits(['connect']);
const store = useFavoritesStore();

const connection = computed(() => store.selection);

const isConnecting = ref(false);
const isOpeningFile = ref(false);

const driver = computed(() => drivers.find(d => d.label === connection.value.driverName));

const lastError = ref();

watch(connection, () => lastError.value = null);

async function testConnection() {
  hideSnack();
  isConnecting.value = true;
  lastError.value = null
  try {
    const connector = new driver.value.connector({ ...connection.value });
    const rsp = await connector.test();
    makeHappySnack(rsp);
  } catch (e) {
    console.error(e);
    lastError.value = (e.error ?? e).toString();
    // makeSpicySnack(e);
  }
  isConnecting.value = false;
}

async function connect() {
  lastError.value = null;
  isConnecting.value = true;
  try {
    console.log('Connecting', connection.value);
    const connector = new driver.value.connector({ ...connection.value });
    await connector.connect();
    emit('connect', connector);
  } catch (e) {
    console.error(e);
    lastError.value = (e.error ?? e).toString();
    // makeSpicySnack(e);
  }
  isConnecting.value = false;
}

async function openSshKeyDialog() {
  isOpeningFile.value = true;
  connection.value.sshOpts.keyfile = await open();
  isOpeningFile.value = false;
}

document.title = 'New Connection';

if (/localhost:1420/.test(window.location)) {
  connection.value.driverName = 'Test';
}

</script>

<template>
  <div class="connect-view d-flex">
    <v-sheet width="250" class="flex-grow-0">
      <FavoritesList @connect="connect" />
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
          <v-row>
            <v-col cols="8">
              <v-text-field label="SSH Host" density="compact" v-model="connection.sshOpts.host" variant="outlined" />
            </v-col>
            <v-col cols="4">
              <v-text-field label="Port" density="compact" v-model="connection.sshOpts.port" variant="outlined" type="number" />
            </v-col>
          </v-row>
          <v-text-field label="SSH User" density="compact" v-model="connection.sshOpts.user" variant="outlined" />
          <v-row>
            <v-col cols="6">
              <Password label="SSH Password" density="compact" v-model="connection.sshOpts.password" variant="outlined" />
            </v-col>
            <v-col cols="6" class="pl-3">
              <v-btn prepend-icon="mdi-shield-key" @click="openSshKeyDialog" :disabled="isOpeningFile" variant="outlined" color="grey">
                Select SSH Key
              </v-btn>
              <div><small v-if="connection.sshOpts.keyfile" v-text="connection.sshOpts.keyfile" /></div>
            </v-col>
          </v-row>
        </div>

        <v-divider class="mb-5" />

        <component v-if="driver" :is="driver.fieldset" v-model="connection.driverOpts" />

        <v-alert class="mb-2" v-if="lastError" :text="lastError" type="error" />

        <v-card-actions class="d-flex">
          <v-btn class="mr-auto" rounded @click="store.saveFavorite(connection)" variant="outlined">Save</v-btn>

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