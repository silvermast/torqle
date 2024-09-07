<script setup>
import { ref, computed, watch, onMounted, onUnmounted, handleError } from 'vue';
import { makeSpicySnack, makeHappySnack } from '~/components/Snacks.vue';
import QueryEditor from '~/components/QueryEditor.vue';
import QueryResults from '../components/QueryResults.vue';
import QueryStats from '../components/QueryStats.vue';
import IconButton from '~/components/IconButton.vue';
import ResizeHandle from '~/components/ResizeHandle.vue';
import { Connector } from '~/connectors/Connector.js';
import shortcuts from '~/services/KeyboardShortcuts.js';
import SchemaSidebar from '../components/SchemaSidebar.vue';

const emit = defineEmits(['disconnect']);

const props = defineProps({
  connector: { type: Connector, required: true },
  editorLang: { type: String, default: 'sql' },
});

const color = computed(() => props.connector?.opts?.color);
document.documentElement.style.setProperty('--connection-color', `${color.value}60`);

/**
 * @type {Connector}
 */
const connector = props.connector;

const selectedDatabase = ref(connector.getDatabase());
const tables = ref();
const databases = ref();
const queryText = ref('-- Run a query!');
const isQuerying = ref(false);
const queryResult = ref();
const queryError = ref();
const isReconnecting = ref(false);

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
  try {
    tables.value = await connector.loadTables();
    console.log('Tables', tables.value);
  } catch (e) {
    makeSpicySnack(e);
  }
}

async function reloadTablesAndDatabases() {
  await Promise.all([loadDatabases, loadTables]);
}

async function runQuery() {
  isQuerying.value = true;
  queryError.value = null;

  try {
    queryResult.value = await connector.query(queryText.value, selectedDatabase.value);
    console.log(queryResult.value);
  } catch (e) {
    console.warn(e);
    queryError.value = (e.error ?? e).toString();
  }

  isQuerying.value = false;
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
const elEditor = ref();

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

    <section id="view--content">
      <div id="view--editor" style="height:320px; min-height:320px;" ref="elEditor">
        <QueryEditor v-model="queryText" @run-selected="runQuery" />
      </div>

      <ResizeHandle :color="color" :target="elEditor" :thickness="5" horizontal />

      <div id="view--actions" class="d-flex flex-row align-center">
        <v-btn v-bind="{ color }" size="x-small" variant="elevated" rounded class="ml-auto mr-1" @click="runQuery"
          :disabled="isQuerying || !queryText">Run Query</v-btn>
      </div>

      <div id="view--results">
        <QueryResults v-bind="{ isQuerying, queryResult, queryError, color }" />
      </div>

      <div id="view--stats" class="d-flex align-center">
        <QueryStats v-bind="queryResult" v-if="queryResult" />
      </div>
    </section>

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

  section#view--content {
    width: inherit;
    height: 100%;
    max-height: 100%;
    min-height: 100%;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    flex-basis: 100%;
    overflow: hidden;

    #view--editor {
      overflow: auto;
      height: calc(100% - $actionHeight);
    }

    #view--actions {
      border-top: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
      height: $actionHeight;
      min-height: $actionHeight;
    }

    #view--results {
      position: relative;
      overflow: auto;
      margin-bottom: $actionHeight;
      min-width: 100%;
    }

    #view--stats {
      position: absolute;
      bottom: 0;
      height: $actionHeight;
      min-height: $actionHeight;
      width: 100%;
      border-top: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
    }
  }
}
</style>
