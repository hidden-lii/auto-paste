<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { writeText } from '@tauri-apps/api/clipboard';
import { message, confirm } from '@tauri-apps/api/dialog';
import { Account } from './account';

const items = ref<Account[]>([]);
const type = ref(['all', 'name', 'username']);
const selected_type = ref('all');
const liked = ref(false);
const like_type = ref(0);
const dialog_insert = ref(false);
const dialog_update = ref(false);
const sequences = ref(Array.from({ length: 10 }, (k, v) => v + 1));
const likes = ref([
	{ value: true, title: 'Liked' },
	{ value: false, title: 'Unliked' },
]);
const keyword = ref();
const rules = ref({
	required: (v: string) => !!v || 'Field is required',
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
	show: false,
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
	show: false,
});

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
		show: false,
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
		show: false,
	};
}

function toggle_liked() {
	like_type.value = (like_type.value + 1) % 3;
	liked.value = like_type.value === 1;
}

async function on_click_copy(text: string | number | null) {
	if (!text) {
		await message('text is required', { title: 'Copy Error', type: 'error' });
		return;
	}
	await writeText(text.toString());
}

async function on_click_like(id: number | null, liked: boolean) {
	if (!id) {
		await message('id is required', {
			title: 'Mark Like Error',
			type: 'error',
		});
		return;
	}
	await invoke('update_like', { id: id, liked: !liked })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				query_all();
			} else {
				message('like failed', { title: 'Mark Like Error', type: 'error' });
			}
		})
		.catch((err: unknown) => {
			message(JSON.stringify(err), { title: 'Mark Like Error', type: 'error' });
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
		show: account.show,
	};
}

async function on_update_quit() {
	const quit = await confirm('Your account info is not saved. Are you sure?', {
		title: 'Insert Account',
		type: 'warning',
	});
	if (!quit) {
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
		show: false,
	};
	if (!account.id) {
		await message('id is required', {
			title: 'Update Account Error',
			type: 'error',
		});
		return;
	}
	if (!account.name) {
		await message('name is required', {
			title: 'Insert Account Error',
			type: 'error',
		});
		return;
	}
	if (!account.username) {
		await message('username is required', {
			title: 'Insert Account Error',
			type: 'error',
		});
		return;
	}
	if (!account.password) {
		await message('password is required', {
			title: 'Insert Account Error',
			type: 'error',
		});
		return;
	}
	await invoke('update', { account: account })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				dialog_update.value = false;
				query_all();
				clear_update_account_info();
			} else {
				message('update failed', {
					title: 'Update Account Error',
					type: 'error',
				});
			}
		})
		.catch((err: unknown) => {
			message(JSON.stringify(err), {
				title: 'Update Account Error',
				type: 'error',
			});
		});
}

async function on_insert_quit() {
	const quit = await confirm('Your account info is not saved. Are you sure?', {
		title: 'Insert Account',
		type: 'warning',
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
		show: false,
	};
	if (!account.name) {
		await message('name is required', {
			title: 'Insert Account Error',
			type: 'error',
		});
		return;
	}
	if (!account.username) {
		await message('username is required', {
			title: 'Insert Account Error',
			type: 'error',
		});
		return;
	}
	if (!account.password) {
		await message('password is required', {
			title: 'Insert Account Error',
			type: 'error',
		});
		return;
	}
	await invoke('insert', { account: account })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				dialog_insert.value = false;
				query_all();
				clear_insert_account_info();
			} else {
				message('insert failed', {
					title: 'Insert Account Error',
					type: 'error',
				});
			}
		})
		.catch((err: unknown) => {
			message(JSON.stringify(err), {
				title: 'Insert Account Error',
				type: 'error',
			});
		});
}

async function refresh() {
	keyword.value = '';
	like_type.value = 0;
	liked.value = false;
	await query_all();
}

async function query_all() {
	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	await invoke('query_all').then((res) => {
		// 在这里处理返回的 Account[] 类型数据
		if (!!res && res instanceof Array) {
			items.value = res as Account[];
		}
	});
}

async function query_by_value() {
	let query_value: Account = {
		id: null,
		name:
			selected_type.value === 'all' || selected_type.value === 'name'
				? keyword.value
				: '',
		username:
			selected_type.value === 'all' || selected_type.value === 'username'
				? keyword.value
				: '',
		password: '',
		sequence: null,
		liked: liked.value,
		description: '',
		last_update_time: '',
		show: false,
	};
	await invoke('query_by_value', {
		account: query_value,
		withLiked: like_type.value > 0,
	}).then((res) => {
		// 在这里处理返回的 Account[] 类型数据
		if (!!res && res instanceof Array) {
			items.value = res as Account[];
		}
	});
}

