<script>
import { ref } from 'vue';

const snack = ref({
  enabled: false,
  text: '',
  color: 'gray',
});

export function makeSnack({ text, color }) {
  snack.value = { text, color, enabled: true };
}
export function hideSnack() {
  snack.value = { ...snack.value, enabled: false };
}
export function makeSpicySnack(text) {
  snack.value = { color: 'red', enabled: true, text };
}
export function makeHappySnack(text) {
  snack.value = { color: 'green', enabled: true, text };
}

export default {
  data: () => ({ snack }),
  setup() {
    snack.value.enabled = false;
  }
}
</script>

<template>
  <v-snackbar v-model="snack.enabled" :color="snack.color" contained location="bottom">
    {{ snack.text }}
    <template v-slot:actions>
      <v-btn size="small" rounded variant="tonal" @click="snack.enabled = false">
        Dismiss
      </v-btn>
    </template>
  </v-snackbar>
</template>

<style lang="scss" scoped>
</style>