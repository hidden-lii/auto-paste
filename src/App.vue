<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { writeText } from '@tauri-apps/api/clipboard';
import { Account } from './account';
import { useConfirm, useSnackbar } from 'vuetify-use-dialog';
import { Category } from './category';

const accounts = ref<Account[]>([]);
const types = ref(['全部', '名称', '账号']);
const categories = ref<Category[]>([new Category(-1, '全部')]);
const available_categories = ref<Category[]>([]);
const selected_type = ref('全部');
const selected_category = ref(new Category(-1, '全部'));
const liked = ref(false);
const like_type = ref(0);
const dialog_insert = ref(false);
const dialog_update = ref(false);
const dialog_insert_category = ref(false);
const dialog_update_category = ref(false);
const sequences = ref(Array.from({ length: 10 }, (k, v) => v + 1));
const likes = ref([
	{ value: true, title: '喜欢' },
	{ value: false, title: '普通' }
]);
const keyword = ref('');
const show_category_select = ref(true);
const rules = ref({
	required: (v: string) => !!v || v !== '' || '该项必填!'
});
const insert_account_info = ref<Account>(new Account());
const update_account_info = ref<Account>(new Account());
const insert_category_info = ref<Category>(new Category(null, ''));
const update_category_info = ref<Category>(new Category(null, ''));
const createConfirm = useConfirm();
const createSnackbar = useSnackbar();

function clear_insert_account_info() {
	insert_account_info.value = new Account();
}

function clear_update_account_info() {
	update_account_info.value = new Account();
}

function clear_insert_category_info() {
	insert_category_info.value = new Category(null, '');
}

function clear_update_category_info() {
	update_category_info.value = new Category(null, '');
}

function toggle_show_category_select() {
	show_category_select.value = !show_category_select.value;
}

async function on_insert_category_quit() {
	const quit = await createConfirm({
		content: '分组还没有保存, 确认退出吗?',
		title: '添加分组',
		confirmationText: '确认',
		cancellationText: '取消'
	});

	if (!quit) {
		return;
	}

	dialog_insert_category.value = false;
	clear_insert_category_info();
}

async function on_insert_category_save() {
	let insert_value = insert_category_info.value;

	if (!insert_value.name || insert_value.name === '') {
		common_snacker_bar('添加分组失败: 名称 为空', 'error');
		return;
	}
	await invoke('create_category', { category: insert_value })
		.then((res) => {
			if (!!res && typeof res === 'boolean' && res) {
				dialog_insert_category.value = false;
				common_snacker_bar('添加分组成功', 'success');
				// 添加分组时,可能会增加分组和用户的关系,所以用户信息也要重新查询一遍
				query_accounts_by_value(false);
				query_all_categories(false);
				clear_insert_category_info();
			} else {
				common_snacker_bar('添加分组失败: 未知原因', 'error');
			}
		})
		.catch((err: unknown) => {
			common_snacker_bar('添加分组失败: ' + JSON.stringify(err), 'error');
		});
}

async function on_update_category_quit() {
	const quit = await createConfirm({
		content: '分组还没有保存, 确认退出吗?',
		title: '修改分组',
		confirmationText: '确认',
		cancellationText: '取消'
	});

	if (!quit) {
		return;
	}

	dialog_update_category.value = false;
	clear_update_category_info();
}

async function on_update_category_save() {
	let update_value = update_category_info.value;

	if (!update_value.name || update_value.name === '') {
		common_snacker_bar('修改分组失败: 名称 为空', 'error');
		return;
	}
	await invoke('update_category', { category: update_value })
		.then((res) => {
			if (!!res && typeof res === 'boolean' && res) {
				dialog_update_category.value = false;
				common_snacker_bar('修改分组成功', 'success');
				// 修改分组时,可能会变更分组和用户的关系,所以用户信息也要重新查询一遍
				query_accounts_by_value(false);
				query_all_categories(false);
				clear_update_category_info();
			} else {
				common_snacker_bar('修改分组失败: 未知原因', 'error');
			}
		})
		.catch((err: unknown) => {
			common_snacker_bar('修改分组失败: ' + JSON.stringify(err), 'error');
		});
}

async function toggle_liked() {
	like_type.value = (like_type.value + 1) % 3;
	liked.value = like_type.value === 1;
	await query_accounts_by_value();
}

