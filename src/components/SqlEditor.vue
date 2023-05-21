<script setup>
import { ref, onMounted } from 'vue';
import { EditorView, keymap, gutter, lineNumbers, ViewPlugin, ViewUpdate } from '@codemirror/view';
import { EditorState, Compartment, StateField } from '@codemirror/state';
import { standardKeymap } from '@codemirror/commands';
import { LanguageSupport } from '@codemirror/language';
import { sql as langSql, MySQL as dialectMysql, PostgreSQL as dialectPostgresql, SQLite as dialectSqlite, sql } from '@codemirror/lang-sql';

const dialects = {
  Sqlite: dialectSqlite,
  Postgres: dialectPostgresql,
  Mysql: dialectMysql,
};

const emit = defineEmits(['runSelected', 'update:modelValue']);
const props = defineProps({
  dialect: { type: String, default: 'Mysql' },
});

const elContainer = ref();

/**
 * Based on the cursor position, return the highlighted text
 * @param {Text} fullText 
 * @param {Object}
 */
function getSelectedQuery(fullText, { from, to }) {
  let startIndex = from;
  let endIndex = to;
  if (from === to) { // if no selection is made, pull the highlighted query
    // look backwards for a delimiter (;) and set startIndex to that index + 1
    for (let i = from - 1; i >= 0; i--) {
      startIndex = i; // support returning 0 if there's only 1 query in the dataset
      if (fullText[i] === ';') {
        startIndex++; // increment by 1 so we don't include the ;
        break;
      }
    }
    // look forwards for a delimiter
    for (let i = to; i <= fullText.length - 1; i++) {
      endIndex = i;
      if (fullText[i] === ';') {
        break;
      }
    }
  }

  return fullText.slice(startIndex, endIndex);
}

const languageSupport = new Compartment();
const lang = new langSql();
const cmState = EditorState.create({
  doc: '',
  tabSize: 4,
  extensions: [
    ViewPlugin.fromClass(class {
      timeout = null;

      constructor(view) {
      }

      /**
       * @param {ViewUpdate} update 
       */
      update(update) {
        clearTimeout(this.timeout);
        this.timeout = setTimeout(() => {
          emit('update:modelValue', getSelectedQuery(update.state.doc.toString(), update.state.selection.main));
        }, 50);
      }
    }),

    /** @TODO - syntax highlighting not working? */
    /** @TODO - code completion */
    // language
    languageSupport.of(lang),
    // (new Compartment()).of(langSql({
    //   dialect: dialects[props.dialect],
    // })),

    // custom keymaps
    keymap.of([
      ...standardKeymap,
      { key: 'Ctrl-Enter', mac: 'Cmd-Enter', run: () => runSelected() },
    ]),

    lineNumbers(),
    gutter({ class: "cm-gutter" }),
  ]
});

function runSelected() {
  // const cursorPosition = cmState.doc;
  emit('runSelected', fullText);
}

// CodeMirror inits
onMounted(() => {
  try {
    new EditorView({
      parent: elContainer.value,
      state: cmState,
    });
  } catch (e) {
    console.error(e);
  }
});

</script>

<template>
  <div class="editor-container" ref="elContainer"></div>
</template>

<style lang="scss" scoped>
.editor-container {
  width: 100%;
  height: 100%;
  overflow: auto;
}
</style>
<style lang=scss>
.cm-editor {
  height: 100%;
  width: 100%;
}
</style>
