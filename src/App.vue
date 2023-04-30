<script setup>
import { ref } from 'vue';
import ConnectView from './components/ConnectView.vue';
import QueryView from './components/QueryView.vue';

// import Greet from './components/Greet.vue';

let connection = ref({ fll: true });

</script>

<template>
  <v-app>
    <ConnectView v-if="!connection" @connect="$event => connection = { ...$event }" />
    <QueryView v-else v-bind="{ connection }" @disconnect="connection = null" />
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
