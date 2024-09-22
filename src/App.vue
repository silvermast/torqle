<script setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';
import ConnectView from './views/ConnectView.vue';
import ActionView from './views/ActionView.vue';
import Snacks from './components/Snacks.vue';

let connector = ref();

function connect($event) {
  console.log('received connect', $event);
  connector.value = $event;
  document.title = $event.title ?? `${$event.type} Connection`;
  getCurrentWindow().setTitle(document.title);
}
function disconnect($event) {
  console.log('received disconnect', $event);
  connector.value = null;
  getCurrentWindow().setTitle('New Connection');
}

</script>

<template>
  <v-app>
    <Snacks />
    <ConnectView v-if="!connector" @connect="connect" />
    <ActionView v-else v-bind="{ connector }" @disconnect="disconnect" />
  </v-app>
</template>

<style scoped>

</style>
