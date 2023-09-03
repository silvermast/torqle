<script setup>
import { computed, watch, ref } from 'vue';
const props = defineProps(['show']);
const runningElapsed = ref();

let startTime = null;
let interval = null;
watch(() => props.show, () => {
    if (props.show) {
        startTime = Date.now();
        interval = setInterval(() => runningElapsed.value = (Date.now() - startTime), 100);
    } else {
        clearInterval(interval);
    }
});
</script>

<template>
    <div class="query-wait" v-if="props.show">
        <div>Executing Query...</div>
        <div>Elapsed: {{ runningElapsed }}ms</div>
    </div>
</template>

<style scoped>
.query-wait {
  margin: 6em;
  text-align: center;
}
</style>