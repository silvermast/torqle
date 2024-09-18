<script setup>
import { defineEmits } from "vue";
import swatches from '~/swatches.json';
const emit = defineEmits(['update:modelValue']);
const props = defineProps(['modelValue']);

const defaults = {
    showSwatches: true,
    hideInputs: true,
    hideSliders: true,
    hideCanvas: true,
    mode: 'rgb',
    swatchesMaxHeight: 300,
};

</script>

<template>
    <v-color-picker v-bind="{ ...defaults, swatches, ...props }"
        @update:modelValue="$event => emit('update:modelValue', $event)" />
</template>
