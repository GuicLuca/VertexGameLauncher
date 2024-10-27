import { createApp } from "vue";
import App from "./App.vue";

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

// .v-navigation-drawer {
//   background-color: #2f2f2f;
// }
// .v-list-item {
//   color: white;
// }
// .v-list-item--active {
//   background-color: #3f3f3f;
// }

const vuetify = createVuetify({
    components,
    directives,
    icons: {
      defaultSet: 'mdi',
    },
    theme: {
      defaultTheme: 'dark'
    },
    display: {
      mobileBreakpoint: 'sm',
      thresholds: {
        xs: 500,
        sm: 500,
        md:500,
        lg: 500,
        xl: 500,
      },
    },
  })

//~ End Vuetify

createApp(App).use(vuetify).mount("#app");
