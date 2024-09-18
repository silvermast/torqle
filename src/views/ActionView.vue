<script setup>
import { ref, computed } from 'vue';
import { makeSpicySnack, makeHappySnack } from '~/components/Snacks.vue';
import QueryTab from '~/components/tabs/QueryTab.vue';
import IconButton from '~/components/IconButton.vue';
import ResizeHandle from '~/components/ResizeHandle.vue';
import { Connector } from '~/connectors/Connector.js';
import shortcuts from '~/services/KeyboardShortcuts.js';
import SchemaSidebar from '../components/SchemaSidebar.vue';
import { v4 as uuidv4 } from 'uuid';

const emit = defineEmits(['disconnect']);

const props = defineProps({
  connector: { type: Connector, required: true },
});

/**
 * @type {Connector}
 */
const connector = props.connector;

const color = computed(() => connector.color);
const selectedDatabase = ref(connector.getDatabase());
const tables = ref();
const databases = ref();
const isReconnecting = ref(false);

const maxTabs = 20;
let tabCounter = 0;

const tabs = ref([
  { id: uuidv4(), title: `Query ${++tabCounter}`, component: QueryTab },
]);
const selectedTab = ref(tabs.value[0].id);

async function selectDatabase(database) {
  await connector.setDatabase(database);
  await loadTables();
}
async function selectTable(table) {
  console.log('selectTable', table);
  // @TODO New tab for table inspection
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
    makeSpicySnack(e);
    emit('disconnect');
    isReconnecting.value = false;
  }
  isReconnecting.value = false;
}

function newQueryTab() {
  const id = uuidv4();
  tabs.value.push({ id, title: `Query ${++tabCounter}`, component: QueryTab });
  selectedTab.value = id;
}

function closeTab(tab) {
  const tabIndex = tabs.value.indexOf(tab);
  if (tabIndex !== -1) {
    tabs.value.splice(tabIndex, 1);
    const nestNearestTab = tabs.value[tabIndex] ?? tabs.value[tabIndex - 1] ?? tabs.value[0];
    selectedTab.value = nestNearestTab?.id;
  }
  if (tabs.value.length === 0) {
    newQueryTab();
  }
}

function selectNextTab() {
  const tabIndex = tabs.value.findIndex(tab => tab.id === selectedTab.value) + 1;
  selectedTab.value = tabs.value[tabIndex]?.id || tabs.value[0].id;
}
function selectPrevTab() {
  const tabIndex = tabs.value.findIndex(tab => tab.id === selectedTab.value) - 1;
  selectedTab.value = tabs.value[tabIndex]?.id || tabs.value[tabs.value.length - 1].id;
}

/**
 * Page Initialization
 */
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

    <section id="view--tab-container">
      <v-btn-toggle v-model="selectedTab" :color="color" rounded="0" density="compact" :max="maxTabs" group mandatory>
        <v-btn v-for="tab in tabs" :value="tab.id">
          {{ tab.title }}
          <template v-slot:append>
            <IconButton variant="text" icon="mdi-close" @click.stop="closeTab(tab)" />
          </template>
        </v-btn>

        <IconButton variant="text" height="100%" class="ml-2 px-5" title="New Query Tab" icon="mdi-plus" @click="newQueryTab" :disabled="tabs.length === maxTabs" />
      </v-btn-toggle>

      <div class="view--tabs-window">
        <component class="view--tabs-window-item" v-for="tab in tabs" :is="tab.component" v-bind="{ connector }" :key="tab.id" v-show="tab.id === selectedTab" />
      </div>
    </section>

  </main>
</template>

<style lang="scss" scoped>
$actionHeight: 32px;
$tabsHeight: 32px;

nav#vertical-nav {
  width: 48px;
  min-width: 48px;
  max-width: 48px;
  background-color: rgba(var(--v-theme-primary));
}

main {
  height: 100vh;
  max-height: 100vh;
  min-height: 100vh;
  width: 100vw;
  max-width: 100vw;
  min-width: 100vw;
  flex-basis: 100vw;
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;

  section#view--tab-container {
    width: auto;
    flex-grow: 1;
    overflow: hidden;
  }
}

.view--tabs-window {
  height: calc(100vh - $tabsHeight);
  max-height: calc(100vh - $tabsHeight);
  min-height: calc(100vh - $tabsHeight);
  .view--tabs-window-item {
    height: 100%;
    max-height: 100%;
    min-height: 100%;
  }
}
</style>
