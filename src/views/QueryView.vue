<script setup>
import { ref, computed, watch } from 'vue';
import { makeSpicySnack, makeHappySnack } from '~/components/Snacks.vue';
import SqlEditor from '~/components/SqlEditor.vue';
// import CodeMirror from 'vue-codemirror6';
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

const selectedSchema = ref(connector.getSchema());
const tableList = ref();
const schemaList = ref();
const tableFilter = ref();
const queryText = ref('SELECT * FROM FOO WHERE BLAH;');
const results = ref();
const resultsCount = ref();
const resultsTime = ref();
const isQuerying = ref(false);

const showResultsCount = computed(() => resultsCount.value !== undefined);
const showResultsTime = computed(() => resultsTime.value !== undefined);

watch(selectedSchema, async (newValue) => {
  await connector.changeSchema(newValue);
  loadTables();
});

watch(queryText, () => console.log('queryText:', queryText.value));

async function loadSchemas() {
  try {
    schemaList.value = await connector.loadSchemas();
  } catch (e) {
    makeSpicySnack(e);
  }
}

async function loadTables() {
  if (!selectedSchema.value) {
    return;
  }
  try {
    tableList.value = await connector.loadTables();
  } catch (e) {
    makeSpicySnack(e);
  }
}

async function runQuery() {
  isQuerying.value = true;

  try {
    const queryResult = await connector.query(queryText.value);
    results.value = queryResult.rows;
    resultsCount.value = queryResult.num_rows;
    resultsTime.value = queryResult.elapsed_ms;
  } catch (e) {
    makeSpicySnack(e);
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

const filteredTableList = computed(() => {
  const regex = new RegExp(tableFilter.value);
  return (tableList.value ?? []).filter(v => regex.test(v));
});

function matchesTableFilter(value) {
  return new RegExp(tableFilter.value, 'i').test(value);
}

/**
 * Resizing functionality
 */

const elSidebar = ref();
const elEditor = ref();
const actionHeight = 32;
const appBarHeight = 48;

function startResize($event) {
  $event.preventDefault();
  $event.stopPropagation();
  const isSidebar = $event.target.id === 'sidebar-resize-handle';
  const resizeTarget = isSidebar ? elSidebar : elEditor;
  const resizeMetric = isSidebar ? 'pageX' : 'clientY';
  const resizeStyle = isSidebar ? 'width' : 'height';
  const maxValue = isSidebar ? (document.body.offsetWidth) : (document.body.offsetHeight - actionHeight);
  const minValue = 0;
  const offset = isSidebar ? 0 : appBarHeight;

  function resize(e) {
    if (e[resizeMetric] > minValue && e[resizeMetric] < maxValue) {
      const newValue = e[resizeMetric] - offset;
      resizeTarget.value.style[resizeStyle] = `${newValue}px`;
      resizeTarget.value.style[`min-${resizeStyle}`] = `${newValue}px`;
    }
  }

  function stopResize(e) {
    document.removeEventListener('mousemove', resize); // stop resizing
    $event.target.removeEventListener('mouseup', stopResize); // cleanup
    document.removeEventListener('mouseup', stopResize); // cleanup
  }

  document.addEventListener('mousemove', resize);
  $event.target.addEventListener('mouseup', stopResize)
  document.addEventListener('mouseup', stopResize);
}

function debug($event) {
  console.log($event);
}

loadSchemas();
loadTables();

</script>

<template>
  <v-app-bar density="compact" v-bind="{ color }">
    <div class="schema-selector-container ml-1">
      <v-select density="compact" v-model="selectedSchema" :items="schemaList" item-title="Schema" hide-details
        variant="outlined" label="Select Schema" no-data-text="No schemas found" single-line />
    </div>
    <v-btn size="x-small" variant="elevated" rounded class="ml-auto mr-1" @click="disconnect">Disconnect</v-btn>
  </v-app-bar>
  <main>
    <section id="view--sidebar" style="width:256px; min-width:256px;" ref="elSidebar">
      <v-text-field density="compact" label="Filter Tables" clearable hide-details single-line v-model="tableFilter"></v-text-field>
      <v-list id="table-list">
        <v-list-item class="li-table" density="compact" v-for="table in tableList" @click="debug"
          v-show="matchesTableFilter(table)"
          v-text="table"></v-list-item>
      </v-list>
    </section>

    <div id="sidebar-resize-handle" @mousedown="startResize" :style="{ background: color }"></div>

    <section id="view--content">
      <div id="view--editor" style="height:320px; min-height:320px;" ref="elEditor">
        <SqlEditor v-model="queryText" @run-selected="runQuery" />
      </div>

      <hr id="editor-resize-handle" @mousedown="startResize" :style="{ background: color }" />

      <div id="view--actions" class="d-flex flex-row align-center">
        <v-btn v-bind="{ color }" size="x-small" variant="elevated" rounded class="ml-auto mr-1" @click="runQuery"
          :disabled="isQuerying">Run Query</v-btn>
      </div>

      <div id="view--results">
        <v-overlay v-model="isQuerying" contained>
          <v-icon icon="loading" size="x-large" />
        </v-overlay>
        <table v-if="results">
          <thead>
            <tr><th v-for="value, field in results[0]" v-text="field" /></tr>
          </thead>
          <tbody>
            <tr v-for="row in results"><td v-for="value in row" v-text="value" /></tr>
          </tbody>
        </table>
      </div>

      <div id="view--stats">
        <v-chip density="compact" variant="plain" v-if="showResultsCount">Rows: {{ resultsCount }}</v-chip>
        <v-chip density="compact" variant="plain" v-if="showResultsTime">Time: {{ resultsTime }}ms</v-chip>
      </div>
    </section>

  </main>
</template>

<style lang="scss" scoped>
$navHeight: 48px;
$actionHeight: 32px;
$handleThickness: 6px;

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

    #table-list {
      height: calc(100% - 42px);
      overflow-y: auto;
      overflow-x: clip;
    }

    .li-table {
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
    }

    #view--results {
      position: relative;
      overflow: auto;

      table {
        border-spacing: 0;

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
      height: $actionHeight;
      border-top: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
    }
  }
}

.schema-selector-container {
  max-width: 200px;
  width: 200px;
}

#editor-resize-handle {
  box-sizing: border-box;
  height: $handleThickness;
  min-height: $handleThickness;
  border: none;
  background-color: rgba(var(--v-theme-primary));
  cursor: row-resize;
}

#sidebar-resize-handle {
  height: 100%;
  width: $handleThickness;
  min-width: $handleThickness;
  box-sizing: border-box;
  border: none;
  background-color: rgba(var(--v-theme-primary));
  cursor: col-resize;
  // cursor: ew-resize;
}
</style>
