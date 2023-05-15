<script setup>
import { ref } from 'vue';
import ConnectView from './views/ConnectView.vue';
import QueryView from './views/QueryView.vue';
import Snacks from './components/Snacks.vue';

let connection = ref();

function connect($event) {
  console.log('received connect');
  connection.value = { ...$event }
}
function disconnect($event) {
  console.log('received disconnect');
  connection.value = null;
}

</script>

<template>
  <v-app>
    <Snacks />
    <ConnectView v-if="!connection" @connect="connect" />
    <QueryView v-else v-bind="{ connection }" @disconnect="disconnect" />
  </v-app>
</template>

<style scoped>
/* .logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
} */
</style>
