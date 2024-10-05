<script setup>
import { computed, ref, watch } from 'vue';
import { hideSnack, makeHappySnack, makeSpicySnack } from '~/components/Snacks.vue';
import Password from '~/components/fields/Password.vue';
import FavoritesList from '~/components/FavoritesList.vue';
import ColorPicker from '~/components/fields/ColorPicker.vue';
import ResizeHandle from '~/components/ResizeHandle.vue';
import { open } from '@tauri-apps/plugin-dialog';

import useFavoritesStore from '~/store/main.js';

import { TestConnector } from '~/connectors/TestConnector.js';
import { MysqlConnector } from '~/connectors/MysqlConnector.js';
import { SqliteConnector } from '~/connectors/SqliteConnector.js';

import TestFieldset from '~/components/fieldsets/TestFieldset.vue';
import SqliteFieldset from '~/components/fieldsets/SqliteFieldset.vue';
import MysqlFieldset from '~/components/fieldsets/MysqlFieldset.vue';

const drivers = [
  { label: 'MySQL', connector: MysqlConnector, fieldset: MysqlFieldset },
  // { label: 'PostgreSQL', value: 'Postgresql' },
  // { label: 'MongoDB', value: 'Mongodb' },
  { label: 'SQLite', connector: SqliteConnector, fieldset: SqliteFieldset },
];

if (process.env.NODE_ENV === 'development') {
  drivers.push({ label: 'Test', connector: TestConnector, fieldset: TestFieldset });
}

const emit = defineEmits(['connect']);
const store = useFavoritesStore();

const connection = computed(() => store.selection);

const isConnecting = ref(false);
const isOpeningFile = ref(false);

const driver = computed(() => drivers.find(d => d.label === connection.value.driverName));
const selectedColor = computed(() => connection.value.color ?? 'primary');
const lastError = ref();

const elFavoritesList = ref();

watch(connection, () => lastError.value = null);

function handleError(e) {
  console.error('handleError', e);
  lastError.value = String(e.error ?? e);
}

async function testConnection() {
  hideSnack();
  isConnecting.value = true;
  lastError.value = null
  try {
    const connector = new driver.value.connector({ ...connection.value });
    const rsp = await connector.test();
    makeHappySnack(rsp);
  } catch (e) {
    handleError(e);
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
    console.log('caught connect error', e);
    handleError(e);
  }
  isConnecting.value = false;
}

async function openSshKeyDialog() {
  isOpeningFile.value = true;
  const fileHandle = await open();
  connection.value.sshOpts.keyfile = fileHandle.path;
  isOpeningFile.value = false;
}

</script>

<template>
  <div class="connect-view d-flex">
    <div ref="elFavoritesList" class="v-sheet flex-grow-0" style="width:250px; min-width:250px;">
        <FavoritesList @connect="connect" />
    </div>

    <ResizeHandle :color="selectedColor" :target="elFavoritesList" :thickness="5" vertical />

    <div class="flex-grow-1 overflow-y-auto">
      <v-card max-width="700" class="mx-auto pa-5 my-5">
        <v-row>
          <v-col cols="2">
            <v-menu>
              <template v-slot:activator="{ props }">
                <v-btn :color="connection.color" v-bind="props" rounded>Color</v-btn>
              </template>
              <color-picker v-model="connection.color" />
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
        <template v-if="connection.driverName !== 'Sqlite'">
          <v-switch density="compact" v-model="connection.useSsh" label="SSH Tunnel"></v-switch>
          <div v-if="connection.useSsh">
            <v-row>
              <v-col cols="8">
                <v-text-field label="SSH Host" density="compact" v-model="connection.sshOpts.host" variant="outlined" />
              </v-col>
              <v-col cols="4">
                <v-text-field label="Port" density="compact" v-model="connection.sshOpts.port" variant="outlined"
                  type="number" />
              </v-col>
            </v-row>
            <v-text-field label="SSH User" density="compact" v-model="connection.sshOpts.user" variant="outlined" />
            <v-row>
              <v-col cols="6">
                <Password label="SSH Password" density="compact" v-model="connection.sshOpts.password"
                  variant="outlined" />
              </v-col>
              <v-col cols="6" class="pl-3">
                <v-btn prepend-icon="mdi-shield-key" @click="openSshKeyDialog" :disabled="isOpeningFile"
                  variant="outlined" color="grey">
                  Select SSH Key
                </v-btn>
                <v-btn class="mx-2" v-if="connection.sshOpts.keyfile !== null" @click="connection.sshOpts.keyfile = null" variant="outlined"
                  color="grey">
                  Clear
                </v-btn>
                <div><small v-if="connection.sshOpts.keyfile" v-text="connection.sshOpts.keyfile" /></div>
              </v-col>
            </v-row>
          </div>
        </template>

        <v-divider class="mb-5" />

        <component v-if="driver" :is="driver.fieldset" v-model="connection.driverOpts" />

        <v-alert class="mt-5" v-if="lastError" :text="lastError" type="error" />

        <v-divider class="my-5" />
        <v-card-actions class="d-flex">
          <v-btn class="mr-auto" rounded @click="store.saveFavorite(connection)" variant="outlined">Save</v-btn>

          <v-progress-circular v-if="isConnecting" indeterminate></v-progress-circular>

          <v-btn class="mr-2" rounded @click="testConnection" variant="outlined" :disabled="isConnecting">Test</v-btn>
          <v-btn rounded :color="selectedColor" type="submit" variant="elevated" @click="connect"
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