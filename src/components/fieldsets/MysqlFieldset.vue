<script setup>
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const emit = defineEmits(['update:modelValue']);
const props = defineProps({
  modelValue: { type: Object, required: true },
});

const dbHost = computed({
  get:() => props.modelValue?.dbHost ?? 'localhost',
  set:(dbHost) => emit('update:modelValue', { ...props.modelValue, dbHost }),
});

const dbUser = computed({
  get:() => props.modelValue?.dbUser ?? '',
  set:(dbUser) => emit('update:modelValue', { ...props.modelValue, dbUser }),
});

const dbPassword = computed({
  get:() => props.modelValue?.dbPassword ?? '',
  set:(dbPassword) => emit('update:modelValue', { ...props.modelValue, dbPassword }),
});

const dbPort = computed({
  get:() => props.modelValue?.dbPort ?? '3306',
  set:(dbPort) => emit('update:modelValue', { ...props.modelValue, dbPort }),
});

// const dbHost = ref(props.connection.dbHost ?? '');
// const dbUser = ref(props.value.value?.dbUser ?? '');
// const dbPassword = ref(props.value.value?.dbPassword ?? '');
// const dbPort = ref(props.value.value?.dbPort ?? '');

// watch(dbHost, () => emit('input', { ...connection, dbHost }));
// watch(dbUser, () => emit('input', { ...value.value, dbUser }));
// watch(dbPassword, () => emit('input', { ...value.value, dbPassword }));
// watch(dbPort, () => emit('input', { ...value.value, dbPort }));

</script>

<template>
  <v-text-field dense v-model="dbHost" variant="underlined" label="MySQL Host"></v-text-field>
  <v-text-field dense v-model="dbUser" variant="underlined" label="MySQL User"></v-text-field>
  <v-text-field dense v-model="dbPassword" type="password" variant="underlined" label="MySQL Password"></v-text-field>
  <v-text-field dense v-model="dbPort" variant="underlined" label="MySQL Port"></v-text-field>
</template>
