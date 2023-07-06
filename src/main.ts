import { createApp } from 'vue';
import './styles.css';
import '@mdi/font/css/materialdesignicons.css';

import App from './App.vue';
// Vuetify
import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { aliases, mdi } from 'vuetify/iconsets/mdi';
import VuetifyUseDialog from 'vuetify-use-dialog';

const app = createApp(App);
const vuetify = createVuetify({
	icons: {
		defaultSet: 'mdi',
		aliases,
		sets: {
			mdi
		}
	},
	components,
	directives,
	theme: {
		defaultTheme: 'dark'
	}
});

app.use(vuetify);
app.use(VuetifyUseDialog);

app.mount('#app');
