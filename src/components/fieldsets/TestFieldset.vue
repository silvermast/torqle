<script setup>
import { computed } from "vue";

const emit = defineEmits(['update:modelValue']);
const props = defineProps({
  modelValue: { type: Object, required: true },
});

/**
 * These variable names correlate directly with the rust mysql package options
 * @see https://docs.rs/mysql/23.0.1/mysql/struct.OptsBuilder.html#method.new
 */

const database = computed({
  get:() => props.modelValue?.database ?? 'localbase',
  set:(database) => emit('update:modelValue', { ...props.modelValue, database }),
});

// const database = ref(props.connection.database ?? '');
// const user = ref(props.value.value?.user ?? '');
// const password = ref(props.value.value?.password ?? '');
// const port = ref(props.value.value?.port ?? '');

// watch(database, () => emit('input', { ...connection, database }));
// watch(user, () => emit('input', { ...value.value, user }));
// watch(password, () => emit('input', { ...value.value, password }));
// watch(port, () => emit('input', { ...value.value, port }));

</script>

<template>
  <v-text-field density="compact" v-model="database" variant="outlined" label="Test database"></v-text-field>
</template>
