import { createApp } from "vue";
import './polyfill.mjs';
import "./styles.scss";
import { createPinia } from 'pinia'
import App from "./App.vue";

// Vuetify
import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

const isDarkMode = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;

// const vuetifyDefaults = {
//   global: {
//     density: 'compact',
//   },
//   VRow: { dense: true },
//   VBtn: { density: 'default', rounded: true },
//   VTextField: {
//     paste(val) {
//       console.log(val);
//       return val;
//     },
//   },
// };

const vuetify = createVuetify({
  components,
  directives,
  defaults: {
    global: { density: 'compact' },
    VRow: { dense: true },
    VBtn: { density: 'default', rounded: true },
  },
  icons: {
    iconfont: 'mdi', // https://pictogrammers.com/library/mdi/
  },
  theme: {
    defaultTheme: isDarkMode ? 'dark' : 'light'
  }
})

const pinia = createPinia();

createApp(App).use(pinia).use(vuetify).mount("#app");
