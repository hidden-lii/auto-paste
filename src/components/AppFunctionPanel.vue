<script setup lang="ts">
import { computed, ref } from 'vue';
import WindowSizeDialog from './WindowSizeDialog.vue';
import ExportSettingsDialog from './ExportSettingsDialog.vue';
import { useWindowSize } from '../composables/useWindowSize';

const props = defineProps<{
	hideUsername: boolean;
	hidePassword: boolean;
	defaultHideUsername: boolean;
	defaultHidePassword: boolean;
	networkSyncEnabled: boolean;
	lastSync: string | null;
}>();

const emit = defineEmits<{
	'toggle-username': [];
	'toggle-password': [];
	'update:default-hide-username': [value: boolean];
	'update:default-hide-password': [value: boolean];
	'update:network-sync-enabled': [value: boolean];
}>();

const expanded = ref(false);

const formattedLastSync = computed(() => {
	if (!props.lastSync) {
		return null;
	}
	const timestamp = Number(props.lastSync);
	if (Number.isNaN(timestamp)) {
		return props.lastSync;
	}
	const date = new Date(timestamp * 1000);
	const pad = (value: number) => String(value).padStart(2, '0');
	return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`;
});
const windowSizeDialogOpen = ref(false);
const exportSettingsDialogOpen = ref(false);
const { width, height, refresh } = useWindowSize();

const usernameDisplay = computed({
	get: () => (props.hideUsername ? 'hide' : 'show'),
	set: (value: string) => {
		const shouldHide = value === 'hide';
		if (shouldHide !== props.hideUsername) {
			emit('toggle-username');
		}
	}
});

const passwordDisplay = computed({
	get: () => (props.hidePassword ? 'hide' : 'show'),
	set: (value: string) => {
		const shouldHide = value === 'hide';
		if (shouldHide !== props.hidePassword) {
			emit('toggle-password');
		}
	}
});

function toggleExpanded() {
	expanded.value = !expanded.value;
}

function openWindowSizeDialog() {
	expanded.value = false;
	windowSizeDialogOpen.value = true;
}

function openExportSettingsDialog() {
	expanded.value = false;
	exportSettingsDialogOpen.value = true;
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

function onNetworkSyncChange(value: boolean | null | undefined) {
	if (typeof value !== 'boolean') {
		return;
	}
	emit('update:network-sync-enabled', value);
}

async function onWindowSizeRestored() {
	await refresh();
}
</script>

<template>
	<div v-click-outside="() => (expanded = false)" class="function-panel">
		<v-expand-transition>
			<v-list v-show="expanded" density="compact" nav class="function-menu">
				<v-list-subheader class="function-menu-subheader">
					当前显示
				</v-list-subheader>
				<v-list-item class="function-menu-radio">
					<template #prepend>
						<v-icon icon="mdi-account" size="small" />
					</template>
					<div class="function-menu-radio-label">用户名</div>
					<v-radio-group
						v-model="usernameDisplay"
						density="compact"
						hide-details
						inline
						class="function-inline-radio"
					>
						<v-radio label="显示" value="show" />
						<v-radio label="隐藏" value="hide" />
					</v-radio-group>
				</v-list-item>
				<v-list-item class="function-menu-radio">
					<template #prepend>
						<v-icon icon="mdi-lock" size="small" />
					</template>
					<div class="function-menu-radio-label">密码</div>
					<v-radio-group
						v-model="passwordDisplay"
						density="compact"
						hide-details
						inline
						class="function-inline-radio"
					>
						<v-radio label="显示" value="show" />
						<v-radio label="隐藏" value="hide" />
					</v-radio-group>
				</v-list-item>
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
				<v-list-subheader class="function-menu-subheader">
					网络同步
				</v-list-subheader>
				<v-list-item title="联网同步区服数据" class="function-menu-switch">
					<template #append>
						<v-switch
							:model-value="networkSyncEnabled"
							density="compact"
							hide-details
							color="primary"
							@click.stop
							@update:model-value="onNetworkSyncChange"
						/>
					</template>
				</v-list-item>
				<v-list-item
					v-if="formattedLastSync"
					:title="`上次同步: ${formattedLastSync}`"
					class="function-menu-caption"
				/>
				<v-divider class="my-1" />
				<v-list-item
					title="导出设置"
					prepend-icon="mdi-export"
					@click="openExportSettingsDialog"
				/>
				<v-list-item
					title="窗口大小"
					prepend-icon="mdi-resize"
					@click="openWindowSizeDialog"
				/>
			</v-list>
		</v-expand-transition>

		<v-toolbar class="function-toolbar" density="comfortable">
			<v-btn size="small" icon @click="toggleExpanded">
				<v-icon icon="mdi-apps" />
			</v-btn>
		</v-toolbar>

		<WindowSizeDialog
			v-model="windowSizeDialogOpen"
			:width="width"
			:height="height"
			@restored="onWindowSizeRestored"
		/>
		<ExportSettingsDialog v-model="exportSettingsDialogOpen" />
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
	width: 240px;
	padding: 0;
	background: rgb(var(--v-theme-surface));
	box-shadow: 0 2px 8px rgba(0, 0, 0, 0.25);
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

.function-menu-radio {
	min-height: auto;
	padding-top: 4px;
	padding-bottom: 4px;
}

.function-menu-radio-label {
	font-size: 0.85rem;
	margin-bottom: 2px;
}

.function-inline-radio {
	margin-top: 0;
}

.function-inline-radio :deep(.v-selection-control-group) {
	gap: 4px;
}

.function-menu-caption :deep(.v-list-item-title) {
	font-size: 0.7rem;
	opacity: 0.7;
	white-space: normal;
}
</style>
