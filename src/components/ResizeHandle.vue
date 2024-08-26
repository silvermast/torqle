<script setup>
import { computed, ref, getCurrentInstance } from 'vue';
const props = defineProps({
    target: Object,
    vertical: Boolean,
    color: String,
    thickness: Number,
});

const elHandle = ref();

function getTarget() {
    return props.target ?? elHandle.value.offsetParent ?? elHandle.value.parentNode;
}

const style = computed(() => {
    const result = {
        background: props.color,
    };
    if (props.vertical) {
        result.height = '100%';
        result.width = result.maxWidth = result.minWidth = `${props.thickness}px`;
        result.cursor = 'col-resize';
    } else {
        result.width = '100%';
        result.height = result.maxHeight = result.minHeight = `${props.thickness}px`;
        result.cursor = 'row-resize';
    }
    return result;
});

function startResize($event) {
    $event.preventDefault();
    $event.stopPropagation();
    const resizeTarget = getTarget();
    const resizeMetric = props.vertical ? 'pageX' : 'pageY';
    const resizeStyle = props.vertical ? 'width' : 'height';
    const offset = props.vertical ? resizeTarget.offsetLeft : resizeTarget.offsetTop;
    const viewportMax = props.vertical ? (document.body.offsetWidth) : (document.body.offsetHeight);
    const maxValue = viewportMax - offset;
    const minValue = 0;

    function resize(e) {
        if (e[resizeMetric] > minValue && e[resizeMetric] < maxValue) {
            const newValue = e[resizeMetric] - offset;
            resizeTarget.style[resizeStyle] = `${newValue}px`;
            resizeTarget.style[`min-${resizeStyle}`] = `${newValue}px`;
        }
    }

    function stopResize(e) {
        document.removeEventListener('mousemove', resize); // stop resizing
        $event.target.removeEventListener('mouseup', stopResize); // cleanup
        document.removeEventListener('mouseup', stopResize); // cleanup
    }

    document.addEventListener('mousemove', resize);
    $event.target.addEventListener('mouseup', stopResize)
    document.addEventListener('mouseup', stopResize);
}

</script>

<template>
    <div ref="elHandle" class="resize-handle" :class="{ vertical: props.vertical }" @mousedown="startResize" :style="style"></div>
</template>

<style lang="scss" scoped>
.resize-handle {
    box-sizing: border-box;
    border: none;
    background: rgba(var(--v-theme-primary));
}
</style>