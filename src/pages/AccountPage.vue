<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { writeText } from '@tauri-apps/api/clipboard';
import { appWindow } from '@tauri-apps/api/window';
import { Account } from '../entity/account';
import { Category } from '../entity/category';
import { useFeedback } from '../utils/feedback';
import {
	deleteAccount,
	insertAccount,
	queryAccountsByValue,
	queryAllAccounts,
	reorderAccounts,
	updateAccount,
	updateLike
} from '../api/account';
import {
	createCategory,
	deleteCategoryById,
	queryAllCategories,
	updateCategory
} from '../api/category';
import AccountSearchBar from '../components/AccountSearchBar.vue';
import CategoryAccountPanel from '../components/CategoryAccountPanel.vue';
import AccountFormDialog from '../components/AccountFormDialog.vue';
import CategoryFormDialog from '../components/CategoryFormDialog.vue';
import AppFooterToolbar from '../components/AppFooterToolbar.vue';
import AppFunctionPanel from '../components/AppFunctionPanel.vue';
import { loadDisplaySettings, persistDisplaySettings } from '../utils/display';

const { showConfirm, showSnackbar } = useFeedback();

const availableAccounts = ref<Account[]>([]);
const accounts = ref<Account[]>([]);
const types = ['全部', '名称', '账号'];
const categories = ref<Category[]>([new Category(-1, '全部')]);
const availableCategories = ref<Category[]>([]);
const selectedType = ref('全部');
const selectedCategory = ref(new Category(-1, '全部'));
const likeType = ref(0);
const dialogInsert = ref(false);
const dialogUpdate = ref(false);
const dialogInsertCategory = ref(false);
const dialogUpdateCategory = ref(false);
const likes = [
	{ value: true, title: '喜欢' },
	{ value: false, title: '普通' }
];
const keyword = ref('');
const insertAccountInfo = ref<Account>(new Account());
const updateAccountInfo = ref<Account>(new Account());
const updateAccountSnapshot = ref<Account | null>(null);
const insertCategoryInfo = ref<Category>(new Category(null, ''));
const hideUsername = ref(false);
const hidePassword = ref(true);
const defaultHideUsername = ref(false);
const defaultHidePassword = ref(true);
const alwaysOnTop = ref(false);

const draggableEnabled = computed(
	() =>
		!keyword.value &&
		(selectedCategory.value.id === -1 || selectedCategory.value.id === null) &&
		likeType.value === 0
);

function clearInsertAccountInfo() {
	insertAccountInfo.value = new Account();
}

function clearUpdateAccountInfo() {
	updateAccountInfo.value = new Account();
	updateAccountSnapshot.value = null;
}

function cloneAccountForEdit(account: Account): Account {
	return {
		...account,
		account_category_ids: [...(account.account_category_ids ?? [])]
	};
}

function isSameIdList(left: number[], right: number[]): boolean {
	if (left.length !== right.length) {
		return false;
	}
	const sortedLeft = [...left].sort((a, b) => a - b);
	const sortedRight = [...right].sort((a, b) => a - b);
	return sortedLeft.every((value, index) => value === sortedRight[index]);
}

function isAccountUnchanged(current: Account, snapshot: Account): boolean {
	return (
		current.name === snapshot.name &&
		current.username === snapshot.username &&
		current.password === snapshot.password &&
		(current.description ?? '') === (snapshot.description ?? '') &&
		current.liked === snapshot.liked &&
		current.sequence === snapshot.sequence &&
		isSameIdList(
			current.account_category_ids ?? [],
			snapshot.account_category_ids ?? []
		)
	);
}

function isCategoryUnchanged(current: Category, snapshot: Category): boolean {
	return (
		current.name === snapshot.name &&
		(current.sequence ?? null) === (snapshot.sequence ?? null) &&
		isSameIdList(current.account_ids ?? [], snapshot.account_ids ?? [])
	);
}

