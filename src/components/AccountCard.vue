<script setup lang="ts">
import { encryptPassword, encryptUsername } from '../utils/mask';
import { Account } from '../entity/account';

defineProps<{
	account: Account;
	hideUsername: boolean;
	hidePassword: boolean;
	draggable: boolean;
}>();

const emit = defineEmits<{
	edit: [account: Account];
	like: [id: number, liked: boolean];
	delete: [id: number];
	copy: [text: string];
}>();
</script>

<template>
	<v-card
		color="black"
		theme="dark"
		density="compact"
		:class="['account-card', { 'account-card--draggable': draggable }]"
		@click="account.show = !account.show"
	>
		<div class="account-card-actions">
			<v-btn
				class="no-drag"
				:icon="account.liked ? 'mdi-heart' : 'mdi-heart-outline'"
				size="x-small"
				variant="tonal"
				@click.stop="account.id && emit('like', account.id, account.liked)"
			/>
			<v-btn
				class="no-drag"
				icon="mdi-delete"
				size="x-small"
				variant="tonal"
				@click.stop="account.id && emit('delete', account.id)"
			/>
		</div>

		<v-card-title class="account-card-title">
			<span
				class="account-card-name no-drag"
				@click.stop="emit('edit', account)"
			>
				{{ account.name }}
			</span>
		</v-card-title>

		<v-card-subtitle class="account-card-subtitle">
			<div v-if="draggable" class="drag-handle" @click.stop>
				<v-icon icon="mdi-drag" size="small" />
			</div>
			<span class="account-card-priority">
				优先级: {{ account.sequence }}
			</span>
		</v-card-subtitle>

		<v-card-text>
			<v-row dense>
				<v-col cols="12">
					<v-btn
						variant="tonal"
						width="100%"
						class="text-none no-drag"
						@click.stop
						@click="emit('copy', account.username)"
					>
						{{
							hideUsername
								? encryptUsername(account.username)
								: account.username
						}}
					</v-btn>
				</v-col>
				<v-col cols="12">
					<v-btn
						variant="tonal"
						width="100%"
						class="text-none no-drag"
						@click.stop
						@click="emit('copy', account.password)"
					>
						{{
							hidePassword
								? encryptPassword(account.password)
								: account.password
						}}
					</v-btn>
				</v-col>
			</v-row>
		</v-card-text>

		<v-expand-transition>
			<div v-show="account.show">
				<v-divider />
				<v-card-text>{{ account.description }}</v-card-text>
			</div>
		</v-expand-transition>
	</v-card>
</template>

<style scoped>
.account-card {
	position: relative;
}

.account-card--draggable {
	user-select: none;
}

.drag-handle {
	display: flex;
	align-items: center;
	justify-content: center;
	width: 24px;
	height: 24px;
	border-radius: 4px;
	color: rgba(255, 255, 255, 0.55);
	cursor: grab;
	touch-action: none;
	flex-shrink: 0;
}

.drag-handle:active {
	cursor: grabbing;
}

.drag-handle:hover {
	color: rgba(255, 255, 255, 0.85);
	background: rgba(255, 255, 255, 0.08);
}

.account-card-actions {
	position: absolute;
	top: 4px;
	left: 4px;
	right: 4px;
	z-index: 2;
	display: flex;
	justify-content: space-between;
	pointer-events: none;
	opacity: 0;
	transition: opacity 0.15s ease;
}

.account-card:hover .account-card-actions {
	opacity: 1;
}

.account-card-actions :deep(.v-btn) {
	pointer-events: auto;
}

.account-card-title {
	display: flex;
	justify-content: center;
	line-height: 1.2;
	padding-top: 8px;
	text-align: center;
}

.account-card-name {
	display: inline-block;
	max-width: 100%;
	padding: 6px 14px;
	border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
	border-radius: 999px;
	line-height: 1.3;
	font-size: 0.95rem;
	font-weight: 500;
	word-break: break-all;
	cursor: pointer;
	user-select: none;
	transition:
		border-color 0.15s ease,
		background-color 0.15s ease;
}

.account-card-name:hover {
	border-color: rgba(var(--v-theme-primary), 0.6);
	background: rgba(var(--v-theme-primary), 0.08);
}

.account-card-subtitle {
	display: flex;
	align-items: center;
	justify-content: center;
	gap: 6px;
	padding-top: 0;
	text-align: center;
	user-select: none;
}

.account-card-priority {
	line-height: 1.2;
}
</style>
