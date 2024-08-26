<script setup>
import { computed, ref } from "vue";
import { open } from '@tauri-apps/plugin-dialog';
import { makeSpicySnack } from "../Snacks.vue";

const emit = defineEmits(['update:modelValue']);
const props = defineProps({
  modelValue: { type: Object, required: true },
});

const data = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val),
});
const isUsingMemory = computed(() => data.value.filepath === ':memory:')
const isUsingFile = computed(() => data.value.filepath && data.value.filepath !== ':memory:')
const isOpeningFile = ref(false);

async function openDialog() {
  isOpeningFile.value = true;
  try {
    const fileHandle = await open();
    data.value.filepath = fileHandle.path;
  } catch (e) {
    makeSpicySnack(e);
    console.error(e);
  }
  isOpeningFile.value = false;
}

function useMemory() {
  data.value.filepath = ':memory:';
}

</script>

<template>
  <v-row>
    <v-col cols="8">
      <v-btn prepend-icon="mdi-memory" @click="useMemory" variant="outlined"
        class="mr-3"
        :color="isUsingMemory ? 'white' : 'grey'">
        Use Memory
      </v-btn>
      <v-btn prepend-icon="mdi-file" @click="openDialog" :disabled="isOpeningFile" variant="outlined"
        :color="isUsingFile ? 'white' : 'grey'">
        Select File
      </v-btn>
      <div class="mx-3 my-3" v-if="isUsingFile"><small v-text="data.filepath" /></div>
    </v-col>
  </v-row>
</template>
