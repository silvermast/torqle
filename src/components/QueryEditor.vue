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
import shortcuts from '../services/KeyboardShortcuts.js';

const delimiter = ';'; // @todo: convert to prop?

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

/** @type {import('ace-builds').Ace.Editor} */
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

/**
 * @todo optimize this...
 * - do NOT debounce
 * - separate text highlight from query.value mutation?
 */
function moveCursor() {
    // remove any background highlighting for the selected query
    aceEditor.session.removeMarker(selectedQueryMarkerId);
    
    // If there's a manual selection, return that, verbatim.
    if (aceEditor?.getSelectedText()) {
      query.value = aceEditor?.getSelectedText();
      return;
    }

    // If there's no selection, infer the query based on the cursor position and specified delimiter
    const lines = aceEditor.session.selection.cursor.document.getAllLines();
    const { column, row } = aceEditor.session.selection.cursor;

    // run these async, but don't define the methods as async
    let start = findDelimiterBefore({ lines, column, row });
    let end = findDelimiterAfter({ lines, column, row });
    let range = new Range(start.row, start.column, end.row, end.column);

    /**
     * @todo: select previous query when cursor is positioned immediately after
     * @example SELECT foo FROM bar;|
     */
    //
    // let queryText = aceEditor.session.doc.getTextRange(range);
    // if (!queryText.trim().length) {
    //   end = { ...start };
    //   start = findDelimiterBefore({ lines, ...start });
    //   queryText = aceEditor.session.doc.getTextRange(range);
    // }
    
    aceEditor.session.removeMarker(selectedQueryMarkerId);
    selectedQueryMarkerId = aceEditor.session.addMarker(range, 'ace_active-line', 'text');
    
    query.value = aceEditor.session.doc.getTextRange(range);
}

function findDelimiterBefore({ lines, column, row }) {
  // loop backwards through lines
  for (let i = row; i >= 0; i--) {
    // For starting row, trim off everything _after_ the cursor position.
    let text = i === row ? lines[i].substr(0, column) : lines[i];
    if (text.includes(delimiter)) {
      let textTrimmed = text.trim();
      if (textTrimmed.lastIndexOf(delimiter) === textTrimmed.length - 1) {
        // is there text on the same line after the delimiter? If not, use the next line.
        return { 
          column: 0,
          row: i + 1,
        };
      } else {
        // e.g. SELECT foo; SELE|CT bar;
        return {
          column: text.lastIndexOf(delimiter) + 1,
          row: i,
        };
      }
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
        column: text.indexOf(delimiter, start) + 1,
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
  ace.config.setModuleUrl('ace/mode/mysql', aceModeMysqlUrl); /** @todo move to connection, or computed Dialect prop */
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

  // custom cursor logic happens here. Be careful about adding too many listeners!
  aceEditor.session.selection.on('changeCursor', () => moveCursor());

  /**
   * Keyboard Shortcuts!
   * @see https://ace.c9.io/demo/keyboard_shortcuts.html
   * @todo Update multicursor support to match Sublime (or vscode)
   */

  aceEditor.commands.addCommand({
    name: 'runQuery',
    bindKey: shortcuts.editor.runQuery.forAce(),
    exec: function(editor) {
        emit('runSelected', query.value);
    },
    readOnly: true,
  });

  aceEditor.commands.addCommand({
    ...aceEditor.commands.byName.selectMoreAfter,
    bindKey: shortcuts.editor.addCursorAtNextItem.forAce(),
  });

  aceEditor.commands.addCommand({
    ...aceEditor.commands.byName.selectMoreBefore,
    bindKey: shortcuts.editor.addCursorAtPrevItem.forAce(),
  });

  aceEditor.commands.addCommand({
    ...aceEditor.commands.byName.toggleSplitSelectionIntoLines,
    bindKey: shortcuts.editor.addCursorsAtSelectedLines.forAce(),
  });

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
