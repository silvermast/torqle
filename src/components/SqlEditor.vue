<script setup>
import { ref, onMounted, computed } from 'vue';
import hljs from 'highlight.js';
import CodeEditor from 'simple-code-editor';

const emit = defineEmits(['runSelected', 'update:modelValue']);
const props = defineProps({
  modelValue: { type: String, default: '' },
  dialect: { type: String, default: 'Mysql' },
});

const query = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const cursor = ref({
  start: 0,
  end: 0,
});

let debounce;
function emitSelectedQuery(event) {
  clearTimeout(debounce);
  debounce = setTimeout(() => {
    const text = event.target.value;
    const start = event.target.selectionStart;
    const end = event.target.selectionEnd;
    cursor.value = { start, end };

    let startIndex = start;
    let endIndex = end;
    if (start === end) { // if no selection is made, pull the highlighted query
      // look backwards for a delimiter (;) and set startIndex to that index + 1
      for (let i = start - 1; i >= 0; i--) {
        startIndex = i; // support returning 0 if there's only 1 query in the dataset
        if (text[i] === ';') {
          startIndex++; // increment by 1 so we don't include the ;
          break;
        }
      }
      // look forwards for a delimiter
      for (let i = end; i <= text.length - 1; i++) {
        endIndex = i;
        if (text[i] === ';') {
          break;
        }
      }
    }

    query.value = text.slice(startIndex, endIndex);
  }, 50);
}

/**
 * Binds events to the text area element in the code editor
 * @param {HTMLTextAreaElement} element 
 */
function textarea(element) {
  const events = ['select', 'input', 'keypress', 'paste', 'cut'];
  events.forEach(type => element.addEventListener(type, e => emitSelectedQuery(e)));
}

const isDark = Boolean(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
const options = {
  autofocus: true,
  header: false,
  wrap: false,
  theme: isDark ? 'base16-material-darker' : 'base16-material-lighter',
  lineNums: true,
  tabSpaces: 2,
  
  // css style overrides
  width: '100%',
  height: '100%',
  value: query.value,
  fontSize: '14px',
  borderRadius: '0',
  padding: '8px',
};
</script>

<template>
  <CodeEditor class="the-editor" v-bind="options" @textarea="textarea" />
</template>

<style lang="scss">
.the-editor {

}
</style>
