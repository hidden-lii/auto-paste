<script setup lang="ts">
import AccountPage from './pages/AccountPage.vue';

// 禁止右键菜单的出现
function disableContextMenu(event: { preventDefault: () => void }) {
	event.preventDefault();
}

// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
</script>

<template>
	<v-app @contextmenu="disableContextMenu">
		<account-page />
	</v-app>
</template>

<style scoped>
</style>
