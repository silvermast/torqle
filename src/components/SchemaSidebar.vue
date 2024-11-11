<script setup>
import { computed, ref, onBeforeUnmount } from 'vue';
import { shortcuts } from '../services/KeyboardShortcuts.js';

const props = defineProps({
    selectedDatabase: { type: String, default: null },
    databases: { type: Array, default: [] },
    tables: { type: Array, default: [] }
});

const emit = defineEmits(['reloadSchemas', 'selectTable', 'selectDatabase'])

const tableFilter = ref();
const elDatabaseSelector = ref();
const elTableFilter = ref();

const selectedDatabaseComputed = computed({
    get: () => props.selectedDatabase,
    set: (value) => emit('selectDatabase', value),
});

function matchesTableFilter(value) {
    if (!tableFilter?.value?.length) {
        return true;
    }
    return new RegExp(tableFilter?.value, 'i').test(value);
}

shortcuts.selectDatabase.register(() => {
    elDatabaseSelector.value?.focus();
});
shortcuts.filterTables.register(() => {
    elTableFilter.value?.focus();
});

onBeforeUnmount(() => {
    shortcuts.selectDatabase.unregister();
    shortcuts.filterTables.unregister();
});
</script>

<template>
    <div class="sidebar-container">
        <v-autocomplete class="mx-2 mt-2" ref="elDatabaseSelector" v-model="selectedDatabaseComputed"
            :items="props.databases" item-title="Database" hide-details variant="solo" rounded density="compact"
            label="Select Database" no-data-text="No databases found" single-line :disabled="!props.databases?.length" auto-select-first>
            <template v-slot:append>
                <v-btn @click="emit('reloadSchemas')" class="mr-1 ml-0" size="x-small" variant="tonal"
                    icon="mdi-refresh" title="Refresh Database List" rounded />
            </template>
            <template v-slot:item="{ props, item }">
                <v-list-item v-bind="props" :title="item.raw" class="autocomplete-item"></v-list-item>
            </template>
        </v-autocomplete>
        <v-text-field class="mx-2 my-2" ref="elTableFilter" variant="solo" density="compact" label="Filter Tables"
            clearable hide-details single-line v-model="tableFilter" rounded />
        <v-list class="table-list">
            <v-list-item class="li-table" density="compact" v-for="table in props.tables"
                @click="emit('selectTable', table)" v-show="matchesTableFilter(table)" v-text="table"></v-list-item>
        </v-list>
    </div>
</template>

<style scoped lang="scss">
:deep(.v-autocomplete__content) {
    border: 2px solid var(--connection-color);
}

.sidebar-container {
    width: inherit;
    position: relative;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;

    >* {
        flex-grow: 0;
    }

    :deep(.autocomplete-item) {
        font-size: 0.8em;
        height: 28px;
        min-height: 16px;
        padding: 4px 16px;
    }

    .table-list {
        flex-grow: 1;
        overflow-y: auto;
        overflow-x: clip;

        .li-table {
            font-size: 0.8em;
            height: 28px;
            min-height: 16px;
            padding: 4px 16px;
        }
    }
}
</style>