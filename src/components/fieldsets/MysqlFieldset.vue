<script setup>
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const emit = defineEmits(['update:modelValue']);
const props = defineProps({
  modelValue: { type: Object, required: true },
});

/**
 * These variable names correlate directly with the rust mysql package options
 * @see https://docs.rs/mysql/23.0.1/mysql/struct.OptsBuilder.html#method.new
 */

const host = computed({
  get:() => props.modelValue?.host ?? 'localhost',
  set:(host) => emit('update:modelValue', { ...props.modelValue, host }),
});

const user = computed({
  get:() => props.modelValue?.user ?? '',
  set:(user) => emit('update:modelValue', { ...props.modelValue, user }),
});

const password = computed({
  get:() => props.modelValue?.password ?? '',
  set:(password) => emit('update:modelValue', { ...props.modelValue, password }),
});

const port = computed({
  get:() => props.modelValue?.port ?? '3306',
  set:(port) => emit('update:modelValue', { ...props.modelValue, port }),
});

// const host = ref(props.connection.host ?? '');
// const user = ref(props.value.value?.user ?? '');
// const password = ref(props.value.value?.password ?? '');
// const port = ref(props.value.value?.port ?? '');

// watch(host, () => emit('input', { ...connection, host }));
// watch(user, () => emit('input', { ...value.value, user }));
// watch(password, () => emit('input', { ...value.value, password }));
// watch(port, () => emit('input', { ...value.value, port }));

</script>

<template>
  <v-row>
    <v-col cols="8">
      <v-text-field density="compact" v-model="host" variant="underlined" label="MySQL Host"></v-text-field>
    </v-col>
    <v-col cols="4">
      <v-text-field density="compact" v-model="port" variant="underlined" label="MySQL Port"></v-text-field>
    </v-col>
  </v-row>
  <v-text-field density="compact" v-model="user" variant="underlined" label="MySQL User"></v-text-field>
  <v-text-field density="compact" v-model="password" type="password" variant="underlined" label="MySQL Password"></v-text-field>
</template>
