<script setup lang="ts">
import { ref } from 'vue';
import draggable from 'vuedraggable';
import AccountCard from './AccountCard.vue';
import { Account } from '../entity/account';

const accounts = defineModel<Account[]>({ required: true });

defineProps<{
	hideUsername: boolean;
	hidePassword: boolean;
	draggableEnabled: boolean;
}>();

const emit = defineEmits<{
	reorder: [ids: number[]];
	edit: [account: Account];
	like: [id: number, liked: boolean];
	delete: [id: number];
	copy: [text: string];
}>();

const orderBeforeDrag = ref<number[]>([]);

function onDragStart() {
	orderBeforeDrag.value = accounts.value
		.map((item) => item.id)
		.filter((id): id is number => id !== null);
}

function isSameOrder(left: number[], right: number[]): boolean {
	return left.length === right.length && left.every((id, index) => id === right[index]);
}

function onDragEnd() {
	const ids = accounts.value
		.map((item) => item.id)
		.filter((id): id is number => id !== null);

	if (isSameOrder(ids, orderBeforeDrag.value)) {
		return;
	}

	emit('reorder', ids);
}
</script>

<template>
	<div class="account-list">
		<div class="account-list-inner">
			<draggable
				v-model="accounts"
				item-key="id"
				tag="div"
				class="account-grid"
				:disabled="!draggableEnabled"
				filter=".no-drag"
				:prevent-on-filter="false"
				@start="onDragStart"
				@end="onDragEnd"
			>
				<template #item="{ element }">
					<div class="account-grid-item">
						<AccountCard
							:account="element"
							:hide-username="hideUsername"
							:hide-password="hidePassword"
							:draggable="draggableEnabled"
							@edit="emit('edit', $event)"
							@like="(id, liked) => emit('like', id, liked)"
							@delete="(id) => emit('delete', id)"
							@copy="emit('copy', $event)"
						/>
					</div>
				</template>
			</draggable>
		</div>
	</div>
</template>

<style scoped>
.account-list {
	height: 100%;
	overflow-y: auto;
	overflow-x: hidden;
	overscroll-behavior: contain;
	-webkit-overflow-scrolling: touch;
}

.account-list-inner {
	padding: 8px 10px;
}

.account-grid {
	display: flex;
	flex-wrap: wrap;
	margin: -4px;
}

.account-grid-item {
	box-sizing: border-box;
	width: 50%;
	padding: 4px;
}
</style>
