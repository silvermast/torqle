<script setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';
import ConnectView from './views/ConnectView.vue';
import ActionView from './views/ActionView.vue';
import Snacks from './components/Snacks.vue';

let connector = ref();

const defaultTitle = 'New Connection';

function connect($connector) {
  console.log('received connect', $connector);
  connector.value = $connector;
  getCurrentWindow().setTitle($connector.name ?? `${$connector.type} Connection`);
}
function disconnect($event) {
  console.log('received disconnect', $event);
  connector.value = null;
  getCurrentWindow().setTitle(defaultTitle);
}

getCurrentWindow().setTitle(defaultTitle);

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
