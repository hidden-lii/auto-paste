<script setup lang="ts">
import { Account } from '../entity/account';
import { Category } from '../entity/category';
import { Jx3Server } from '../entity/jx3Server';
import AccountCardList from './AccountCardList.vue';
import CategoryTabBar from './CategoryTabBar.vue';

const accounts = defineModel<Account[]>({ required: true });
const selectedCategory = defineModel<Category>('selectedCategory', { required: true });

defineProps<{
	categories: Category[];
	hideUsername: boolean;
	hidePassword: boolean;
	draggableEnabled: boolean;
	servers: Jx3Server[];
}>();

const emit = defineEmits<{
	'insert-category': [];
	'update-category': [];
	'delete-category': [];
	reorder: [ids: number[]];
	edit: [account: Account];
	like: [id: number, liked: boolean];
	delete: [id: number];
	share: [account: Account];
	copy: [text: string];
}>();
</script>

<template>
	<div class="category-account-panel">
		<CategoryTabBar
			:categories="categories"
			v-model:selected-category="selectedCategory"
			@insert-category="emit('insert-category')"
			@update-category="emit('update-category')"
			@delete-category="emit('delete-category')"
		/>
		<div class="category-account-panel__content">
			<AccountCardList
				v-model="accounts"
				:hide-username="hideUsername"
				:hide-password="hidePassword"
				:draggable-enabled="draggableEnabled"
				:servers="servers"
				@reorder="emit('reorder', $event)"
				@edit="emit('edit', $event)"
				@like="(id, liked) => emit('like', id, liked)"
				@delete="(id) => emit('delete', id)"
				@share="emit('share', $event)"
				@copy="emit('copy', $event)"
			/>
		</div>
	</div>
</template>

<style scoped>
.category-account-panel {
	display: flex;
	flex-direction: column;
	flex: 1 1 auto;
	min-height: 0;
	border: thin solid rgba(var(--v-theme-on-surface), 0.12);
	border-radius: 4px;
	overflow: hidden;
}

.category-account-panel__content {
	flex: 1 1 auto;
	min-height: 0;
	background: rgb(var(--v-theme-surface));
}
</style>
