<script setup>
import { isNull } from '~/helpers.js';

const props = defineProps({
  numRows: { type: Number, default: null },
  elapsedMs: { type: Number, default: null },
});

</script>

<template>
  <div>
    <v-chip density="compact" variant="plain" v-if="!isNull(props.numRows)">
      Rows: {{ props.numRows }}
    </v-chip>
    <v-chip density="compact" variant="plain" v-if="!isNull(props.elapsedMs)">
      Query Time: {{ props.elapsedMs.toFixed(2) }}ms
    </v-chip>
  </div>
</template>