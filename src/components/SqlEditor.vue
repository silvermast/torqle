<script setup>
import { ref, onMounted } from 'vue';
import { EditorView, keymap, gutter, lineNumbers } from '@codemirror/view';
import { EditorState, Compartment, StateField } from '@codemirror/state';
import { standardKeymap } from '@codemirror/commands';
import { sql as langSql, MySQL as dialectMysql, PostgreSQL as dialectPostgresql, SQLite as dialectSqlite } from '@codemirror/lang-sql';

const dialects = {
  Sqlite: dialectSqlite,
  Postgres: dialectPostgresql,
  Mysql: dialectMysql,
};

const emit = defineEmits(['runSelected']);
const props = defineProps({
  dialect: { type: String, default: 'Mysql' },
})

const elContainer = ref();

const cmState = EditorState.create({
  doc: '',
  tabSize: 4,
  extensions: [
    // EditorView.updateListener.of(e => {
    //   console.log(e);
    //   // emit('update:modelValue', cmState.doc.toString());
    // }),

    /** @TODO - syntax highlighting not working? */
    /** @TODO - code completion */
    // language
    (new Compartment()).of(langSql({
      dialect: dialects[props.dialect],
    })),

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
  emit('runSelected',)
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
