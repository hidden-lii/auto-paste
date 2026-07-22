<script setup lang="ts">
import { onMounted } from 'vue';
import AccountPage from './pages/AccountPage.vue';
import { migrateWindowSizeFromLocalStorage, normalizeSavedWindowSize } from './utils/window';

function disableContextMenu(event: { preventDefault: () => void }) {
	event.preventDefault();
}

onMounted(async () => {
	await migrateWindowSizeFromLocalStorage();
	await normalizeSavedWindowSize();
});
</script>

<template>
	<v-app @contextmenu="disableContextMenu">
		<v-main class="app-main">
			<account-page />
		</v-main>
	</v-app>
</template>

<style>
html,
body,
#app {
	height: 100%;
	margin: 0;
	overflow: hidden;
}

.v-application {
	height: 100% !important;
}

.v-application__wrap {
	min-height: 0 !important;
	height: 100%;
	overflow: hidden;
}

.app-main {
	height: 100%;
	padding: 0 !important;
	overflow: hidden;
}
</style>
