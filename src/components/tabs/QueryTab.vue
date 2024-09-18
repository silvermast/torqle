<script setup>
import { ref } from 'vue';
import { Connector } from '~/connectors/Connector.js';
import QueryEditor from '~/components/QueryEditor.vue';
import QueryResults from '~/components/QueryResults.vue';
import QueryStats from '~/components/QueryStats.vue';
import ResizeHandle from '~/components/ResizeHandle.vue';

const queryText = ref('-- Run a query!');
const isQuerying = ref(false);
const queryResult = ref();
const queryError = ref();
const elEditor = ref();

const { connector } = defineProps({
  connector: { type: Connector, required: true },
});

const color = connector.color

async function runQuery() {
  isQuerying.value = true;
  queryError.value = null;

  try {
    queryResult.value = await connector.query(queryText.value);
    console.log(queryResult.value);
  } catch (e) {
    console.warn(e);
    queryError.value = (e.error ?? e).toString();
  }

  isQuerying.value = false;
}

</script>

<template>
    <section class="tab--content">
      <div class="tab--editor" style="height:320px; min-height:320px;" ref="elEditor">
        <QueryEditor v-model="queryText" @run-selected="runQuery" />
      </div>

      <ResizeHandle v-bind="{ color }" :target="elEditor" :thickness="5" horizontal />

      <div class="tab--actions d-flex flex-row align-center">
        <v-btn v-bind="{ color }" size="x-small" variant="elevated" rounded class="ml-auto mr-1" @click="runQuery"
          :disabled="isQuerying || !queryText">Run Query</v-btn>
      </div>

      <div class="tab--results">
        <QueryResults v-bind="{ isQuerying, queryResult, queryError, color }" />
      </div>

      <div class="tab--stats d-flex align-center">
        <QueryStats v-bind="queryResult" v-if="queryResult" />
      </div>
    </section>
</template>

<style lang="scss" scoped>
$actionHeight: 32px;

section.tab--content {
    width: 100%;
    height: 100%;
    max-height: 100%;
    min-height: 100%;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    flex-basis: 100%;
    overflow: hidden;

    > * {
      flex-grow: 0;
    }

    .tab--editor {
        overflow: auto;
        height: calc(100% - $actionHeight);
    }

    .tab--actions {
        border-top: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
        height: $actionHeight;
        min-height: $actionHeight;
    }

    .tab--results {
        position: relative;
        overflow: auto;
        margin-bottom: $actionHeight;
        min-width: 100%;
        flex-grow: 1;
    }

    .tab--stats {
        position: absolute;
        bottom: 0;
        height: $actionHeight;
        min-height: $actionHeight;
        max-height: $actionHeight;
        width: 100%;
        border-top: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
    }
}
</style>