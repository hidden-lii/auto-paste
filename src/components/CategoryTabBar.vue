<script setup lang="ts">
import { nextTick, ref, watch } from 'vue';
import { Category } from '../entity/category';

const props = defineProps<{
	categories: Category[];
	selectedCategory: Category;
}>();

const emit = defineEmits<{
	'update:selectedCategory': [value: Category];
	'insert-category': [];
	'update-category': [];
	'delete-category': [];
}>();

const scrollRef = ref<HTMLElement | null>(null);
const tabRefs = ref<Record<string, HTMLElement | null>>({});
const contextMenuOpen = ref(false);
const contextMenuActivator = ref<HTMLElement | undefined>(undefined);
const contextMenuCategory = ref<Category | null>(null);

function tabKey(category: Category) {
	return String(category.id ?? 'all');
}

function setTabRef(category: Category, element: unknown) {
	tabRefs.value[tabKey(category)] =
		element instanceof HTMLElement ? element : null;
}

function canManageCategory(category: Category): boolean {
	return category.id !== null && category.id !== -1;
}

async function scrollActiveTabIntoView(behavior: ScrollBehavior = 'smooth') {
	await nextTick();
	const key = tabKey(props.selectedCategory);
	const tab = tabRefs.value[key];
	const container = scrollRef.value;
	if (!tab || !container) {
		return;
	}

	const tabLeft = tab.offsetLeft;
	const tabRight = tabLeft + tab.offsetWidth;
	const viewLeft = container.scrollLeft;
	const viewRight = viewLeft + container.clientWidth;

	if (tabLeft < viewLeft) {
		container.scrollTo({ left: tabLeft, behavior });
	} else if (tabRight > viewRight) {
		container.scrollTo({ left: tabRight - container.clientWidth, behavior });
	}
}

function clearTextSelection() {
	window.getSelection()?.removeAllRanges();
}

function onTabClick(category: Category) {
	emit('update:selectedCategory', category);
}

async function onTabContextMenu(event: MouseEvent, category: Category) {
	if (!canManageCategory(category)) {
		return;
	}
	event.preventDefault();
	clearTextSelection();
	contextMenuCategory.value = category;
	contextMenuActivator.value = event.currentTarget as HTMLElement;
	await nextTick();
	contextMenuOpen.value = true;
}

function onTabMouseDown(event: MouseEvent) {
	if (event.button === 2) {
		event.preventDefault();
	}
}

function onContextUpdateCategory() {
	if (!contextMenuCategory.value) {
		return;
	}
	emit('update:selectedCategory', contextMenuCategory.value);
	emit('update-category');
	contextMenuOpen.value = false;
}

function onContextDeleteCategory() {
	if (!contextMenuCategory.value) {
		return;
	}
	emit('update:selectedCategory', contextMenuCategory.value);
	emit('delete-category');
	contextMenuOpen.value = false;
}

watch(
	() => props.selectedCategory.id,
	() => {
		scrollActiveTabIntoView();
	},
	{ flush: 'post' }
);

watch(contextMenuOpen, (open) => {
	if (!open) {
		contextMenuActivator.value = undefined;
		contextMenuCategory.value = null;
	}
});
</script>

<template>
	<div class="category-tabs">
		<div ref="scrollRef" class="category-tabs__track">
			<button
				v-for="category in categories"
				:key="tabKey(category)"
				:ref="(element) => setTabRef(category, element)"
				type="button"
				class="category-tabs__tab"
				:class="{ 'is-active': selectedCategory.id === category.id }"
				@click="onTabClick(category)"
				@contextmenu="onTabContextMenu($event, category)"
				@mousedown="onTabMouseDown"
			>
				<span class="category-tabs__label">{{ category.name }}</span>
			</button>
			<button
				type="button"
				class="category-tabs__tab category-tabs__tab--add"
				@click="emit('insert-category')"
			>
				<v-icon icon="mdi-plus" size="small" />
			</button>
		</div>

		<v-menu
			v-if="contextMenuActivator"
			v-model="contextMenuOpen"
			:activator="contextMenuActivator"
			:close-on-content-click="true"
			location="bottom start"
		>
			<v-list density="compact" nav>
				<v-list-item
					title="修改分组"
					prepend-icon="mdi-account-multiple"
					@click="onContextUpdateCategory"
				/>
				<v-list-item
					title="删除分组"
					prepend-icon="mdi-account-multiple-remove"
					@click="onContextDeleteCategory"
				/>
			</v-list>
		</v-menu>
	</div>
</template>

<style scoped>
.category-tabs {
	height: 45px;
	flex-shrink: 0;
	overflow: hidden;
}

.category-tabs__track {
	display: flex;
	height: 100%;
	overflow-x: auto;
	overflow-y: hidden;
	scrollbar-width: none;
	-webkit-overflow-scrolling: touch;
	border-bottom: thin solid rgba(var(--v-theme-on-surface), 0.12);
}

.category-tabs__track::-webkit-scrollbar {
	display: none;
}

.category-tabs__tab {
	flex: 0 0 auto;
	min-width: 72px;
	max-width: 120px;
	box-sizing: border-box;
	display: flex;
	align-items: center;
	justify-content: center;
	height: 100%;
	padding: 0 8px;
	border: none;
	border-right: thin solid rgba(var(--v-theme-on-surface), 0.08);
	background: transparent;
	color: rgba(var(--v-theme-on-surface), 0.6);
	cursor: pointer;
	user-select: none;
	-webkit-user-select: none;
	transition:
		color 0.15s ease,
		background-color 0.15s ease;
}

.category-tabs__tab--add {
	flex: 0 0 45px;
	min-width: 45px;
	max-width: 45px;
	padding: 0;
	border-right: none;
}

.category-tabs__tab:hover:not(.is-active) {
	color: rgba(var(--v-theme-on-surface), 0.87);
	background: rgba(var(--v-theme-on-surface), 0.06);
}

.category-tabs__tab.is-active {
	position: relative;
	z-index: 1;
	background: rgb(var(--v-theme-surface));
	color: rgb(var(--v-theme-on-surface));
	border: thin solid rgba(var(--v-theme-on-surface), 0.12);
	border-bottom-color: rgb(var(--v-theme-surface));
	border-radius: 4px 4px 0 0;
	margin: 0 -1px;
}

.category-tabs__label {
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
	width: 100%;
	text-align: center;
	font-size: 0.875rem;
	line-height: 1.25rem;
	user-select: none;
	-webkit-user-select: none;
}
</style>
