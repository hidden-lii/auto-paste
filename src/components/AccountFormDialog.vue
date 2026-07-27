<script setup lang="ts">
import { computed } from 'vue';
import { Account } from '../entity/account';
import { Category } from '../entity/category';
import { Jx3Server } from '../entity/jx3Server';
import { Role } from '../entity/role';
import {
	filterJx3ServerOption,
	sortJx3Servers
} from '../utils/jx3Server';

const open = defineModel<boolean>({ required: true });
const account = defineModel<Account>('account', { required: true });

const props = defineProps<{
	mode: 'insert' | 'update';
	categories: Category[];
	likes: { value: boolean; title: string }[];
	servers: Jx3Server[];
}>();

const emit = defineEmits<{
	quit: [];
	save: [];
}>();

const serverOptions = computed(() =>
	sortJx3Servers(props.servers).map((item) => ({
		title: `${item.server} (${item.status})`,
		value: item.server,
		zone: item.zone,
		status: item.status
	}))
);

const rules = {
	required: (v: string) => !!v || v !== '' || '该项必填!',
	numeric: (v: string | number) => {
		if (v === '' || v === null || v === undefined) {
			return '请输入优先级数字';
		}
		const num = Number(v);
		if (!Number.isInteger(num) || num < 1) {
			return '请输入正整数';
		}
		return true;
	}
};

function preventNonDigitInput(event: KeyboardEvent) {
	const allowedKeys = [
		'Backspace',
		'Delete',
		'Tab',
		'ArrowLeft',
		'ArrowRight',
		'Home',
		'End'
	];
	if (allowedKeys.includes(event.key) || event.ctrlKey || event.metaKey) {
		return;
	}
	if (!/^\d$/.test(event.key)) {
		event.preventDefault();
	}
}

function addRole() {
	if (!account.value.roles) {
		account.value.roles = [];
	}
	account.value.roles.push(new Role());
}

function removeRole(index: number) {
	account.value.roles.splice(index, 1);
}
</script>

<template>
	<v-dialog v-model="open" persistent>
		<v-card class="mx-12 dialog-card" density="compact">
			<v-card-title>
				{{ mode === 'insert' ? '添加账号信息' : '修改账号信息' }}
			</v-card-title>
			<v-card-text style="padding: 0 24px">
				<v-container style="padding: 0">
					<v-text-field
						v-model="account.name"
						label="名称*"
						variant="solo-filled"
						clearable
						density="compact"
						:rules="[rules.required]"
					/>
					<v-text-field
						v-model="account.username"
						label="账号*"
						variant="solo-filled"
						clearable
						density="compact"
						:rules="[rules.required]"
					/>
					<v-text-field
						v-model="account.password"
						label="密码*"
						variant="solo-filled"
						clearable
						density="compact"
						:rules="[rules.required]"
					/>
					<v-text-field
						v-model="account.description"
						label="描述"
						variant="solo-filled"
						density="compact"
						clearable
					/>
					<v-select
						v-model="account.liked"
						label="收藏状态"
						:items="likes"
						item-title="title"
						item-value="value"
						density="compact"
						variant="solo-filled"
					/>
					<div class="role-section">
						<div class="role-section-title">角色区服</div>
						<div
							v-for="(role, index) in account.roles"
							:key="index"
							class="role-row"
						>
							<v-text-field
								v-model="role.role_id"
								label="角色 ID"
								variant="solo-filled"
								density="compact"
								clearable
								class="role-field"
							/>
							<v-autocomplete
								v-model="role.server"
								label="区服*"
								:items="serverOptions"
								item-title="title"
								item-value="value"
								variant="solo-filled"
								density="compact"
								class="role-field"
								clearable
								:menu-props="{ maxHeight: 250 }"
								:custom-filter="filterJx3ServerOption"
							>
								<template #item="{ props: itemProps, item }">
									<v-list-item
										v-bind="itemProps"
										:subtitle="item.raw.zone"
									/>
								</template>
							</v-autocomplete>
							<v-btn
								icon="mdi-delete"
								size="small"
								variant="text"
								color="error"
								@click="removeRole(index)"
							/>
						</div>
						<v-btn
							variant="tonal"
							size="small"
							prepend-icon="mdi-plus"
							class="mt-1"
							@click="addRole"
						>
							添加角色
						</v-btn>
					</div>
					<v-text-field
						v-model.number="account.sequence"
						label="账号优先级(用于排序)"
						type="number"
						min="1"
						step="1"
						inputmode="numeric"
						density="compact"
						variant="solo-filled"
						:rules="[rules.numeric]"
						@keydown="preventNonDigitInput"
					/>
					<v-autocomplete
						v-model="account.account_category_ids"
						label="选择所属分组(可多选)"
						:items="categories"
						item-title="name"
						item-value="id"
						multiple
						chips
						closable-chips
						:menu-props="{ maxHeight: 250 }"
						density="compact"
						class="ac-input-no-padding"
						variant="solo-filled"
					>
						<template #item="{ props: itemProps, item }">
							<v-list-item
								v-bind="itemProps"
								prepend-icon="mdi-account-group"
								:title="item.raw.name"
							/>
						</template>
					</v-autocomplete>
				</v-container>
			</v-card-text>

			<v-divider style="margin-top: 10px" />

			<v-card-actions style="padding: 0 24px">
				<v-spacer />
				<v-btn color="error" variant="text" @click="emit('quit')">关闭</v-btn>
				<v-btn color="success" variant="text" @click="emit('save')">保存</v-btn>
			</v-card-actions>
		</v-card>
	</v-dialog>
</template>

<style scoped>
.ac-input-no-padding :deep(input) {
	padding: 0 !important;
	box-shadow: none !important;
}

.role-section {
	margin: 8px 0 12px;
}

.role-section-title {
	font-size: 0.85rem;
	margin-bottom: 8px;
	opacity: 0.85;
}

.role-row {
	display: flex;
	align-items: flex-start;
	gap: 8px;
	margin-bottom: 8px;
}

.role-field {
	flex: 1;
	min-width: 0;
}
</style>
