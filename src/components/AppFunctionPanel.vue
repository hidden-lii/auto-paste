<script setup lang="ts">
import { computed, ref } from 'vue';
import WindowSizeDialog from './WindowSizeDialog.vue';
import { useWindowSize } from '../composables/useWindowSize';

const props = defineProps<{
	hideUsername: boolean;
	hidePassword: boolean;
}>();

const emit = defineEmits<{
	'toggle-username': [];
	'toggle-password': [];
}>();

const expanded = ref(false);
const windowSizeDialogOpen = ref(false);
const { width, height, refresh } = useWindowSize();

const usernameTitle = computed(() =>
	props.hideUsername ? '显示完整用户名' : '隐藏完整用户名'
);
const passwordTitle = computed(() =>
	props.hidePassword ? '显示明文密码' : '隐藏明文密码'
);

function toggleExpanded() {
	expanded.value = !expanded.value;
}

function openWindowSizeDialog() {
	expanded.value = false;
	windowSizeDialogOpen.value = true;
}

function onToggleUsername() {
	expanded.value = false;
	emit('toggle-username');
}

function onTogglePassword() {
	expanded.value = false;
	emit('toggle-password');
}

async function onWindowSizeRestored() {
	await refresh();
}
</script>

<template>
	<div v-click-outside="() => (expanded = false)" class="function-panel">
		<v-expand-transition>
			<v-list v-show="expanded" density="compact" nav class="function-menu">
				<v-list-item
					:title="usernameTitle"
					:prepend-icon="hideUsername ? 'mdi-account-off' : 'mdi-account'"
					@click="onToggleUsername"
				/>
				<v-list-item
					:title="passwordTitle"
					:prepend-icon="hidePassword ? 'mdi-lock-off' : 'mdi-lock'"
					@click="onTogglePassword"
				/>
				<v-list-item
					title="窗口大小"
					prepend-icon="mdi-resize"
					@click="openWindowSizeDialog"
				/>
			</v-list>
		</v-expand-transition>

		<v-toolbar class="function-toolbar" density="comfortable">
			<v-btn
				size="small"
				icon
				@click="toggleExpanded"
			>
				<v-icon icon="mdi-apps" />
			</v-btn>
		</v-toolbar>

		<WindowSizeDialog
			v-model="windowSizeDialogOpen"
			:width="width"
			:height="height"
			@restored="onWindowSizeRestored"
		/>
	</div>
</template>

<style scoped>
.function-panel {
	position: relative;
	display: flex;
	flex-direction: column-reverse;
	align-items: stretch;
	flex-shrink: 0;
	width: 48px;
}

.function-toolbar {
	width: 100%;
}

.function-menu {
	position: absolute;
	bottom: 100%;
	left: 0;
	width: 180px;
	padding: 0;
	background: rgb(var(--v-theme-surface));
	box-shadow: none;
}
</style>
