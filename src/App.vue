<script setup>
import { ref, onMounted } from 'vue';
import ConnectView from './views/ConnectView.vue';
import QueryView from './views/QueryView.vue';
import Snacks from './components/Snacks.vue';

let connector = ref();

function connect($event) {
  console.log('received connect');
  connector.value = $event;
  document.title = connector.value.title ?? 'Connection';
}
function disconnect($event) {
  console.log('received disconnect');
  connector.value = null;
}

document.title = 'New Connection';

const vuetifyDefaults = ref({
  global: {
    density: 'compact',
  },
  VRow: { dense: true },
  VBtn: { density: 'default' },
  VTextField: { 
    paste(val) {
      console.log(val);
      return val;
    } 
  }
});

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

</style>
