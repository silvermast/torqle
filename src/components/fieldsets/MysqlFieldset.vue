<script setup>
import { computed, ref, watch } from "vue";
import Password from '~/components/Password.vue';

const emit = defineEmits(['update:modelValue']);
const props = defineProps({
  modelValue: { type: Object, required: true },
});

const data = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val),
});

if (!data.value.host) {
  data.value.host = 'localhost';
}
if (!data.value.port) {
  data.value.port = '3306';
}

// watch(props.modelValue, (newData) => data.value = { ...newData });
// watch(data, () => emit('update:modelValue', data.value));

// data.value.host = data.value.host || 'localhost';
// data.value.port = data.value.port || '3306';

/**
 * These variable names correlate directly with the rust mysql package options
 * @see https://docs.rs/mysql/23.0.1/mysql/struct.OptsBuilder.html#method.new
 */
// const host = computed({
//   get: () => props.modelValue?.host ?? '',
//   set: (host) => emit('update:modelValue', { ...props.modelValue, host }),
// });

// const user = computed({
//   get: () => props.modelValue?.user ?? '',
//   set: (user) => emit('update:modelValue', { ...props.modelValue, user }),
// });

// const password = computed({
//   get: () => props.modelValue?.password ?? '',
//   set: (password) => emit('update:modelValue', { ...props.modelValue, password }),
// });

// const port = computed({
//   get: () => props.modelValue?.port ?? '',
//   set: (port) => emit('update:modelValue', { ...props.modelValue, port }),
// });

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
      <v-text-field density="compact" v-model="data.host" variant="outlined" label="MySQL Host" />
    </v-col>
    <v-col cols="4">
      <v-text-field density="compact" v-model="data.port" variant="outlined" label="Port" type="number" />
    </v-col>
  </v-row>
  <v-text-field density="compact" v-model="data.user" variant="outlined" label="MySQL User" />
  <Password density="compact" v-model="data.password" variant="outlined" label="MySQL Password" />
</template>