function closeUpdateDialog() {
	dialogUpdate.value = false;
	clearUpdateAccountInfo();
}

function clearInsertCategoryInfo() {
	insertCategoryInfo.value = new Category(null, '');
}

async function loadAllAccounts(
	showSnackbarMsg = true,
	updateAccounts = true
) {
	const result = await queryAllAccounts();
	availableAccounts.value = result;
	if (updateAccounts) {
		accounts.value = result;
	}
	if (showSnackbarMsg) {
		showSnackbar('查询成功', 'success');
	}
}

async function loadAccountsByValue(showSnackbarMsg = true) {
	const account = new Account();
	if (likeType.value === 1) {
		account.liked = true;
	} else if (likeType.value === 2) {
		account.liked = false;
	}
	if (selectedType.value === '全部') {
		account.name = keyword.value;
		account.username = keyword.value;
	} else if (selectedType.value === '名称') {
		account.name = keyword.value;
	} else if (selectedType.value === '账号') {
		account.username = keyword.value;
	}

	const result = await queryAccountsByValue(
		account,
		likeType.value > 0,
		selectedCategory.value.id
	);
	accounts.value = result;
	if (showSnackbarMsg) {
		showSnackbar('查询成功', 'success');
	}
}

async function loadAllCategories(showSnackbarMsg = true) {
	const result = await queryAllCategories();
	categories.value = [new Category(-1, '全部'), ...result];
	availableCategories.value = result;
	if (showSnackbarMsg) {
		showSnackbar('查询成功', 'success');
	}
}

async function refreshData() {
	await Promise.all([
		loadAllAccounts(true, true),
		loadAllCategories(true)
	]);
}

async function onCategoryChange(category: Category) {
	selectedCategory.value = category;
	await loadAccountsByValue(false);
}

async function onInsertCategoryQuit() {
	if (isCategoryUnchanged(insertCategoryInfo.value, new Category(null, ''))) {
		dialogInsertCategory.value = false;
		clearInsertCategoryInfo();
		return;
	}

	const quit = await showConfirm('分组还没有保存, 确认退出吗?', '添加分组');
	if (!quit) {
		return;
	}
	dialogInsertCategory.value = false;
	clearInsertCategoryInfo();
}

async function onInsertCategorySave() {
	const insertValue = { ...insertCategoryInfo.value };
	if (!insertValue.name) {
		showSnackbar('添加分组失败: 名称 为空', 'error');
		return;
	}

	try {
		const success = await createCategory(insertValue);
		if (success) {
			dialogInsertCategory.value = false;
			showSnackbar('添加分组成功', 'success');
			await loadAccountsByValue(false);
			await loadAllCategories(false);
			await loadAllAccounts(false, false);
			clearInsertCategoryInfo();
		} else {
			showSnackbar('添加分组失败: 未知原因', 'error');
		}
	} catch (err) {
		showSnackbar('添加分组失败: ' + JSON.stringify(err), 'error');
	}
}

async function onUpdateCategoryQuit() {
	const quit = await showConfirm('分组信息还没有保存, 确认退出吗?', '修改分组');
	if (!quit) {
		return;
	}
	dialogUpdateCategory.value = false;
}

async function onUpdateCategorySave() {
	const updateValue = { ...selectedCategory.value };
	if (!updateValue.name) {
		showSnackbar('修改分组失败: 名称 为空', 'error');
		return;
	}

	try {
		const success = await updateCategory(updateValue);
		if (success) {
			dialogUpdateCategory.value = false;
			showSnackbar('修改分组成功', 'success');
			await loadAccountsByValue(false);
			await loadAllCategories(false);
			await loadAllAccounts(false, false);
		} else {
			showSnackbar('修改分组失败: 未知原因', 'error');
		}
	} catch (err) {
		showSnackbar('修改分组失败: ' + JSON.stringify(err), 'error');
	}
}

async function toggleLiked() {
	likeType.value = (likeType.value + 1) % 3;
	await loadAccountsByValue();
}

