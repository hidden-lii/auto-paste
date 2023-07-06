<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { writeText } from '@tauri-apps/api/clipboard';
import { Account } from './account';
import { useConfirm, useSnackbar } from 'vuetify-use-dialog';

const items = ref<Account[]>([]);
const type = ref(['全部', '名称', '账号']);
const selected_type = ref('全部');
const liked = ref(false);
const like_type = ref(0);
const dialog_insert = ref(false);
const dialog_update = ref(false);
const sequences = ref(Array.from({ length: 10 }, (k, v) => v + 1));
const likes = ref([
	{ value: true, title: '喜欢' },
	{ value: false, title: '普通' }
]);
const keyword = ref('');
const rules = ref({
	required: (v: string) => !!v || '该项必填!'
});
const insert_account_info = ref<Account>({
	id: null,
	name: null,
	username: '',
	password: '',
	sequence: 1,
	liked: false,
	description: '',
	last_update_time: null,
	show: false
});
const update_account_info = ref<Account>({
	id: null,
	name: null,
	username: '',
	password: '',
	sequence: 1,
	liked: false,
	description: '',
	last_update_time: null,
	show: false
});
const createConfirm = useConfirm();
const createSnackbar = useSnackbar();

function clear_insert_account_info() {
	insert_account_info.value = {
		id: null,
		name: null,
		username: '',
		password: '',
		sequence: 1,
		liked: false,
		description: '',
		last_update_time: null,
		show: false
	};
}

function clear_update_account_info() {
	update_account_info.value = {
		id: null,
		name: null,
		username: '',
		password: '',
		sequence: 1,
		liked: false,
		description: '',
		last_update_time: null,
		show: false
	};
}

async function toggle_liked() {
	like_type.value = (like_type.value + 1) % 3;
	liked.value = like_type.value === 1;
	await query_by_value();
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
				query_all(false);
			} else {
				common_snacker_bar(is_like + '失败: 未知原因', 'error');
			}
		})
		.catch((err: unknown) => {
			common_snacker_bar(is_like + '失败: ' + JSON.stringify(err), 'error');
		});
}

function toggle_update(account: Account) {
	update_account_info.value = {
		id: account.id,
		name: account.name,
		username: account.username,
		password: account.password,
		sequence: account.sequence,
		liked: account.liked,
		description: account.description,
		last_update_time: account.last_update_time,
		show: account.show
	};
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

async function on_update_save() {
	let update_value = update_account_info.value;
	const account: Account = {
		id: update_value.id,
		name: update_value.name,
		username: update_value.username,
		password: update_value.password,
		sequence: update_value.sequence,
		liked: update_value.liked,
		description: update_value.description,
		last_update_time: null,
		show: false
	};
	if (!account.id) {
		common_snacker_bar('修改失败: id 为空', 'error');
		return;
	}
	if (!account.name) {
		common_snacker_bar('修改失败: name 为空', 'error');
		return;
	}
	if (!account.username) {
		common_snacker_bar('修改失败: username 为空', 'error');
		return;
	}
	if (!account.password) {
		common_snacker_bar('修改失败: password 为空', 'error');
		return;
	}
	await invoke('update', { account: account })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				dialog_update.value = false;
				common_snacker_bar('修改成功', 'success');
				query_all(false);
				clear_update_account_info();
			} else {
				common_snacker_bar('修改失败: 未知原因', 'error');
			}
		})
		.catch((err: unknown) => {
			common_snacker_bar('修改失败: ' + JSON.stringify(err), 'error');
		});
}