async function on_click_copy(text: string | number | null) {
	if (!text) {
		common_snacker_bar('复制失败: 没有内容诶,你在复制什么?', 'error');
		return;
	}
	await writeText(text.toString());
	common_snacker_bar('复制成功', 'success');
}

async function on_click_like(id: number | null, liked: boolean) {
	const is_like = liked ? '取消标记' : '标记';
	if (!id) {
		common_snacker_bar(is_like + '失败: 需要指定id', 'error');
		return;
	}
	await invoke('update_like', { id: id, liked: !liked })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				common_snacker_bar(is_like + '成功', 'success');
				query_accounts_by_value(false);
			} else {
				common_snacker_bar(is_like + '失败: 未知原因', 'error');
			}
		})
		.catch((err: unknown) => {
			common_snacker_bar(is_like + '失败: ' + JSON.stringify(err), 'error');
		});
}

function toggle_update(account: Account) {
	update_account_info.value = account;

	dialog_update.value = true;
}

async function on_update_quit() {
	const isConfirmed = await createConfirm({
		content: '变更未做保存,确认退出吗?',
		title: '修改账号信息',
		confirmationText: '确认',
		cancellationText: '取消'
	});

	if (!isConfirmed) {
		return;
	}

	dialog_update.value = false;
	clear_update_account_info();
}

async function on_update_account_save() {
	let update_value = update_account_info.value;
	if (!update_value.id) {
		common_snacker_bar('修改失败: id 为空', 'error');
		return;
	}
	if (!update_value.name || update_value.name === '') {
		common_snacker_bar('修改失败: name 为空', 'error');
		return;
	}
	if (!update_value.username || update_value.username === '') {
		common_snacker_bar('修改失败: username 为空', 'error');
		return;
	}
	if (!update_value.password) {
		common_snacker_bar('修改失败: password 为空', 'error');
		return;
	}
	await invoke('update_account', { account: update_value })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				dialog_update.value = false;
				common_snacker_bar('修改成功', 'success');
				query_accounts_by_value(false);
				clear_update_account_info();
			} else {
				common_snacker_bar('修改失败: 未知原因', 'error');
			}
		})
		.catch((err: unknown) => {
			common_snacker_bar('修改失败: ' + JSON.stringify(err), 'error');
		});
}

async function on_insert_account_quit() {
	const quit = await createConfirm({
		content: '账号信息还没有保存, 确认退出吗?',
		title: '添加账号信息',
		confirmationText: '确认',
		cancellationText: '取消'
	});

	if (!quit) {
		return;
	}

	dialog_insert.value = false;
	clear_insert_account_info();
}

async function on_insert_account_save() {
	let account = insert_account_info.value;
	if (!account.name || account.name === '') {
		common_snacker_bar('添加账号信息失败: name 为空', 'error');
		return;
	}
	if (!account.username || account.username === '') {
		common_snacker_bar('添加账号信息失败: username 为空', 'error');
		return;
	}
	if (!account.password) {
		common_snacker_bar('添加账号信息失败: password 为空', 'error');
		return;
	}
	await invoke('insert_account', { account: account })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				dialog_insert.value = false;
				common_snacker_bar('添加账号信息成功', 'success');
				query_accounts_by_value(false);
				clear_insert_account_info();
			} else {
				common_snacker_bar('添加账号信息失败: 未知原因', 'error');
			}
		})
		.catch((err: unknown) => {
			common_snacker_bar('添加账号信息失败: ' + JSON.stringify(err), 'error');
		});
}

async function refresh() {
	keyword.value = '';
	like_type.value = 0;
	liked.value = false;
	selected_type.value = '全部';
	selected_category.value = new Category(-1, '全部');
	// 因为清空了所有选项,所以这里查全部就可以
	await query_all_accounts();
}

async function query_all_accounts(show_snackbar: boolean = true) {
	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	await invoke('query_all_accounts').then((res) => {
		// 在这里处理返回的 Account[] 类型数据
		if (!!res && res instanceof Array) {
			accounts.value = res as Account[];

			if (show_snackbar) {
				common_snacker_bar('查询成功', 'success');
			}
		}
	});
}

