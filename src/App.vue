<script setup>
import { ref } from 'vue';
import ConnectView from './views/ConnectView.vue';
import QueryView from './views/QueryView.vue';
import Snacks from './components/Snacks.vue';

let connector = ref();

function connect($event) {
  console.log('received connect');
  connector.value = $event;
}
function disconnect($event) {
  console.log('received disconnect');
  connector.value = null;
}

const defaults = ref({
  global: {
    density: 'compact',
  },
  VRow: { dense: true },
  VBtn: { size: 'small' },
})

</script>

<template>
  <v-defaults-provider :defaults="vuetifyDefaults">
    <v-app>
      <Snacks />
      <ConnectView v-if="!connector" @connect="connect" />
      <QueryView v-else v-bind="{ connector }" @disconnect="disconnect" />
    </v-app>
  </v-defaults-provider>
</template>

<style scoped>
/* .logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
} */
</style>