async function onClickCopy(text: string) {
	if (!text) {
		showSnackbar('复制失败: 没有内容诶,你在复制什么?', 'error');
		return;
	}
	await writeText(text);
	showSnackbar('复制成功', 'success');
}

async function onClickLike(id: number, isLiked: boolean) {
	const action = isLiked ? '取消标记' : '标记';
	try {
		const success = await updateLike(id, !isLiked);
		if (success) {
			showSnackbar(action + '成功', 'success');
			await loadAccountsByValue(false);
			await loadAllAccounts(false, false);
		} else {
			showSnackbar(action + '失败: 未知原因', 'error');
		}
	} catch (err) {
		showSnackbar(action + '失败: ' + JSON.stringify(err), 'error');
	}
}

function openUpdateDialog(account: Account) {
	updateAccountInfo.value = cloneAccountForEdit(account);
	updateAccountSnapshot.value = cloneAccountForEdit(account);
	dialogUpdate.value = true;
}

async function onUpdateQuit() {
	if (
		updateAccountSnapshot.value &&
		isAccountUnchanged(updateAccountInfo.value, updateAccountSnapshot.value)
	) {
		closeUpdateDialog();
		return;
	}

	const confirmed = await showConfirm(
		'账号信息还没有保存, 确认退出吗?',
		'修改账号信息'
	);
	if (!confirmed) {
		return;
	}
	closeUpdateDialog();
}

async function onUpdateAccountSave() {
	const updateValue = { ...updateAccountInfo.value };
	if (!updateValue.id) {
		showSnackbar('修改失败: id 为空', 'error');
		return;
	}
	if (
		updateAccountSnapshot.value &&
		isAccountUnchanged(updateValue, updateAccountSnapshot.value)
	) {
		closeUpdateDialog();
		return;
	}
	if (!updateValue.name) {
		showSnackbar('修改失败: name 为空', 'error');
		return;
	}
	if (!updateValue.username) {
		showSnackbar('修改失败: username 为空', 'error');
		return;
	}
	if (!updateValue.password) {
		showSnackbar('修改失败: password 为空', 'error');
		return;
	}

	try {
		const success = await updateAccount(updateValue);
		if (success) {
			closeUpdateDialog();
			showSnackbar('修改成功', 'success');
			await loadAccountsByValue(false);
			await loadAllCategories(false);
			await loadAllAccounts(false, false);
		} else {
			showSnackbar('修改失败: 未知原因', 'error');
		}
	} catch (err) {
		showSnackbar('修改失败: ' + JSON.stringify(err), 'error');
	}
}

async function onInsertQuit() {
	if (isAccountUnchanged(insertAccountInfo.value, new Account())) {
		dialogInsert.value = false;
		clearInsertAccountInfo();
		return;
	}

	const quit = await showConfirm(
		'账号信息还没有保存, 确认退出吗?',
		'添加账号信息'
	);
	if (!quit) {
		return;
	}
	dialogInsert.value = false;
	clearInsertAccountInfo();
}

async function onInsertAccountSave() {
	const account = { ...insertAccountInfo.value };
	if (!account.name) {
		showSnackbar('添加账号信息失败: name 为空', 'error');
		return;
	}
	if (!account.username) {
		showSnackbar('添加账号信息失败: username 为空', 'error');
		return;
	}
	if (!account.password) {
		showSnackbar('添加账号信息失败: password 为空', 'error');
		return;
	}

	try {
		const success = await insertAccount(account);
		if (success) {
			dialogInsert.value = false;
			showSnackbar('添加账号信息成功', 'success');
			await loadAccountsByValue(false);
			await loadAllCategories(false);
			await loadAllAccounts(false, false);
			clearInsertAccountInfo();
		} else {
			showSnackbar('添加账号信息失败: 未知原因', 'error');
		}
	} catch (err) {
		showSnackbar('添加账号信息失败: ' + JSON.stringify(err), 'error');
	}
}

