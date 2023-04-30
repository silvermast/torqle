<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from 'vue';
import { Codemirror } from 'vue-codemirror';
import VueResizable from 'vue-resizable';

const emit = defineEmits('disconnect');

const props = defineProps({
  connection: { type: Object, required: true },
  editorLang: { type: String, default: 'sql' },
});

const queryText = ref('');
const results = ref(null);
const isQuerying = ref(false);

const resultsHeight = ref(100);
const editorHeight = ref(256);

function resize() {
  const appBarHeight = 24;
  const actionsHeight = 36;
  const resultsInfoHeight = 24;
  const viewHeight = Math.floor(window.outerHeight - appBarHeight);
  // tableHeight.value = Math.floor(viewHeight - editorHeight.value - actionsHeight - resultsInfoHeight);
}

async function runQuery() {
  isQuerying.value = true;
  setTimeout(() => {
    isQuerying.value = false;
    results.value = new Array(1000).fill(0).map(() => ({
      id: Math.random(),
      firstName: Math.random(),
      last_name: Math.random(),
      status: Math.random(),
      email: Math.random(),
      password: Math.random(),
      foo: Math.random(),
      blahsadf: Math.random(),
      afje0: Math.random(),
      Fjveiagae: Math.random(),
      date: Math.random(),
      dateUpdated: Math.random(),
      dateCreated: Math.random(),
      deleted: Math.random(),
      otherStuff: Math.random(),
      someReallyLogFieldName: Math.random(),
      active: Math.random(),
    }));
  }, 100)
}

// onMounted(() => {
//   // resize();
// });

runQuery();

</script>

<template>
  <main>
    <nav>
      <v-text-field label="Search"></v-text-field>
      <ul>
        <li v-for="i, k in Array(200)">table {{ k }}</li>
      </ul>
      <v-btn class="ml-auto" size="x-small" variant="elevated" rounded color="error" @click="emit('disconnect')">Disconnect</v-btn>
    </nav>

    <section>

      <div id="view--editor">
        <Codemirror
          v-model="queryText"
          placeholder="SELECT ..."
          :style="{ height: '100%', width: '100%' }"
          :autofocus="true"
          :indent-with-tab="true"
          :tab-size="2"
          :smart-indent="true"
        />
      </div>

      <div id="view--actions" class="d-flex flex-row align-center">
        <v-btn size="x-small" variant="elevated" rounded color="primary" class="ml-auto mr-1" @click="runQuery" :disabled="isQuerying">Run Query</v-btn>
      </div>
      
      <div id="view--results">
        <table v-if="results">
          <thead>
            <tr><th v-for="value, field in results[0]" v-text="field"></th></tr>
          </thead>
          <tbody>
            <tr v-for="row in results">
              <td v-for="value in row" v-text="value"></td>
            </tr>
          </tbody>
        </table>
      </div>

      <div id="view--stats">1234 results in 23ms</div>
    </section>

  </main>
</template>

<style lang="scss" scoped>
  $navWidth: 256px;
  $actionHeight: 32px;
  $thHeight: 32px;

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

    nav {
      width: $navWidth;
      min-width: $navWidth;
      max-width: $navWidth;
      overflow-y: auto;
    }
    section {
      width: calc(100vw - $navWidth);
      height: 100vh;
      max-height: 100vh;
      min-height: 100vh;
      flex-grow: 1;
      display: flex;
      flex-direction: column;
      flex-wrap: nowrap;
      flex-basis: 100vh;

      #view--editor {
        overflow: auto;
        height: calc(50% - $actionHeight);
      }
      #view--actions {
        height: $actionHeight;
      }
      #view--results {
        position: relative;
        overflow: auto;
        max-height: calc(50% - $actionHeight);
        table {
          border-spacing: 0;
          th {
            height: $thHeight;
            position: -webkit-sticky; /* for Safari */
            position: sticky;
            top: 0;
            background: rgb(var(--v-theme-background));
            text-align: left;
          }
          th, td {
            padding: 2px 4px;
            border-bottom: thin solid rgba(var(--v-border-color),var(--v-border-opacity));
          }
        }
      }
      #view--stats {
        height: $actionHeight;
      }
    }
  }

  // #view--results table th {
  //   position: sticky;
  // }
</style>