<script setup>
import { ref, computed } from 'vue';
import QueryResult from '~/services/QueryResult.js';
import QueryWait from '~/components/QueryWait.vue';
import DataTable from './DataTable.vue';

const { isQuerying, queryError, queryResult, color } = defineProps({
  isQuerying: { type: Boolean, default: false },
  queryError: { type: String, default: null },
  queryResult: { type: QueryResult, default: null },
  color: { type: String, default: null },
});

const dialogText = ref();

const showDialog = computed(() => !!dialogText.value);
const noResultsFound = computed(() => String(queryResult?.value?.numRows) === '0');

</script>

<template>
  <div>
    <QueryWait :show="isQuerying" />

    <template v-if="!isQuerying">
      <v-alert v-if="queryError" :text="queryError" type="error" class="ma-5" />
      <v-alert v-else-if="noResultsFound" class="ma-5" v-bind="{ color }" text="No Results" variant="outlined" />

      <DataTable v-else-if="queryResult" v-bind="queryResult" />

      <!-- <v-dialog v-model="showDialog" scrollable width="auto" rounded>
        <v-card>
          <v-card-title class="d-flex justify-end">
            <v-icon icon="mdi-close" title="Close" @click="dialogText = null" />
          </v-card-title>
          <v-card-text>{{ dialogText }}</v-card-text>
        </v-card>
      </v-dialog> -->

    </template>
  </div>
</template>

<style lang="scss">

</style>
