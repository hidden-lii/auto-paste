<script setup lang="ts">
import { computed, ref } from 'vue';
import WindowSizeDialog from './WindowSizeDialog.vue';
import { useWindowSize } from '../composables/useWindowSize';

const props = defineProps<{
	hideUsername: boolean;
	hidePassword: boolean;
	defaultHideUsername: boolean;
	defaultHidePassword: boolean;
}>();

const emit = defineEmits<{
	'toggle-username': [];
	'toggle-password': [];
	'update:default-hide-username': [value: boolean];
	'update:default-hide-password': [value: boolean];
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

function onDefaultHideUsernameChange(value: boolean | null | undefined) {
	if (typeof value !== 'boolean') {
		return;
	}
	emit('update:default-hide-username', value);
}

function onDefaultHidePasswordChange(value: boolean | null | undefined) {
	if (typeof value !== 'boolean') {
		return;
	}
	emit('update:default-hide-password', value);
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
				<v-divider class="my-1" />
				<v-list-subheader class="function-menu-subheader">
					默认显示设置
				</v-list-subheader>
				<v-list-item title="默认隐藏用户名" class="function-menu-switch">
					<template #append>
						<v-switch
							:model-value="defaultHideUsername"
							density="compact"
							hide-details
							color="primary"
							@click.stop
							@update:model-value="onDefaultHideUsernameChange"
						/>
					</template>
				</v-list-item>
				<v-list-item title="默认隐藏密码" class="function-menu-switch">
					<template #append>
						<v-switch
							:model-value="defaultHidePassword"
							density="compact"
							hide-details
							color="primary"
							@click.stop
							@update:model-value="onDefaultHidePasswordChange"
						/>
					</template>
				</v-list-item>
				<v-divider class="my-1" />
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
	width: 220px;
	padding: 0;
	background: rgb(var(--v-theme-surface));
	box-shadow: none;
}

.function-menu-subheader {
	min-height: 28px;
	padding-top: 4px;
	padding-bottom: 0;
	font-size: 0.75rem;
}

.function-menu-switch {
	min-height: 40px;
}

.function-menu-switch :deep(.v-list-item__append) {
	margin-inline-start: 8px;
}
</style>
