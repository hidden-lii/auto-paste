<script setup lang="ts">
import { encryptPassword, encryptUsername } from '../utils/mask';
import { Account } from '../entity/account';
import { Jx3Server } from '../entity/jx3Server';

defineProps<{
	account: Account;
	hideUsername: boolean;
	hidePassword: boolean;
	draggable: boolean;
	servers: Jx3Server[];
}>();

const emit = defineEmits<{
	edit: [account: Account];
	like: [id: number, liked: boolean];
	delete: [id: number];
	share: [account: Account];
	copy: [text: string];
}>();

function serverMeta(serverName: string, servers: Jx3Server[]) {
	return servers.find((item) => item.server === serverName);
}

function statusColor(status: string) {
	switch (status) {
		case '正常':
			return 'success';
		case '拥挤':
			return 'warning';
		case '爆满':
			return 'error';
		default:
			return 'default';
	}
}
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
			<v-menu location="bottom end">
				<template #activator="{ props: menuProps }">
					<v-btn
						v-bind="menuProps"
						class="no-drag"
						icon="mdi-dots-horizontal"
						size="x-small"
						variant="tonal"
						@click.stop
					/>
				</template>
				<v-list density="compact" nav>
					<v-list-item
						title="修改"
						prepend-icon="mdi-pencil"
						@click="emit('edit', account)"
					/>
					<v-list-item
						title="分享"
						prepend-icon="mdi-share-variant"
						@click="emit('share', account)"
					/>
					<v-list-item
						title="删除"
						prepend-icon="mdi-delete"
						base-color="error"
						@click="account.id && emit('delete', account.id)"
					/>
				</v-list>
			</v-menu>
		</div>

		<v-card-title class="account-card-title">
			<span class="account-card-name">
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
				<v-card-text>
					<div v-if="account.roles?.length" class="role-list">
						<div
							v-for="(role, index) in account.roles"
							:key="`${role.role_id}-${role.server}-${index}`"
							class="role-item no-drag"
							@click.stop
							@click="
								emit(
									'copy',
									`${role.role_id} @ ${role.server}`
								)
							"
						>
							<span>{{ role.role_id }}</span>
							<span class="role-separator">·</span>
							<span>
								{{
									serverMeta(role.server, servers)
										? `${serverMeta(role.server, servers)!.zone}·${role.server}`
										: role.server
								}}
							</span>
							<v-chip
								v-if="serverMeta(role.server, servers)"
								size="x-small"
								class="ms-1"
								:color="
									statusColor(
										serverMeta(role.server, servers)!.status
									)
								"
							>
								{{ serverMeta(role.server, servers)!.status }}
							</v-chip>
						</div>
					</div>
					<div v-else class="text-caption text-medium-emphasis">
						暂无角色
					</div>
					<div v-if="account.description" class="account-description">
						{{ account.description }}
					</div>
				</v-card-text>
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
	user-select: none;
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

.role-list {
	display: flex;
	flex-direction: column;
	gap: 6px;
	margin-bottom: 8px;
}

.role-item {
	display: flex;
	align-items: center;
	flex-wrap: wrap;
	gap: 4px;
	padding: 6px 8px;
	border-radius: 6px;
	background: rgba(255, 255, 255, 0.06);
	cursor: pointer;
	font-size: 0.85rem;
}

.role-item:hover {
	background: rgba(255, 255, 255, 0.1);
}

.role-separator {
	opacity: 0.5;
}

.account-description {
	margin-top: 8px;
	font-size: 0.85rem;
	opacity: 0.85;
}
</style>
