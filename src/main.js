import { createApp } from "vue";
import './helpers.mjs';
import './polyfill.mjs';
import './styles.scss';
import { createPinia } from 'pinia'
import App from "./App.vue";
// import { getCurrent } from "@tauri-apps/api/window";

// Vuetify
import '~/scss/vuetify-theme.scss';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

const isDarkMode = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
// console.log('THEME', getCurrent().theme);

const vuetify = createVuetify({
  components,
  directives,
  defaults: {
    // global: { density: 'compact' },
    VRow: { dense: true },
    VBtn: { density: 'default', rounded: true },
  },
  icons: {
    iconfont: 'mdi', // https://pictogrammers.com/library/mdi/
  },
  theme: {
    defaultTheme: isDarkMode ? 'dark' : 'light',
  },
})

const pinia = createPinia();

createApp(App).use(pinia).use(vuetify).mount("#app");
