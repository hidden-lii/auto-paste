import { createApp } from "vue";
import "./styles.css";
import '@mdi/font/css/materialdesignicons.css'

import App from "./App.vue";
// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

const vuetify = createVuetify({
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi,
        },
    },
    components,
    directives,
    theme: {
        defaultTheme: 'dark',
    }
})

createApp(App).use(vuetify).mount("#app");
