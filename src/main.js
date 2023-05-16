import { createApp } from "vue";
import './polyfill.mjs';
import "./styles.scss";
// import * as bootstrap from 'bootstrap';
import App from "./App.vue";

// Vuetify
import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

const isDarkMode = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;

const vuetify = createVuetify({
  components,
  directives,
  icons: {
    iconfont: 'mdi', // https://pictogrammers.com/library/mdi/
  },
  theme: {
    defaultTheme: isDarkMode ? 'dark' : 'light'
  }
})

createApp(App).use(vuetify).mount("#app");
