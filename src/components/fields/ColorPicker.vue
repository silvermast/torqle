<script setup>
import { defineEmits } from "vue";
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

const swatches = [
    [
        "#f44336",
        "#e91e63",
        "#9c27b0",
        "#3f51b5",
        "#2196f3",
        "#4caf50",
        "#ffeb3b",
        "#ff9800",
        "#607d8b"
    ],
    [
        "#c62828",
        "#ad1457",
        "#6a1b9a",
        "#283593",
        "#1565c0",
        "#2e7d32",
        "#f9a825",
        "#ef6c00",
        "#37474f"
    ],
    [
        "#ef9a9a",
        "#f48fb1",
        "#ce93d8",
        "#9fa8da",
        "#90caf9",
        "#a5d6a7",
        "#fff59d",
        "#ffcc80",
        "#b0bec5"
    ],
    [
        "#ffcdd2",
        "#f8bbd0",
        "#e1bee7",
        "#c5cae9",
        "#bbdefb",
        "#c8e6c9",
        "#fff9c4",
        "#ffe0b2",
        "#cfd8dc"
    ],
    [
        "#000000",
        "#ffffff"
    ],
];


</script>

<template>
    <v-color-picker v-bind="{ ...defaults, swatches, ...props }"
        @update:modelValue="$event => emit('update:modelValue', $event)" />
</template>