async function refresh() {
	keyword.value = '';
	likeType.value = 0;
	selectedType.value = '全部';
	selectedCategory.value = new Category(-1, '全部');
	await refreshData();
}

async function deleteOneAccount(id: number) {
	const deleteConfirm = await showConfirm(
		'这个操作不可回退, 确认删除此账号吗?',
		'删除账号提示'
	);
	if (!deleteConfirm) {
		return;
	}

	try {
		const success = await deleteAccount(id);
		if (success) {
			showSnackbar('删除成功', 'success');
			await loadAccountsByValue(false);
			await loadAllAccounts(false, false);
			await loadAllCategories(false);
		} else {
			showSnackbar('删除账号失败: 未知原因', 'error');
		}
	} catch (err) {
		showSnackbar('删除账号失败: ' + JSON.stringify(err), 'error');
	}
}

async function onDeleteCategory() {
	const id = selectedCategory.value.id;
	if (!id || id === -1) {
		showSnackbar('删除分组失败: id 为空', 'error');
		return;
	}

	const deleteConfirm = await showConfirm(
		'这个操作不可回退, 确认删除此分组吗?',
		'删除分组提示'
	);
	if (!deleteConfirm) {
		return;
	}

	try {
		const success = await deleteCategoryById(id);
		if (success) {
			showSnackbar('删除成功', 'success');
			await loadAllCategories(false);
			await loadAccountsByValue(false);
			await loadAllAccounts(false, false);
			selectedCategory.value = new Category(-1, '全部');
		} else {
			showSnackbar('删除分组失败: 未知原因', 'error');
		}
	} catch (err) {
		showSnackbar('删除分组失败: ' + JSON.stringify(err), 'error');
	}
}

async function onReorderAccounts(ids: number[]) {
	const previous = [...accounts.value];
	accounts.value = accounts.value.map((item, index) => ({
		...item,
		sequence: index + 1
	}));

	try {
		const success = await reorderAccounts(ids);
		if (success) {
			showSnackbar('排序已保存', 'success');
			await loadAllAccounts(false, false);
		} else {
			accounts.value = previous;
			showSnackbar('排序保存失败', 'error');
		}
	} catch (err) {
		accounts.value = previous;
		showSnackbar('排序保存失败: ' + JSON.stringify(err), 'error');
	}
}

function onHideUsernameClick() {
	hideUsername.value = !hideUsername.value;
	showSnackbar(
		hideUsername.value ? '关闭展示完整用户名' : '开启展示完整用户名',
		hideUsername.value ? 'warning' : 'success'
	);
}

async function onDefaultHideUsernameChange(value: boolean) {
	defaultHideUsername.value = value;
	hideUsername.value = value;
	await persistDisplaySettings(hideUsername.value, hidePassword.value);
	showSnackbar(
		value ? '默认隐藏用户名' : '默认显示用户名',
		'success'
	);
}

async function onDefaultHidePasswordChange(value: boolean) {
	defaultHidePassword.value = value;
	hidePassword.value = value;
	await persistDisplaySettings(hideUsername.value, hidePassword.value);
	showSnackbar(
		value ? '默认隐藏密码' : '默认显示密码',
		value ? 'warning' : 'success'
	);
}

async function toggleAlwaysOnTop() {
	alwaysOnTop.value = !alwaysOnTop.value;
	await appWindow.setAlwaysOnTop(alwaysOnTop.value);
	showSnackbar(
		alwaysOnTop.value ? '窗口已置顶' : '窗口置顶已关闭',
		alwaysOnTop.value ? 'success' : 'info'
	);
}

function onHidePasswordClick() {
	hidePassword.value = !hidePassword.value;
	showSnackbar(
		hidePassword.value ? '关闭展示明文密码' : '开启展示明文密码',
		hidePassword.value ? 'warning' : 'success'
	);
}