async function delete_one(id: number | null) {
	if (!id) {
		await message('id is required', { title: 'Delete Error', type: 'error' });
		return;
	}
	const delete_confirm = await confirm(
		'This action cannot be reverted. Are you sure to delete this account?',
		{
			title: 'Delete Account',
			type: 'warning',
		}
	);
	if (!delete_confirm) {
		return;
	}
	await invoke('delete', { id: id })
		.then((res) => {
			// 在这里处理返回的 Account 类型数据
			if (!!res && typeof res === 'boolean' && res) {
				query_all();
			} else {
				message('delete failed', {
					title: 'Delete Account Error',
					type: 'error',
				});
			}
		})
		.catch((err) => {
			message(JSON.stringify(err), {
				title: 'Delete Account Error',
				type: 'error',
			});
		});
}

function disableContextMenu(event: { preventDefault: () => void }) {
	event.preventDefault();
}

onMounted(() => {
	query_all();
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
								label="Search Type"
								variant="solo-filled"
								hide-details
								density="compact">
              </v-select>
						</v-col>

						<v-col cols="8">
							<v-text-field
								label="Input your search keyword"
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
										Sequence: {{ item.sequence }}
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
														<v-card-title> UPDATE ACCOUNT INFO</v-card-title>
														<v-card-text>
															<v-container>
																<v-text-field
																	v-model="update_account_info.name"
																	label="NAME*"
																	variant="solo-filled"
																	clearable
																	:rules="[rules.required]">
                                </v-text-field>

																<v-text-field
																	v-model="update_account_info.username"
																	label="USERNAME*"
																	variant="solo-filled"
																	clearable
																	:rules="[rules.required]">
                                </v-text-field>

																<v-text-field
																	v-model="update_account_info.password"
																	label="PASSWORD*"
																	variant="solo-filled"
																	clearable
																	:rules="[rules.required]">
                                </v-text-field>

																<v-text-field
																	v-model="update_account_info.description"
																	label="DESCRIPTION"
																	variant="solo-filled"
																	clearable>
                                </v-text-field>

																<v-select
																	v-model="update_account_info.liked"
																	label="mark this account as favorite"
																	:items="likes"
																	item-title="title"
																	item-value="value"
																	variant="solo-filled">
                                </v-select>

																<v-select
																	v-model="update_account_info.sequence"
																	label="Select sequence of this account"
																	:items="sequences"
																	variant="solo-filled">
                                </v-select>
															</v-container>
															<small>*indicates required field</small>
														</v-card-text>

														<v-divider></v-divider>

														<v-card-actions>
															<v-spacer></v-spacer>
															<v-btn
																color="error"
																variant="text"
																@click="on_update_quit">
																Close
															</v-btn>
															<v-btn
																color="success"
																variant="text"
																@click="on_update_save">
																Save
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
						<v-btn
							v-bind="props"
							size="x-small"
							icon="mdi-plus"
							@click="toggle_liked">
							<v-icon icon="mdi-plus"></v-icon>
						</v-btn>
					</template>
					<v-card class="mx-12">
						<v-card-title> CREATE NEW ACCOUNT INFO</v-card-title>
						<v-card-text>
							<v-container>
								<v-text-field
									v-model="insert_account_info.name"
									label="NAME*"
									variant="solo-filled"
									clearable
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="insert_account_info.username"
									label="USERNAME*"
									variant="solo-filled"
									clearable
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="insert_account_info.password"
									label="PASSWORD*"
									variant="solo-filled"
									clearable
									:rules="[rules.required]">
								</v-text-field>

								<v-text-field
									v-model="insert_account_info.description"
									label="DESCRIPTION"
									variant="solo-filled"
									clearable>
								</v-text-field>

								<v-select
									v-model="insert_account_info.liked"
									label="mark this account as favorite"
									:items="likes"
									item-title="title"
									item-value="value"
									variant="solo-filled">
                </v-select>

								<v-select
									v-model="insert_account_info.sequence"
									label="Select sequence of this account"
									:items="sequences"
									variant="solo-filled">
                </v-select>
							</v-container>
							<small>*indicates required field</small>
						</v-card-text>

						<v-divider></v-divider>

						<v-card-actions>
							<v-spacer></v-spacer>
							<v-btn color="error" variant="text" @click="on_insert_quit">
								Close
							</v-btn>
							<v-btn color="success" variant="text" @click="on_insert_save">
								Save
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
