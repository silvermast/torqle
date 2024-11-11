<script setup>
import { computed, ref, watch } from "vue";
import Password from '~/components/fields/Password.vue';

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
  <v-text-field density="compact" v-model="data.database" variant="outlined" label="MySQL Database" />
</template>