async function on_insert_quit() {
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

async function on_insert_save() {
	let insert_value = insert_account_info.value;
	const account: Account = {
		id: null,
		name: insert_value.name,
		username: insert_value.username,
		password: insert_value.password,
		sequence: insert_value.sequence,
		liked: insert_value.liked,
		description: insert_value.description,
		last_update_time: null,
		show: false
	};
	if (!account.name) {
		common_snacker_bar('添加账号信息失败: name 为空', 'error');
		return;
	}
	if (!account.username) {
		common_snacker_bar('添加账号信息失败: username 为空', 'error');
		return;
	}
	if (!account.password) {
		common_snacker_bar('添加账号信息失败: password 为空', 'error');
		return;
	}
	await invoke('insert', { account: account })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				dialog_insert.value = false;
				common_snacker_bar('添加账号信息成功', 'success');
				query_all(false);
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
	await query_all();
}

async function query_all(show_snackbar: boolean = true) {
	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	await invoke('query_all').then((res) => {
		// 在这里处理返回的 Account[] 类型数据
		if (!!res && res instanceof Array) {
			items.value = res as Account[];

			if (show_snackbar) {
				common_snacker_bar('查询成功', 'success');
			}
		}
	});
}

async function query_by_value() {
	let query_value: Account = {
		id: null,
		name:
			selected_type.value === '全部' || selected_type.value === '名称'
				? keyword.value
				: '',
		username:
			selected_type.value === '全部' || selected_type.value === '账号'
				? keyword.value
				: '',
		password: '',
		sequence: null,
		liked: liked.value,
		description: '',
		last_update_time: '',
		show: false
	};
	await invoke('query_by_value', {
		account: query_value,
		withLiked: like_type.value > 0
	}).then((res) => {
		// 在这里处理返回的 Account[] 类型数据
		if (!!res && res instanceof Array) {
			items.value = res as Account[];
			common_snacker_bar('查询成功', 'success');
		}
	});
}

async function delete_one(id: number | null) {
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

	await invoke('delete', { id: id })
		.then((res) => {
			if (!!res && typeof res === 'boolean' && res) {
				common_snacker_bar('删除成功', 'success');
				query_all(false);
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

function disableContextMenu(event: { preventDefault: () => void }) {
	event.preventDefault();
}

onMounted(() => {
	query_all(false);
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
								:items="type"
								label="搜索类型"
								variant="solo-filled"
								hide-details
								density="compact">
							</v-select>
						</v-col>

						<v-col cols="8">
							<v-text-field
								label="输入关键词"
								variant="solo-filled"
								v-model="keyword"
								hide-details
								density="compact">
								<template #append-inner>
									<v-btn
										icon="mdi-magnify"
										size="small"
										@click="query_by_value">
										<v-icon icon="mdi-magnify"></v-icon>
									</v-btn>
								</template>
							</v-text-field>
						</v-col>
					</v-row>
				</v-container>

				<v-divider></v-divider>

				<v-container class="scroll-container">
					<v-row dense>
						<template v-for="(item, index) in items" :key="index">
							<v-col cols="6">
								<v-card>
									<v-card-title>{{ item.name }}</v-card-title>

									<v-card-subtitle>
										优先级: {{ item.sequence }}
									</v-card-subtitle>

									<v-card-text>
										<v-row>
											<v-col cols="12">
												<v-btn
													variant="tonal"
													@click="on_click_copy(item.username)"
													width="100%"
													class="text-none">
													{{ item.username }}
												</v-btn>
											</v-col>
										</v-row>
										<v-row>
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
									<v-card-actions>
										<v-row justify="start">
											<v-col cols="3">
												<v-dialog v-model="dialog_update" persistent>
													<template v-slot:activator="{ props }">
														<v-btn
															variant="tonal"
															v-bind="props"
															icon="mdi-delete"
															size="x-small"
															@click="toggle_update(item)">
															<v-icon icon="mdi-file-edit"></v-icon>
														</v-btn>
													</template>
													<v-card class="mx-12">
														<v-card-title>修改账号信息</v-card-title>
														<v-card-text>
															<v-container>
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
																	label="选择账号优先级(用于排序)"
																	:items="sequences"
																	density="compact"
																	variant="solo-filled">
																</v-select>
															</v-container>
															<small>带*的为必填项!</small>
														</v-card-text>

														<v-divider></v-divider>

														<v-card-actions>
															<v-spacer></v-spacer>
															<v-btn
																color="error"
																variant="text"
																@click="on_update_quit">
																关闭
															</v-btn>
															<v-btn
																color="success"
																variant="text"
																@click="on_update_save">
																保存
															</v-btn>
														</v-card-actions>
													</v-card>
												</v-dialog>
											</v-col>
											<v-col cols="3">
												<v-btn
													icon="mdi-delete"
													size="x-small"
													variant="tonal"
													@click="delete_one(item.id)">
												</v-btn>
											</v-col>
											<v-col cols="3">
												<v-btn
													:icon="item.liked ? 'mdi-heart' : 'mdi-heart-outline'"
													size="x-small"
													variant="tonal"
													@click="on_click_like(item.id, item.liked)">
												</v-btn>
											</v-col>
											<v-col cols="3">
												<v-btn
													:icon="
														item.show ? 'mdi-chevron-up' : 'mdi-chevron-down'
													"
													size="x-small"
													variant="tonal"
													@click="item.show = !item.show">
												</v-btn>
											</v-col>
										</v-row>
									</v-card-actions>

									<v-expand-transition>
										<div v-show="item.show">
											<v-divider></v-divider>
											<v-card-text>
												{{ item.description }}
											</v-card-text>
										</div>
									</v-expand-transition>
								</v-card>
							</v-col>
						</template>
					</v-row>
				</v-container>
			</div>
		</v-main>

		<v-toolbar density="comfortable">
			<template v-slot:prepend>
				<v-dialog v-model="dialog_insert" persistent style="width: inherit">
					<template v-slot:activator="{ props }">
						<v-btn v-bind="props" size="x-small" icon="mdi-plus">
							<v-icon icon="mdi-plus"></v-icon>
						</v-btn>
					</template>
					<v-card class="mx-12">
						<v-card-title>添加账号信息</v-card-title>
						<v-card-text>
							<v-container>
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
									label="选择账号优先级(用于排序)"
									:items="sequences"
									density="compact"
									variant="solo-filled">
								</v-select>
							</v-container>
							<small>带*的为必填项!</small>
						</v-card-text>

						<v-divider></v-divider>

						<v-card-actions>
							<v-spacer></v-spacer>
							<v-btn color="error" variant="text" @click="on_insert_quit">
								关闭
							</v-btn>
							<v-btn color="success" variant="text" @click="on_insert_save">
								保存
							</v-btn>
						</v-card-actions>
					</v-card>
				</v-dialog>
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
				size="x-small"
				@click="toggle_liked"></v-btn>
			<v-divider
				class="mx-3 align-self-center"
				length="24"
				thickness="2"
				vertical></v-divider>
			<v-btn icon="mdi-reload" @click="refresh" size="x-small"></v-btn>
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
</style>
