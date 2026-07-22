<script setup lang="ts">
import { Account } from '../entity/account';
import { Category } from '../entity/category';

const open = defineModel<boolean>({ required: true });
const account = defineModel<Account>('account', { required: true });

defineProps<{
	mode: 'insert' | 'update';
	categories: Category[];
	sequences: number[];
	likes: { value: boolean; title: string }[];
}>();

const emit = defineEmits<{
	quit: [];
	save: [];
}>();

const rules = {
	required: (v: string) => !!v || v !== '' || '该项必填!'
};
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
						label="标记账号为'喜欢'"
						:items="likes"
						item-title="title"
						item-value="value"
						density="compact"
						variant="solo-filled"
					/>
					<v-select
						v-model="account.sequence"
						label="选择账号优先级(用于排序)"
						:items="sequences"
						density="compact"
						variant="solo-filled"
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
</style>
