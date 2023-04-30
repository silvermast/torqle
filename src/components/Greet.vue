<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

const table = ref(new Array(1000).fill(0).map(() => new Array(20).fill('test')));

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet Me</button>
  </div>

  <table v-if="greetMsg">
    <tr v-for="(row, rKey) in table" :key="`row-${rKey}`">
      <td v-for="(field, fKey) in row" :key="`field-${rKey}-${fKey}`">{{ field }}</td>
    </tr>
  </table>
</template>
