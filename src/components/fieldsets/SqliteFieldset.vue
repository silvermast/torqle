<script setup>
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const emit = defineEmits(['update:modelValue']);
const props = defineProps({
  modelValue: { type: Object, required: true },
});

const filePath = computed({
  get:() => props.modelValue?.filePath ?? ':memory:',
  set:(filePath) => emit('update:modelValue', { ...props.modelValue, filePath }),
});

</script>

<template>
  <v-row>
    <v-col cols="8">
      <v-text-field density="compact" v-model="filePath" variant="outlined" label="Sqlite File Path"></v-text-field>
    </v-col>
  </v-row>
</template>
