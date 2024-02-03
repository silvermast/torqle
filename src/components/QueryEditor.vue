<script setup>
import { ref, onMounted, computed } from 'vue';
import ace, { Range } from 'ace-builds';
import aceThemeLightUrl from 'ace-builds/src-noconflict/theme-cloud9_day?url';
import aceThemeDarkUrl from 'ace-builds/src-noconflict/theme-cloud9_night?url';
import aceModeSqlUrl from 'ace-builds/src-noconflict/mode-sql?url';
import aceModeMysqlUrl from 'ace-builds/src-noconflict/mode-mysql?url';
import aceModePostgresUrl from 'ace-builds/src-noconflict/mode-pgsql?url';
import aceModeSqlServerUrl from 'ace-builds/src-noconflict/mode-sqlserver?url';
import aceModeJavascriptUrl from 'ace-builds/src-noconflict/mode-javascript?url';

const delimiter = ';';

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

let selectedQueryMarkerId = null;

function getQuery() {
  return aceEditor?.getSelectedText() || getQueryUnderCursor() || aceEditor?.getValue();
}

function getQueryUnderCursor() {
  const lines = aceEditor.session.selection.cursor.document.getAllLines();
  const { column, row } = aceEditor.session.selection.cursor;

  const start = findDelimiterBefore({ lines, column, row });
  const end = findDelimiterAfter({ lines, column, row });
  const range = new Range(start.row, start.column, end.row, end.column);
  
  aceEditor.session.removeMarker(selectedQueryMarkerId);
  selectedQueryMarkerId = aceEditor.session.addMarker(range, 'ace_active-line', 'text');
  
  const result = aceEditor.session.doc.getTextRange(range);
  console.log('queryUnderCursor', { start, end, result });
  return result;
}

function findDelimiterBefore({ lines, column, row }) {
  // loop backwards through lines
  for (let i = row; i >= 0; i--) {
    // For starting row, trim off everything _after_ the cursor position.
    let text = i === row ? lines[i].substr(0, column) : lines[i];
    if (text.includes(delimiter)) {
      return {
        column: text.lastIndexOf(delimiter) + 1,
        row: i,
      };
    }
  }
  return { column: 0, row: 0 }; // no match? assume beginning of document
}

function findDelimiterAfter({ lines, column, row }) {
  // loop forwards through lines
  for (let i = row; i < lines.length; i++) {
    const text = lines[i];
    const start = i === row ? column : 0; // For starting row, start searching only after the cursor position.
    if (text.includes(delimiter, start)) {
      return {
        column: text.indexOf(delimiter, start),
        row: i,
      }
    }
  }

  // no match? assume end of content
  return {
    column: lines.length - 1,
    row: lines[lines.length - 1].length - 1,
  }
}

const isDark = Boolean(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);

onMounted(() => {
  ace.config.setModuleUrl('ace/mode/mysql', aceModeMysqlUrl);
  ace.config.setModuleUrl('ace/theme/cloud9_day', aceThemeLightUrl);
  ace.config.setModuleUrl('ace/theme/cloud9_night', aceThemeDarkUrl);

  aceEditor = window.aceEditor = ace.edit(elEditor.value, {
    mode: 'ace/mode/mysql',
    theme: isDark ? 'ace/theme/cloud9_night' : 'ace/theme/cloud9_day',
    printMargin: false,
    highlightActiveLine: false,
  });

  aceEditor.setValue(`SELECT * FROM testdb.users LIMIT 100;
  
  SELECT
    foo,
    bar,
    blah
  FROM db.table
  JOIN db.meta ON foo = bar
  WHERE foo = true
  AND bar = 'blah'
  LIMIT 100
  ;
  `);

  const emitData = () => {
    const newValue = aceEditor.getSelectedText() || aceEditor.getValue();
    if (query.value !== newValue) {
      query.value = newValue;
    }
  }

  let selectedText = null;
  let queryUnderCursor = null;

  aceEditor.session.on('change', emitData);
  aceEditor.session.on('changeSelectionStyle', emitData);
  // aceEditor.session.selection.on('changeSelection', e => {
  //   console.log('changeSelection', e);
  //   const { selection } = aceEditor.session;
  //   // const range = selection.getRange();
  //   console.log('Selected Text:', selection.doc.getTextRange(selection.getRange()));
  // });
  aceEditor.session.selection.on('changeCursor', e => {
    console.log('cursor', e);
    const { selection } = aceEditor.session;
    // const range = selection.getRange();
    console.log('Query:', getQuery());
  });

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
