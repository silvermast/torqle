<script setup>
import { ref, computed, watch, onMounted, onUnmounted, handleError } from 'vue';
import { makeSpicySnack, makeHappySnack } from '~/components/Snacks.vue';
import QueryTab from '~/components/tabs/QueryTab.vue';
import IconButton from '~/components/IconButton.vue';
import ResizeHandle from '~/components/ResizeHandle.vue';
import { Connector } from '~/connectors/Connector.js';
import shortcuts from '~/services/KeyboardShortcuts.js';
import SchemaSidebar from '../components/SchemaSidebar.vue';

const emit = defineEmits(['disconnect']);

const props = defineProps({
  connector: { type: Connector, required: true },
});


/**
 * @type {Connector}
 */
const connector = props.connector;

const selectedDatabase = ref(connector.getDatabase());
const tables = ref();
const databases = ref();
const isReconnecting = ref(false);

const tabs = [
  { type: 'QUERY_EDITOR', props: { color, connector } },
];

async function selectDatabase(database) {
  await connector.setDatabase(database);
  await loadTables();
}
async function selectTable(table) {
  console.log('selectTable', table);
  // NEW TAB: table inspection
}

async function loadDatabases() {
  try {
    databases.value = await connector.loadDatabases();
    console.log('Databases', databases.value);
  } catch (e) {
    makeSpicySnack(e);
  }
}

async function loadTables() {
  tables.value = [];
  try {
    tables.value = await connector.loadTables();
    console.log('Tables', tables.value);
  } catch (e) {
    makeSpicySnack(e);
  }
}

async function disconnect() {
  try {
    await connector.disconnect();
    makeHappySnack('Successfully disconnected.');
  } catch (e) {
    makeSpicySnack(e);
    console.error(e);
  }
  emit('disconnect');
}

const elSidebar = ref();

function debug($event) {
  console.log($event);
}

async function reconnect() {
  isReconnecting.value = true;
  try {
    await connector.reconnect();
  } catch (e) {
    handleError(e);
    emit('disconnect');
    isReconnecting.value = false;
  }
  isReconnecting.value = false;
}

/**
 * Page Initialization
 */
const color = computed(() => connector.color);
document.documentElement.style.setProperty('--connection-color', `${color}60`);

loadDatabases();
loadTables();

</script>

<template>
  <main>
    <v-overlay :model-value="isReconnecting" class="align-center justify-center">
      <v-progress-circular v-bind="{ color }" size="64" indeterminate />
    </v-overlay>

    <nav id="vertical-nav" class="d-flex flex-column align-center" :style="{ background: color }">
      <IconButton @click="debug" class="mt-2" icon="mdi-table" title="Table List" />

      <!-- Bottom actions -->
      <div class="d-flex flex-column mt-auto mb-1 align-center">
        <IconButton @click="reconnect" class="mb-2" title="Reconnect" icon="mdi-cached" />
        <IconButton @click="disconnect" class="mb-2 icon-flip-h" title="Disconnect" icon="mdi-logout" />
      </div>
    </nav>

    <section id="view--sidebar" style="width:256px; min-width:256px;" ref="elSidebar">
      <SchemaSidebar v-bind="{ selectedDatabase, databases, tables }" v-on="{ selectTable, selectDatabase }" />
    </section>

    <ResizeHandle :color="color" :target="elSidebar" :thickness="5" vertical />

    <section id="view--tab-group">

    </section>
    <QueryTab v-bind="{ connector }" />

  </main>
</template>

<style lang="scss" scoped>
$actionHeight: 32px;
$navHeight: 0px;

nav#vertical-nav {
  width: 48px;
  min-width: 48px;
  max-width: 48px;
  background-color: rgba(var(--v-theme-primary));
}

main {
  margin-top: $navHeight;
  height: calc(100vh - $navHeight);
  max-height: calc(100vh - $navHeight);
  min-height: calc(100vh - $navHeight);
  width: 100vw;
  max-width: 100vw;
  min-width: 100vw;
  flex-basis: 100vw;
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;

  section#view--tab-group {
    width: inherit;
    height: 100%;
    max-height: 100%;
    min-height: 100%;
    flex-grow: 1;
    overflow: hidden;
  }
}
</style>
