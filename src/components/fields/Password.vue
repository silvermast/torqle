<script setup>
import { computed, ref, defineEmits } from "vue";
const show = ref(false);
const emit = defineEmits(['update:modelValue']);
const props = defineProps(['modelValue']);
</script>

<template>
    <v-text-field v-bind="props" :type="show ? 'text' : 'password'" :append-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
        @update:modelValue="$event => emit('update:modelValue', $event)"
        @click:append="show = !show"></v-text-field>
</template>
