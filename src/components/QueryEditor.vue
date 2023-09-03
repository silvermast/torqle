<script setup>
import { ref, onMounted, computed } from 'vue';
import ace from 'ace-builds';
import aceThemeLightUrl from 'ace-builds/src-noconflict/theme-cloud9_day?url';
import aceThemeDarkUrl from 'ace-builds/src-noconflict/theme-cloud9_night?url';
import aceModeSqlUrl from 'ace-builds/src-noconflict/mode-sql?url';
import aceModeMysqlUrl from 'ace-builds/src-noconflict/mode-mysql?url';
import aceModePostgresUrl from 'ace-builds/src-noconflict/mode-pgsql?url';
import aceModeSqlServerUrl from 'ace-builds/src-noconflict/mode-sqlserver?url';
import aceModeJavascriptUrl from 'ace-builds/src-noconflict/mode-javascript?url';

const modes = {
  mysql: { path: 'ace/mode/mysql', module: aceModeMysqlUrl },
  postgres: { path: 'ace/mode/psql', module: aceModePostgresUrl },
  sql: { path: 'ace/mode/sql', module: aceModeSqlUrl },
  mongo: { path: 'ace/mode/javascript', module: aceModeJavascriptUrl },
  sqlserver: { path: 'ace/mode/sqlserver', module: aceModeSqlServerUrl },
}

/**
 * Extending Ace
 * @url https://www.npmjs.com/package/ace-builds
 * @url https://www.npmjs.com/package/vue3-ace-editor
 */

let aceEditor;
const emit = defineEmits(['runSelected', 'update:modelValue']);
const props = defineProps({
  modelValue: { type: String, default: '' },
  dialect: { type: String, default: 'Mysql' },
});

const query = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const elEditor = ref();
const cursor = ref({
  start: 0,
  end: 0,
});

function getQuery() {
  return aceEditor?.getSelectedText() || aceEditor?.getValue();
}

/**
 * @todo: add ability to run query under cursor
 */
// let debounce;
// function emitSelectedQuery(event) {
//   clearTimeout(debounce);
//   debounce = setTimeout(() => {
//     const text = event.target.value;
//     const start = event.target.selectionStart;
//     const end = event.target.selectionEnd;
//     cursor.value = { start, end };

//     let startIndex = start;
//     let endIndex = end;
//     if (start === end) { // if no selection is made, pull the highlighted query
//       // look backwards for a delimiter (;) and set startIndex to that index + 1
//       for (let i = start - 1; i >= 0; i--) {
//         startIndex = i; // support returning 0 if there's only 1 query in the dataset
//         if (text[i] === ';') {
//           startIndex++; // increment by 1 so we don't include the ;
//           break;
//         }
//       }
//       // look forwards for a delimiter
//       for (let i = end; i <= text.length - 1; i++) {
//         endIndex = i;
//         if (text[i] === ';') {
//           break;
//         }
//       }
//     }

//     query.value = text.slice(startIndex, endIndex);
//   }, 50);
// }

const isDark = Boolean(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);

onMounted(() => {
  ace.config.setModuleUrl('ace/mode/mysql', aceModeMysqlUrl);
  ace.config.setModuleUrl('ace/theme/cloud9_day', aceThemeLightUrl);
  ace.config.setModuleUrl('ace/theme/cloud9_night', aceThemeDarkUrl);

  aceEditor = window.aceEditor = ace.edit(elEditor.value, {
    mode: 'ace/mode/mysql',
    theme: isDark ? 'ace/theme/cloud9_night' : 'ace/theme/cloud9_day',
    printMargin: false,
  });

  aceEditor.setValue('SELECT * FROM testdb.users LIMIT 100;');

  const emitData = () => {
    const newValue = aceEditor.getSelectedText() || aceEditor.getValue();
    if (query.value !== newValue) {
      query.value = newValue;
    }
  }

  aceEditor.on('change', emitData);
  aceEditor.on('changeSelectionStyle', emitData);

  aceEditor.commands.addCommand({
    name: 'runQuery',
    bindKey: {win: 'Ctrl-Enter',  mac: 'Command-Enter'},
    exec: function(editor) {
        emit('runSelected', getQuery());
    },
    readOnly: true,
  })

  // editor.commands.removeCommand
});
</script>

<template>
  <div ref="elEditor" class="the-editor" />
</template>

<style lang="scss">
.the-editor {
  position: relative;
  width: inherit;
  height: inherit;
}
</style>
