<script setup>
const props = defineProps({
    rows: { type: Array, default: [] },
    fields: { type: Array, default: [] },
});

/**
 * @todo: implement framing for vertical scrolling to help reduce memory footprint
 */

</script>

<template>
    <table class="data-table">
        <thead>
            <tr>
                <th v-for="field in fields" width="150">
                    {{ field }}
                </th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="row in rows">
                <td class="data-table-cell" v-for="field in fields" v-text="row[field]" />
            </tr>
        </tbody>
    </table>
</template>

<style scoped lang="scss">
table.data-table {
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
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
        max-width: 200px;
    }

    .data-table-cell {
        cursor: pointer;
    }
}
</style>