async function query_accounts_by_value(show_snackbar: boolean = true) {
	let account = new Account();
	account.name = '';
	account.username = '';
	account.password = '';
	account.liked = liked.value;
	if (selected_type.value === '全部') {
		account.name = keyword.value;
		account.username = keyword.value;
	} else if (selected_type.value === '名称') {
		account.name = keyword.value;
	} else if (selected_type.value === '账号') {
		account.username = keyword.value;
	}
	await invoke('query_accounts_by_value', {
		account: account,
		withLiked: like_type.value > 0,
		// watch保证了这个值不为null
		categoryId: selected_category.value.id
	}).then((res) => {
		// 在这里处理返回的 Account[] 类型数据
		if (!!res && res instanceof Array) {
			accounts.value = res as Account[];

			if (show_snackbar) {
				common_snacker_bar('查询成功', 'success');
			}
		}
	});
}

async function delete_one_account(id: number | null) {
	if (!id) {
		common_snacker_bar('删除账号失败: id 为空', 'error');
		return;
	}
	const delete_confirm = await createConfirm({
		content: '这个操作不可回退, 确认删除此账号吗?',
		title: '删除账号提示',
		confirmationText: '确认',
		cancellationText: '取消'
	});

	if (!delete_confirm) {
		return;
	}

	await invoke('delete_account', { id: id })
		.then((res) => {
			if (!!res && typeof res === 'boolean' && res) {
				common_snacker_bar('删除成功', 'success');
				query_accounts_by_value(false);
			} else {
				common_snacker_bar('删除账号失败: 未知原因', 'error');
			}
		})
		.catch((err) => {
			common_snacker_bar('删除账号失败: ' + JSON.stringify(err), 'error');
		});
}

function common_snacker_bar(text: string, color: string) {
	createSnackbar({
		text: text,
		snackbarProps: {
			timeout: 1000,
			color: color,
			minWidth: 'fit-content',
			maxWidth: 'fit-content'
		},
		showCloseButton: false
	});
}

async function query_all_categories(show_snackbar: boolean = true) {
	await invoke('query_all_category').then((res) => {
		// 在这里处理返回的 Account[] 类型数据
		if (!!res && res instanceof Array) {
			categories.value = [new Category(-1, '全部'), ...(res as Category[])];
			available_categories.value = res as Category[];

			if (show_snackbar) {
				common_snacker_bar('查询分组成功', 'success');
			}
		}
	});
}

// 判断是否显示清除按钮
const shouldShowClearButton = computed(() => {
	return (
		selected_category.value !== null &&
		selected_category.value.id !== null &&
		selected_category.value.id !== -1
	);
});

// 监听 selected_category 变化
watch(selected_category, (newValue, oldValue) => {
	if (newValue === null && oldValue === null) {
		selected_category.value = new Category(-1, '全部');
	} else if (newValue === null) {
		selected_category.value = oldValue;
	} else {
		selected_category.value = newValue;
	}
	update_category_info.value = selected_category.value;
	query_accounts_by_value(true);
});

async function on_delete_category() {
	// 如果不加这个的话,下面拿不到正确的value,暂时还不清楚原因...
	createSnackbar({ snackbarProps: { timeout: 0 }, showCloseButton: false });
	if (selected_category.value === null) {
		common_snacker_bar('删除分组失败: 未获取到选中值', 'error');
		return;
	}
	let id = selected_category.value.id;
	if (!id || id === -1) {
		common_snacker_bar('删除分组失败: id 为空', 'error');
		return;
	}
	const delete_confirm = await createConfirm({
		content: '这个操作不可回退, 确认删除此分组吗?',
		title: '删除分组提示',
		confirmationText: '确认',
		cancellationText: '取消'
	});
	if (!delete_confirm) {
		return;
	}
	await invoke('delete_category_by_id', { id: id })
		.then((res) => {
			if (!!res && typeof res === 'boolean' && res) {
				common_snacker_bar('删除成功', 'success');
				query_all_categories(false);
				selected_category.value = new Category(-1, '全部');
			} else {
				common_snacker_bar('删除分组失败: 未知原因', 'error');
			}
		})
		.catch((err) => {
			common_snacker_bar('删除分组失败: ' + JSON.stringify(err), 'error');
		});
}

// 禁止右键菜单的出现
function disableContextMenu(event: { preventDefault: () => void }) {
	event.preventDefault();
}

onMounted(() => {
	query_all_accounts(false);
	query_all_categories(false);
});

// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
</script>