async function loadDisplayPreferences() {
	const settings = await loadDisplaySettings();
	hideUsername.value = settings.hideUsername;
	hidePassword.value = settings.hidePassword;
	defaultHideUsername.value = settings.hideUsername;
	defaultHidePassword.value = settings.hidePassword;
}

onMounted(async () => {
	await loadDisplayPreferences();
	await loadAllAccounts(false, true);
	await loadAllCategories(false);
});
</script>

<template>
	<div class="page-layout">
		<div class="account-page">
			<div class="search-bar">
				<AccountSearchBar
					:types="types"
					v-model:selected-type="selectedType"
					v-model:keyword="keyword"
					@search="loadAccountsByValue(true)"
				/>
			</div>

			<CategoryAccountPanel
				class="category-panel"
				v-model="accounts"
				:categories="categories"
				v-model:selected-category="selectedCategory"
				:hide-username="hideUsername"
				:hide-password="hidePassword"
				:draggable-enabled="draggableEnabled"
				@update:selected-category="onCategoryChange"
				@insert-category="dialogInsertCategory = true"
				@update-category="dialogUpdateCategory = true"
				@delete-category="onDeleteCategory"
				@reorder="onReorderAccounts"
				@edit="openUpdateDialog"
				@like="onClickLike"
				@delete="deleteOneAccount"
				@copy="onClickCopy"
			/>

			<AccountFormDialog
			v-model="dialogInsert"
			v-model:account="insertAccountInfo"
			mode="insert"
			:categories="availableCategories"
			:likes="likes"
			@quit="onInsertQuit"
			@save="onInsertAccountSave"
		/>

		<AccountFormDialog
			v-model="dialogUpdate"
			v-model:account="updateAccountInfo"
			mode="update"
			:categories="availableCategories"
			:likes="likes"
			@quit="onUpdateQuit"
			@save="onUpdateAccountSave"
		/>

		<CategoryFormDialog
			v-model="dialogInsertCategory"
			v-model:category="insertCategoryInfo"
			mode="insert"
			:accounts="availableAccounts"
			@quit="onInsertCategoryQuit"
			@save="onInsertCategorySave"
		/>

		<CategoryFormDialog
			v-model="dialogUpdateCategory"
			v-model:category="selectedCategory"
			mode="update"
			:accounts="availableAccounts"
			@quit="onUpdateCategoryQuit"
			@save="onUpdateCategorySave"
		/>
		</div>

		<div class="footer-row">
			<AppFunctionPanel
				:hide-username="hideUsername"
				:hide-password="hidePassword"
				:default-hide-username="defaultHideUsername"
				:default-hide-password="defaultHidePassword"
				@toggle-username="onHideUsernameClick"
				@toggle-password="onHidePasswordClick"
				@update:default-hide-username="onDefaultHideUsernameChange"
				@update:default-hide-password="onDefaultHidePasswordChange"
			/>
			<AppFooterToolbar
				class="footer-toolbar"
				:like-type="likeType"
				:always-on-top="alwaysOnTop"
				@insert-account="dialogInsert = true"
				@toggle-liked="toggleLiked"
				@toggle-always-on-top="toggleAlwaysOnTop"
				@refresh="refresh"
			/>
		</div>
	</div>
</template>

<style scoped>
.page-layout {
	display: flex;
	flex-direction: column;
	height: 100%;
	overflow: hidden;
}

.account-page {
	display: flex;
	flex-direction: column;
	flex: 1 1 auto;
	min-height: 0;
	overflow: hidden;
	width: 100%;
	text-align: left;
}

.search-bar {
	flex: 0 0 auto;
}

.category-panel {
	flex: 1 1 auto;
	min-height: 0;
	margin: 0 13px 13px;
}

.footer-row {
	display: flex;
	align-items: stretch;
	flex-shrink: 0;
	width: 100%;
	background: rgb(var(--v-theme-surface));
}

.footer-toolbar {
	flex: 1;
	min-width: 0;
}

.footer-toolbar :deep(.v-toolbar) {
	background: transparent;
}
</style>
