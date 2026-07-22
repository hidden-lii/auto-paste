<script setup lang="ts">
import { Account } from '../entity/account';
import { Category } from '../entity/category';

const open = defineModel<boolean>({ required: true });
const category = defineModel<Category>('category', { required: true });

defineProps<{
	mode: 'insert' | 'update';
	accounts: Account[];
	sequences: number[];
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
				{{ mode === 'insert' ? '添加分组' : '修改分组' }}
			</v-card-title>
			<v-card-text style="padding: 0 24px">
				<v-container style="padding: 0">
					<v-text-field
						v-model="category.name"
						label="名称*"
						variant="solo-filled"
						clearable
						density="compact"
						:rules="[rules.required]"
					/>
					<v-select
						v-model="category.sequence"
						label="选择分组优先级(用于排序)"
						:items="sequences"
						:menu-props="{ maxHeight: 250 }"
						density="compact"
						variant="solo-filled"
					/>
					<v-autocomplete
						v-model="category.account_ids"
						label="关联的账号"
						:items="accounts"
						item-title="username"
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
								prepend-icon="mdi-account"
								:title="item.raw.username"
								:subtitle="item.raw.name"
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