<template>
	<v-app @contextmenu="disableContextMenu">
		<v-main>
			<div class="container">
				<v-container>
					<v-row>
						<v-col cols="4">
							<v-select
								v-model="selected_type"
								:items="types"
								label="搜索类型"
								variant="solo-filled"
								hide-details
								density="compact">
							</v-select>
						</v-col>

						<v-col cols="7">
							<v-text-field
								label="输入关键词"
								variant="solo-filled"
								v-model="keyword"
								hide-details
								density="compact">
								<template #append-inner>
									<v-btn icon size="small" @click="query_accounts_by_value">
										<v-icon icon="mdi-magnify"></v-icon>
									</v-btn>
								</template>
							</v-text-field>
						</v-col>

						<v-col cols="1" class="col-align-center">
							<v-btn
								:icon="
									show_category_select ? 'mdi-chevron-up' : 'mdi-chevron-down'
								"
								size="x-small"
								@click="toggle_show_category_select">
							</v-btn>
						</v-col>
					</v-row>
					<v-expand-transition>
						<v-row v-show="show_category_select">
							<v-col cols="8">
								<v-select
									v-model="selected_category"
									:items="categories"
									item-title="name"
									item-value="id"
									return-object
									label="账号分组"
									variant="solo-filled"
									hide-details
									:clearable="shouldShowClearButton"
									:menu-props="{ maxHeight: 250 }"
									@click:clear="on_delete_category"
									density="compact">
								</v-select>
							</v-col>
							<v-col cols="2" class="col-align-center">
								<v-btn
									variant="tonal"
									@click="dialog_insert_category = true"
									size="small"
									icon>
									<v-icon icon="mdi-account-multiple-plus"></v-icon>
								</v-btn>
							</v-col>
							<v-col cols="2" class="col-align-center">
								<v-btn
									variant="tonal"
									@click="dialog_update_category = true"
									:disabled="
										update_category_info.id === null ||
										update_category_info.id === -1
									"
									size="small"
									icon>
									<v-icon icon="mdi-account-edit"></v-icon>
								</v-btn>
							</v-col>
							<!-- <v-col class="col-align-center">
								<v-btn variant="tonal">删除分组</v-btn>
							</v-col> -->
						</v-row>
					</v-expand-transition>
				</v-container>

				<v-divider></v-divider>

				<v-container class="scroll-container">
					<v-row dense>
						<template v-for="(item, index) in accounts" :key="index">
							<v-col cols="6">
								<v-hover v-slot="{ isHovering, props }">
									<v-card v-bind="props" density="compact">
										<v-sheet class="edit-button" v-if="isHovering">
											<v-btn
												:icon="item.liked ? 'mdi-heart' : 'mdi-heart-outline'"
												size="x-small"
												variant="tonal"
												@click="on_click_like(item.id, item.liked)">
											</v-btn>
										</v-sheet>
										<v-sheet class="close-button" v-if="isHovering">
											<v-btn
												icon="mdi-delete"
												size="x-small"
												variant="tonal"
												@click="delete_one_account(item.id)">
											</v-btn>
										</v-sheet>
										<v-card-title>
											<v-chip
												class="ma-2"
												variant="outlined"
												@click="toggle_update(item)">
												{{ item.name }}
											</v-chip>
											<!-- <v-btn
														:icon="
															item.show ? 'mdi-chevron-up' : 'mdi-chevron-down'
														"
														size="x-small"
														variant="tonal"
														@click="item.show = !item.show">
													</v-btn> -->
										</v-card-title>

										<v-card-subtitle>
											优先级: {{ item.sequence }}
										</v-card-subtitle>

										<v-card-text>
											<v-row dense>
												<v-col cols="12">
													<v-btn
														variant="tonal"
														@click="on_click_copy(item.username)"
														width="100%"
														class="text-none">
														{{ item.username }}
													</v-btn>
												</v-col>
												<v-col cols="12">
													<v-btn
														variant="tonal"
														@click="on_click_copy(item.password)"
														width="100%"
														class="text-none">
														{{ item.password }}
													</v-btn>
												</v-col>
											</v-row>
										</v-card-text>
										<v-expand-transition>
											<div v-show="item.show">
												<v-divider></v-divider>
												<v-card-text>
													{{ item.description }}
												</v-card-text>
											</div>
										</v-expand-transition>
									</v-card>
								</v-hover>
							</v-col>
						</template>
					</v-row>
				</v-container>

				<!-- #update account dialog -->
				<v-dialog v-model="dialog_update" persistent>
					<v-card class="mx-12 dialog-card" density="compact">
						<v-card-title>修改账号信息</v-card-title>
						<v-card-text style="padding: 0 24px">
							<v-container style="padding: 0">
								<v-text-field
									v-model="update_account_info.name"
									label="名称*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="update_account_info.username"
									label="账号*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="update_account_info.password"
									label="密码*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="update_account_info.description"
									label="描述"
									variant="solo-filled"
									density="compact"
									clearable>
								</v-text-field>

								<v-select
									v-model="update_account_info.liked"
									label="标记账号为'喜欢'"
									:items="likes"
									item-title="title"
									item-value="value"
									density="compact"
									variant="solo-filled">
								</v-select>

								<v-select
									v-model="update_account_info.sequence"
									label="选择账号优先级(用于排序,暂未实现)"
									:items="sequences"
									density="compact"
									variant="solo-filled">
								</v-select>

								<v-autocomplete
									v-model="update_account_info.account_category_ids"
									label="选择所属分组(可多选)"
									:items="available_categories"
									item-title="name"
									item-value="id"
									multiple
									chips
									closable-chips
									:menu-props="{ maxHeight: 250 }"
									density="compact"
									class="ac-input-no-padding"
									variant="solo-filled">
									<template v-slot:item="{ props, item }">
										<v-list-item
											v-bind="props"
											prepend-icon="mdi-account-group"
											:title="item.raw.name"></v-list-item>
									</template>
								</v-autocomplete>
							</v-container>
						</v-card-text>

						<v-divider style="margin-top: 10px"></v-divider>

						<v-card-actions style="padding: 0 24px">
							<v-spacer></v-spacer>
							<v-btn color="error" variant="text" @click="on_update_quit">
								关闭
							</v-btn>
							<v-btn
								color="success"
								variant="text"
								@click="on_update_account_save">
								保存
							</v-btn>
						</v-card-actions>
					</v-card>
				</v-dialog>

				<!-- #insert account dialog -->
				<v-dialog v-model="dialog_insert" persistent>
					<v-card class="mx-12 dialog-card" density="compact">
						<v-card-title>添加账号信息</v-card-title>
						<v-card-text style="padding: 0 24px">
							<v-container style="padding: 0">
								<v-text-field
									v-model="insert_account_info.name"
									label="名称*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="insert_account_info.username"
									label="账号*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="insert_account_info.password"
									label="密码*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="insert_account_info.description"
									label="描述"
									variant="solo-filled"
									density="compact"
									clearable>
								</v-text-field>

								<v-select
									v-model="insert_account_info.liked"
									label="标记账号为'喜欢'"
									:items="likes"
									item-title="title"
									item-value="value"
									density="compact"
									variant="solo-filled">
								</v-select>

								<v-select
									v-model="insert_account_info.sequence"
									label="选择账号优先级(用于排序,暂未实现)"
									:items="sequences"
									density="compact"
									variant="solo-filled">
								</v-select>

								<v-autocomplete
									v-model="insert_account_info.account_category_ids"
									label="选择所属分组(可多选)"
									:items="available_categories"
									item-title="name"
									item-value="id"
									multiple
									chips
									closable-chips
									:menu-props="{ maxHeight: 250 }"
									density="compact"
									class="ac-input-no-padding"
									variant="solo-filled">
									<template v-slot:item="{ props, item }">
										<v-list-item
											v-bind="props"
											prepend-icon="mdi-account-group"
											:title="item.raw.name"></v-list-item>
									</template>
								</v-autocomplete>
							</v-container>
						</v-card-text>

						<v-divider style="margin-top: 10px"></v-divider>

						<v-card-actions style="padding: 0 24px">
							<v-spacer></v-spacer>
							<v-btn
								color="error"
								variant="text"
								@click="on_insert_account_quit">
								关闭
							</v-btn>
							<v-btn
								color="success"
								variant="text"
								@click="on_insert_account_save">
								保存
							</v-btn>
						</v-card-actions>
					</v-card>
				</v-dialog>

				<!-- #insert categroy dialog -->
				<v-dialog v-model="dialog_insert_category" persistent>
					<v-card class="mx-12 dialog-card" density="compact">
						<v-card-title>添加分组</v-card-title>
						<v-card-text style="padding: 0 24px">
							<v-container style="padding: 0">
								<v-text-field
									v-model="insert_category_info.name"
									label="名称*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-select
									v-model="insert_category_info.sequence"
									label="选择分组优先级(用于排序,暂未实现)"
									:items="sequences"
									:menu-props="{ maxHeight: 250 }"
									density="compact"
									variant="solo-filled">
								</v-select>

								<v-autocomplete
									v-model="insert_category_info.account_ids"
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
									variant="solo-filled">
									<template v-slot:item="{ props, item }">
										<v-list-item
											v-bind="props"
											prepend-icon="mdi-account"
											:title="item.raw.username"
											:subtitle="item.raw.name"></v-list-item>
									</template>
								</v-autocomplete>
							</v-container>
						</v-card-text>

						<v-divider style="margin-top: 10px"></v-divider>

						<v-card-actions style="padding: 0 24px">
							<v-spacer></v-spacer>
							<v-btn
								color="error"
								variant="text"
								@click="on_insert_category_quit">
								关闭
							</v-btn>
							<v-btn
								color="success"
								variant="text"
								@click="on_insert_category_save">
								保存
							</v-btn>
						</v-card-actions>
					</v-card>
				</v-dialog>

				<!-- #update category dialog -->
				<v-dialog v-model="dialog_update_category" persistent>
					<v-card class="mx-12 dialog-card" density="compact">
						<v-card-title>添加分组</v-card-title>
						<v-card-text style="padding: 0 24px">
							<v-container style="padding: 0">
								<v-text-field
									style="padding: 0"
									v-model="update_category_info.name"
									label="名称*"
									variant="solo-filled"
									clearable
									density="compact"
									:rules="[rules.required]">
								</v-text-field>

								<v-select
									v-model="update_category_info.sequence"
									label="选择分组优先级(用于排序,暂未实现)"
									:items="sequences"
									:menu-props="{ maxHeight: 250 }"
									density="compact"
									variant="solo-filled">
								</v-select>

								<v-autocomplete
									v-model="update_category_info.account_ids"
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
									variant="solo-filled">
									<template v-slot:item="{ props, item }">
										<v-list-item
											v-bind="props"
											prepend-icon="mdi-account"
											:title="item.raw.username"
											:subtitle="item.raw.name"></v-list-item>
									</template>
								</v-autocomplete>
							</v-container>
						</v-card-text>

						<v-divider style="margin-top: 10px"></v-divider>

						<v-card-actions style="padding: 0 24px">
							<v-spacer></v-spacer>
							<v-btn
								color="error"
								variant="text"
								@click="on_update_category_quit">
								关闭
							</v-btn>
							<v-btn
								color="success"
								variant="text"
								@click="on_update_category_save">
								保存
							</v-btn>
						</v-card-actions>
					</v-card>
				</v-dialog>
			</div>
		</v-main>

		<v-toolbar density="comfortable">
			<template v-slot:prepend>
				<v-btn size="small" icon @click="dialog_insert = true">
					<v-icon icon="mdi-account-plus"></v-icon>
				</v-btn>
			</template>
			<v-btn
				:icon="
					like_type === 0
						? 'mdi-heart-off'
						: like_type === 1
						? 'mdi-heart'
						: 'mdi-heart-outline'
				"
				class="ms-5"
				size="small"
				@click="toggle_liked"></v-btn>
			<v-divider
				class="mx-3 align-self-center"
				length="24"
				thickness="2"
				vertical></v-divider>
			<v-btn icon="mdi-reload" @click="refresh" size="small"></v-btn>
		</v-toolbar>
	</v-app>
</template>

<style scoped>
.container {
	display: flex;
	flex-direction: column;
	height: calc(100vh - 55px);
	padding-top: 0;
}

.scroll-container {
	flex: 1;
	overflow-y: auto;
}

.col-align-center {
	align-self: center;
	padding: 0;
	margin: 0;
}

.close-button {
	position: absolute;
	top: 0;
	right: 0;
	z-index: 1;
	cursor: pointer;
	border-radius: 50%;
}

.edit-button {
	position: absolute;
	z-index: 1;
	cursor: pointer;
	border-radius: 50%;
}

/* 自动补全框中的输入文本去掉padding,强制固定成一样的高度;而且关闭阴影 */
.ac-input-no-padding >>> input {
	padding: 0 !important;
	box-shadow: none !important;
}
</style>
