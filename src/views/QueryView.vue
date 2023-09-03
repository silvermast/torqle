<script setup>
import { ref, computed, watch, onMounted } from 'vue';
import { makeSpicySnack, makeHappySnack } from '~/components/Snacks.vue';
import QueryEditor from '~/components/QueryEditor.vue';
import QueryWait from '~/components/QueryWait.vue';
import IconButton from '~/components/IconButton.vue';
import ResizeHandle from '~/components/ResizeHandle.vue';
import { Connector } from '~/services/Connector.js';

const emit = defineEmits(['disconnect']);

const props = defineProps({
  connector: { type: Connector, required: true },
  editorLang: { type: String, default: 'sql' },
});

const color = computed(() => props.connector?.opts?.color ?? 'rgba(var(--v-theme-primary))');

/**
 * @type {Connector}
 */
const connector = props.connector;

const selectedDatabase = ref(connector.getDatabase());
const tableList = ref();
const databaseList = ref();
const tableFilter = ref();
const queryText = ref('-- Run a query!');
const queryResult = ref();
const isQuerying = ref(false);
const queryError = ref();

const showResultsCount = computed(() => queryResult.value?.num_rows !== undefined);
const showResultsTime = computed(() => queryResult.value?.elapsed_ms !== undefined);

watch(selectedDatabase, async (newValue) => {
  await connector.setDatabase(newValue);
  loadTables();
});

watch(queryText, () => console.log('queryText:', queryText.value));

async function loadDatabases() {
  try {
    databaseList.value = await connector.loadDatabases();
  } catch (e) {
    makeSpicySnack(e);
  }
}

async function loadTables() {
  if (!selectedDatabase.value) {
    return;
  }
  try {
    tableList.value = await connector.loadTables();
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
    // results.value = queryResult.rows;
    // resultsCount.value = queryResult.num_rows;
    // resultsTime.value = queryResult.elapsed_ms;
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

function matchesTableFilter(value) {
  return new RegExp(tableFilter.value, 'i').test(value);
}

const elSidebar = ref();
const elEditor = ref();
const elDatabaseSelector = ref();
const elTableFilter = ref();

function debug($event) {
  console.log($event);
}

function keypress(e) {
  const cmdKey = /Mac/.test(window.navigator.userAgent) ? 'metaKey' : 'ctrlKey'
  if (e.code === 'KeyJ' && e[cmdKey]) {
    elDatabaseSelector.value.focus();
  } else if (e.code === 'keyK' && e[cmdKey]) {
    elTableFilter.value.focus();
  }
}

loadDatabases();
loadTables();

</script>

<template>
  <main @keypress="keypress">
    <nav id="vertical-nav" class="d-flex flex-column align-center" :style="{ background: color }">
      <IconButton @click="debug" class="mt-2" icon="mdi-table" title="Table List" />
      
      <!-- Bottom actions -->
      <div class="d-flex flex-column mt-auto mb-1 align-center">
        <IconButton @click="debug" class="mb-2" title="Reconnect" icon="mdi-cached" />
        <IconButton @click="disconnect" class="mb-2 icon-flip-h" title="Disconnect" icon="mdi-logout" />
      </div>
    </nav>

    <section id="view--sidebar" style="width:256px; min-width:256px;" ref="elSidebar">
      <v-autocomplete ref="elDatabaseSelector" v-model="selectedDatabase" :items="databaseList" item-title="Database" hide-details
        variant="solo" rounded label="Select Database" no-data-text="No databases found" single-line
      >
        <template v-slot:append>
          <v-btn @click="reloadTablesAndDatabases" class="mr-1 ml-0" size="small" variant="tonal" icon="mdi-refresh" title="Refresh Database List" rounded />
        </template>
      </v-autocomplete>
      <v-text-field ref="elTableFilter" variant="solo" density="compact" label="Filter Tables" clearable hide-details single-line v-model="tableFilter" rounded />
      <v-list id="table-list">
        <v-list-item class="li-table" density="compact" v-for="table in tableList" @click="debug"
          v-show="matchesTableFilter(table)"
          v-text="table"></v-list-item>
      </v-list>
    </section>

    <ResizeHandle :color="color" :target="elSidebar" :thickness="5" vertical />

    <section id="view--content">
      <div id="view--editor" style="height:320px; min-height:320px;" ref="elEditor">
        <QueryEditor v-model="queryText" @run-selected="runQuery" />
      </div>

      <ResizeHandle :color="color" :target="elEditor" :thickness="5" horizontal />

      <div id="view--actions" class="d-flex flex-row align-center">
        <v-btn v-bind="{ color }" size="x-small" variant="elevated" rounded class="ml-auto mr-1" @click="runQuery"
          :disabled="isQuerying">Run Query</v-btn>
      </div>

      <div id="view--results">
        <QueryWait :show="isQuerying" />

        <v-alert v-if="queryError" :text="queryError" type="error" class="ma-5" />

        <table v-if="queryResult && !isQuerying">
          <thead>
            <tr><th v-for="field in queryResult.fields" v-text="field" width="150" /></tr>
          </thead>
          <tbody>
            <tr v-for="row in queryResult.rows"><td v-for="field in queryResult.fields" v-text="row[field]" /></tr>
          </tbody>
        </table>
      </div>

      <div id="view--stats">
        <v-chip density="compact" variant="plain" v-if="showResultsCount">Rows: {{ queryResult.num_rows }}</v-chip>
        <v-chip density="compact" variant="plain" v-if="showResultsTime">Time: {{ queryResult.elapsed_ms }}ms</v-chip>
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

  section#view--sidebar {
    width: inherit;
    position: relative;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;

    > * {
      flex-grow: 0;
    }

    #table-list {
      flex-grow: 1;
      overflow-y: auto;
      overflow-x: clip;
    }

    .li-table {
      font-size: 0.8em;
      height: 28px;
      min-height: 16px;
      padding: 4px 16px;
    }

  }

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

      table {
        border-spacing: 0;
        min-width: 100%;

        th {
          height: 2em;
          position: -webkit-sticky;
          /* for Safari */
          position: sticky;
          top: 0;
          background: rgb(var(--v-theme-background));
          text-align: left;
        }
        th:not(:first-child) {
          border-left: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
        }

        th,
        td {
          font-size: 0.8em;
          padding: 2px 4px;
          border-bottom: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
          overflow-x: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
        }
      }
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
