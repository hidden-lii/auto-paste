<script setup lang='ts'>
import {ref, onMounted} from 'vue';
import {invoke} from '@tauri-apps/api/tauri';
import {writeText} from '@tauri-apps/api/clipboard';
import {Account} from '../entity/account';
import {useConfirm, useSnackbar} from 'vuetify-use-dialog';
import {Category} from '../entity/category';

const available_accounts = ref<Account[]>([]);
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
const sequences = ref(Array.from({length: 10}, (k, v) => v + 1));
const likes = ref([
  {value: true, title: '喜欢'},
  {value: false, title: '普通'}
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

function show_common_confirm(
    content: string,
    title: string,
    confirmationText: string = '确认',
    cancellationText: string = '取消'
) {
  return createConfirm({
    content: content,
    title: title,
    confirmationText: confirmationText,
    cancellationText: cancellationText,
    dialogProps: {
      persistent: true
    }
  });
}

function show_common_snackbar(
    text: string,
    color: string,
    timeout: number = 1000,
    showCloseButton: boolean = false
) {
  createSnackbar({
    text: text,
    snackbarProps: {
      timeout: timeout,
      color: color,
      minWidth: 'fit-content',
      maxWidth: 'fit-content'
    },
    showCloseButton: showCloseButton
  });
}

async function on_insert_category_quit() {
  const quit = await show_common_confirm(
      '分组还没有保存, 确认退出吗?',
      '添加分组'
  );

  if (!quit) {
    return;
  }

  dialog_insert_category.value = false;
  clear_insert_category_info();
}

async function on_insert_category_save() {
  let insert_value = {...insert_category_info.value};

  if (!insert_value.name || insert_value.name === '') {
    show_common_snackbar('添加分组失败: 名称 为空', 'error');
    return;
  }
  await invoke('create_category', {category: insert_value})
      .then((res) => {
        if (!!res && typeof res === 'boolean' && res) {
          dialog_insert_category.value = false;
          show_common_snackbar('添加分组成功', 'success');
          // 添加分组时,可能会增加分组和账号的关系,所以账号信息也要重新查询一遍
          query_accounts_by_value(false);
          // 更新分组信息
          query_all_categories(false);
          // 更新所有账号信息(用于分组信息更新时的选取)
          query_all_accounts(false, false);
          clear_insert_category_info();
        } else {
          show_common_snackbar('添加分组失败: 未知原因', 'error');
        }
      })
      .catch((err: unknown) => {
        show_common_snackbar('添加分组失败: ' + JSON.stringify(err), 'error');
      });
}

async function on_update_category_quit() {
  const quit = await show_common_confirm(
      '分组信息还没有保存, 确认退出吗?',
      '修改分组'
  );

  if (!quit) {
    return;
  }

  dialog_update_category.value = false;
  clear_update_category_info();
}

async function on_update_category_save() {
  let update_value = {...selected_category.value};

  if (!update_value.name || update_value.name === '') {
    show_common_snackbar('修改分组失败: 名称 为空', 'error');
    return;
  }
  await invoke('update_category', {category: update_value})
      .then((res) => {
        if (!!res && typeof res === 'boolean' && res) {
          dialog_update_category.value = false;
          show_common_snackbar('修改分组成功', 'success');
          // 修改分组时,可能会变更分组和账号的关系,所以账号信息也要重新查询一遍
          query_accounts_by_value(false);
          // 更新分组信息
          query_all_categories(false);
          // 更新所有账号信息(用于分组信息更新时的选取)
          query_all_accounts(false, false);
          clear_update_category_info();
        } else {
          show_common_snackbar('修改分组失败: 未知原因', 'error');
        }
      })
      .catch((err: unknown) => {
        show_common_snackbar('修改分组失败: ' + JSON.stringify(err), 'error');
      });
}

async function toggle_liked() {
  like_type.value = (like_type.value + 1) % 3;
  liked.value = like_type.value === 1;
  await query_accounts_by_value();
}

async function on_click_copy(text: string | number | null) {
  if (!text) {
    show_common_snackbar('复制失败: 没有内容诶,你在复制什么?', 'error');
    return;
  }
  await writeText(text.toString());
  show_common_snackbar('复制成功', 'success');
}

async function on_click_like(id: number | null, liked: boolean) {
  const is_like = liked ? '取消标记' : '标记';
  if (!id) {
    show_common_snackbar(is_like + '失败: 需要指定id', 'error');
    return;
  }
  await invoke('update_like', {id: id, liked: !liked})
      .then((res) => {
        // 在这里处理返回的 Account 类型数据
        if (!!res && typeof res === 'boolean' && res) {
          show_common_snackbar(is_like + '成功', 'success');
          query_accounts_by_value(false);
          query_all_accounts(false, false);
        } else {
          show_common_snackbar(is_like + '失败: 未知原因', 'error');
        }
      })
      .catch((err: unknown) => {
        show_common_snackbar(is_like + '失败: ' + JSON.stringify(err), 'error');
      });
}

function toggle_update(account: Account) {
  update_account_info.value = {...account};

  dialog_update.value = true;
}

async function on_update_quit() {
  const isConfirmed = await show_common_confirm(
      '账号信息还没有保存, 确认退出吗?',
      '修改账号信息'
  );

  if (!isConfirmed) {
    return;
  }

  dialog_update.value = false;
  clear_update_account_info();
}

async function on_update_account_save() {
  let update_value = {...update_account_info.value};
  if (!update_value.id) {
    show_common_snackbar('修改失败: id 为空', 'error');
    return;
  }
  if (!update_value.name || update_value.name === '') {
    show_common_snackbar('修改失败: name 为空', 'error');
    return;
  }
  if (!update_value.username || update_value.username === '') {
    show_common_snackbar('修改失败: username 为空', 'error');
    return;
  }
  if (!update_value.password) {
    show_common_snackbar('修改失败: password 为空', 'error');
    return;
  }
  await invoke('update_account', {account: update_value})
      .then((res) => {
        // 在这里处理返回的 Account 类型数据
        if (!!res && typeof res === 'boolean' && res) {
          dialog_update.value = false;
          show_common_snackbar('修改成功', 'success');
          // 更新当前查询的账号信息
          query_accounts_by_value(false);
          query_all_categories(false);
          query_all_accounts(false, false);
          clear_update_account_info();
        } else {
          show_common_snackbar('修改失败: 未知原因', 'error');
        }
      })
      .catch((err: unknown) => {
        show_common_snackbar('修改失败: ' + JSON.stringify(err), 'error');
      });
}

async function on_insert_account_quit() {
  const quit = await show_common_confirm(
      '账号信息还没有保存, 确认退出吗?',
      '添加账号信息'
  );

  if (!quit) {
    return;
  }

  dialog_insert.value = false;
  clear_insert_account_info();
}

async function on_insert_account_save() {
  let account = {...insert_account_info.value};
  if (!account.name || account.name === '') {
    show_common_snackbar('添加账号信息失败: name 为空', 'error');
    return;
  }
  if (!account.username || account.username === '') {
    show_common_snackbar('添加账号信息失败: username 为空', 'error');
    return;
  }
  if (!account.password) {
    show_common_snackbar('添加账号信息失败: password 为空', 'error');
    return;
  }
  await invoke('insert_account', {account: account})
      .then((res) => {
        // 在这里处理返回的 Account 类型数据
        if (!!res && typeof res === 'boolean' && res) {
          dialog_insert.value = false;
          show_common_snackbar('添加账号信息成功', 'success');
          // 添加账号信息时,可能会增加分组和账号的关系
          // 所以当前查询的账号信息和所有分组信息都需要重新更新一遍
          query_accounts_by_value(false);
          query_all_categories(false);
          // 更新所有账号信息(用于分组信息更新时的选取)
          query_all_accounts(false, false);
          clear_insert_account_info();
        } else {
          show_common_snackbar('添加账号信息失败: 未知原因', 'error');
        }
      })
      .catch((err: unknown) => {
        show_common_snackbar('添加账号信息失败: ' + JSON.stringify(err), 'error');
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
  await query_all_categories();
}

async function query_all_accounts(
    show_snackbar: boolean = true,
    update_accounts: boolean = true
) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke('query_all_accounts').then((res) => {
    // 在这里处理返回的 Account[] 类型数据
    if (!!res && res instanceof Array) {
      available_accounts.value = res as Account[];
      if (update_accounts) {
        accounts.value = res as Account[];
      }

      if (show_snackbar) {
        show_common_snackbar('查询成功', 'success');
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
        show_common_snackbar('查询成功', 'success');
      }
    }
  });
}

async function delete_one_account(id: number | null) {
  if (!id) {
    show_common_snackbar('删除账号失败: id 为空', 'error');
    return;
  }
  const delete_confirm = await show_common_confirm(
      '这个操作不可回退, 确认删除此账号吗?',
      '删除账号提示'
  );

  if (!delete_confirm) {
    return;
  }

  await invoke('delete_account', {id: id})
      .then((res) => {
        if (!!res && typeof res === 'boolean' && res) {
          show_common_snackbar('删除成功', 'success');
          // 更新当前查询的账号信息
          query_accounts_by_value(false);
          // 更新所有账号信息(用于分组信息更新时的选取)
          query_all_accounts(false, false);
          // 更新分组信息
          query_all_categories(false);
        } else {
          show_common_snackbar('删除账号失败: 未知原因', 'error');
        }
      })
      .catch((err) => {
        show_common_snackbar('删除账号失败: ' + JSON.stringify(err), 'error');
      });
}

async function query_all_categories(show_snackbar: boolean = true) {
  await invoke('query_all_category').then((res) => {
    // 在这里处理返回的 Account[] 类型数据
    if (!!res && res instanceof Array) {
      categories.value = [new Category(-1, '全部'), ...(res as Category[])];
      available_categories.value = res as Category[];

      if (show_snackbar) {
        show_common_snackbar('查询成功', 'success');
      }
    }
  });
}

async function on_delete_category() {
  let id = selected_category.value.id;
  if (!id || id === -1) {
    show_common_snackbar('删除分组失败: id 为空', 'error');
    return;
  }
  const delete_confirm = await show_common_confirm(
      '这个操作不可回退, 确认删除此分组吗?',
      '删除分组提示'
  );
  if (!delete_confirm) {
    return;
  }
  await invoke('delete_category_by_id', {id: id})
      .then((res) => {
        if (!!res && typeof res === 'boolean' && res) {
          show_common_snackbar('删除成功', 'success');
          // 更新分组信息
          query_all_categories(false);
          // 更新当前查询的账号信息
          query_accounts_by_value(false);
          // 更新所有账号信息(用于分组信息更新时的选取)
          query_all_accounts(false, false);
          selected_category.value = new Category(-1, '全部');
        } else {
          show_common_snackbar('删除分组失败: 未知原因', 'error');
        }
      })
      .catch((err) => {
        show_common_snackbar('删除分组失败: ' + JSON.stringify(err), 'error');
      });
}

onMounted(() => {
  query_all_accounts(false);
  query_all_categories(false);
});

// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
</script>

<template>
  <div class='container'>
    <!-- #search info area -->
    <v-container>
      <!-- #normal search info -->
      <v-row>
        <v-col cols='4'>
          <v-select
              v-model='selected_type'
              :items='types'
              label='搜索类型'
              variant='solo-filled'
              hide-details
              density='compact'>
          </v-select>
        </v-col>

        <v-col cols='7'>
          <v-text-field
              label='输入关键词'
              variant='solo-filled'
              v-model='keyword'
              hide-details
              density='compact'>
            <template #append-inner>
              <v-btn icon size='small' @click='query_accounts_by_value'>
                <v-icon icon='mdi-magnify'></v-icon>
              </v-btn>
            </template>
          </v-text-field>
        </v-col>

        <v-col cols='1' class='col-align-center'>
          <v-btn
              :icon="show_category_select ? 'mdi-chevron-up' : 'mdi-chevron-down'"
              size='x-small'
              @click='toggle_show_category_select'>
          </v-btn>
        </v-col>
      </v-row>
      <!-- #category search info -->
      <v-expand-transition>
        <v-row v-show='show_category_select'>
          <v-col cols='8'>
            <v-select
                v-model='selected_category'
                :items='categories'
                item-title='name'
                item-value='id'
                return-object
                label='账号分组'
                variant='solo-filled'
                hide-details
                :menu-props='{ maxHeight: 250 }'
                @update:model-value='query_accounts_by_value(true)'
                density='compact'>
            </v-select>
          </v-col>
          <v-col cols='4'>
            <!-- #category btn group -->
            <v-btn-group variant='outlined' divided>
              <v-btn
                  variant='tonal'
                  @click='dialog_insert_category = true'
                  size='small'
                  icon>
                <v-icon icon='mdi-account-multiple-plus'></v-icon>
              </v-btn>
              <v-btn
                  variant='tonal'
                  @click='dialog_update_category = true'
                  :disabled='
									selected_category.id === null || selected_category.id === -1
								'
                  size='small'
                  icon>
                <v-icon icon='mdi-account-multiple'></v-icon>
              </v-btn>
              <v-btn
                  icon
                  @click='on_delete_category'
                  size='small'
                  variant='tonal'
                  :disabled='
									selected_category.id === null || selected_category.id === -1
								'>
                <v-icon icon='mdi-account-multiple-remove'></v-icon>
              </v-btn>
            </v-btn-group>
          </v-col>
        </v-row>
      </v-expand-transition>
    </v-container>

    <v-divider></v-divider>

    <!-- #account info area -->
    <v-container class='scroll-container'>
      <v-row dense>
        <template v-for='(item, index) in accounts' :key='index'>
          <v-col cols='6'>
            <v-hover v-slot='{ isHovering, props }'>
              <v-card
                  v-bind='props'
                  density='compact'
                  @click='item.show = !item.show'>
                <div v-if='isHovering'>
                  <v-sheet class='edit-button' @click.stop>
                    <v-btn
                        :icon="item.liked ? 'mdi-heart' : 'mdi-heart-outline'"
                        size='x-small'
                        variant='tonal'
                        @click='on_click_like(item.id, item.liked)'>
                    </v-btn>
                  </v-sheet>
                  <v-sheet class='close-button' @click.stop>
                    <v-btn
                        icon='mdi-delete'
                        size='x-small'
                        variant='tonal'
                        @click='delete_one_account(item.id)'>
                    </v-btn>
                  </v-sheet>
                </div>
                <v-card-title>
                  <v-chip
                      class='ma-2'
                      variant='outlined'
                      @click.stop
                      @click='toggle_update(item)'>
                    {{ item.name }}
                  </v-chip>
                </v-card-title>

                <v-card-subtitle> 优先级: {{ item.sequence }}</v-card-subtitle>

                <v-card-text>
                  <v-row dense>
                    <v-col cols='12'>
                      <v-btn
                          variant='tonal'
                          @click.stop
                          @click='on_click_copy(item.username)'
                          width='100%'
                          class='text-none'>
                        {{ item.username }}
                      </v-btn>
                    </v-col>
                    <v-col cols='12'>
                      <v-btn
                          variant='tonal'
                          @click.stop
                          @click='on_click_copy(item.password)'
                          width='100%'
                          class='text-none'>
                        {{ item.password }}
                      </v-btn>
                    </v-col>
                  </v-row>
                </v-card-text>
                <v-expand-transition>
                  <div v-show='item.show'>
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
    <v-dialog v-model='dialog_update' persistent>
      <v-card class='mx-12 dialog-card' density='compact'>
        <v-card-title>修改账号信息</v-card-title>
        <v-card-text style='padding: 0 24px'>
          <v-container style='padding: 0'>
            <v-text-field
                v-model='update_account_info.name'
                label='名称*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-text-field
                v-model='update_account_info.username'
                label='账号*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-text-field
                v-model='update_account_info.password'
                label='密码*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-text-field
                v-model='update_account_info.description'
                label='描述'
                variant='solo-filled'
                density='compact'
                clearable>
            </v-text-field>

            <v-select
                v-model='update_account_info.liked'
                label="标记账号为'喜欢'"
                :items='likes'
                item-title='title'
                item-value='value'
                density='compact'
                variant='solo-filled'>
            </v-select>

            <v-select
                v-model='update_account_info.sequence'
                label='选择账号优先级(用于排序,暂未实现)'
                :items='sequences'
                density='compact'
                variant='solo-filled'>
            </v-select>

            <v-autocomplete
                v-model='update_account_info.account_category_ids'
                label='选择所属分组(可多选)'
                :items='available_categories'
                item-title='name'
                item-value='id'
                multiple
                chips
                closable-chips
                :menu-props='{ maxHeight: 250 }'
                density='compact'
                class='ac-input-no-padding'
                variant='solo-filled'>
              <template v-slot:item='{ props, item }'>
                <v-list-item
                    v-bind='props'
                    prepend-icon='mdi-account-group'
                    :title='item.raw.name'></v-list-item>
              </template>
            </v-autocomplete>
          </v-container>
        </v-card-text>

        <v-divider style='margin-top: 10px'></v-divider>

        <v-card-actions style='padding: 0 24px'>
          <v-spacer></v-spacer>
          <v-btn color='error' variant='text' @click='on_update_quit'>
            关闭
          </v-btn>
          <v-btn color='success' variant='text' @click='on_update_account_save'>
            保存
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- #insert account dialog -->
    <v-dialog v-model='dialog_insert' persistent>
      <v-card class='mx-12 dialog-card' density='compact'>
        <v-card-title>添加账号信息</v-card-title>
        <v-card-text style='padding: 0 24px'>
          <v-container style='padding: 0'>
            <v-text-field
                v-model='insert_account_info.name'
                label='名称*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-text-field
                v-model='insert_account_info.username'
                label='账号*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-text-field
                v-model='insert_account_info.password'
                label='密码*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-text-field
                v-model='insert_account_info.description'
                label='描述'
                variant='solo-filled'
                density='compact'
                clearable>
            </v-text-field>

            <v-select
                v-model='insert_account_info.liked'
                label="标记账号为'喜欢'"
                :items='likes'
                item-title='title'
                item-value='value'
                density='compact'
                variant='solo-filled'>
            </v-select>

            <v-select
                v-model='insert_account_info.sequence'
                label='选择账号优先级(用于排序,暂未实现)'
                :items='sequences'
                density='compact'
                variant='solo-filled'>
            </v-select>

            <v-autocomplete
                v-model='insert_account_info.account_category_ids'
                label='选择所属分组(可多选)'
                :items='available_categories'
                item-title='name'
                item-value='id'
                multiple
                chips
                closable-chips
                :menu-props='{ maxHeight: 250 }'
                density='compact'
                class='ac-input-no-padding'
                variant='solo-filled'>
              <template v-slot:item='{ props, item }'>
                <v-list-item
                    v-bind='props'
                    prepend-icon='mdi-account-group'
                    :title='item.raw.name'></v-list-item>
              </template>
            </v-autocomplete>
          </v-container>
        </v-card-text>

        <v-divider style='margin-top: 10px'></v-divider>

        <v-card-actions style='padding: 0 24px'>
          <v-spacer></v-spacer>
          <v-btn color='error' variant='text' @click='on_insert_account_quit'>
            关闭
          </v-btn>
          <v-btn color='success' variant='text' @click='on_insert_account_save'>
            保存
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- #insert categroy dialog -->
    <v-dialog v-model='dialog_insert_category' persistent>
      <v-card class='mx-12 dialog-card' density='compact'>
        <v-card-title>添加分组</v-card-title>
        <v-card-text style='padding: 0 24px'>
          <v-container style='padding: 0'>
            <v-text-field
                v-model='insert_category_info.name'
                label='名称*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-select
                v-model='insert_category_info.sequence'
                label='选择分组优先级(用于排序,暂未实现)'
                :items='sequences'
                :menu-props='{ maxHeight: 250 }'
                density='compact'
                variant='solo-filled'>
            </v-select>

            <v-autocomplete
                v-model='insert_category_info.account_ids'
                label='关联的账号'
                :items='available_accounts'
                item-title='username'
                item-value='id'
                multiple
                chips
                closable-chips
                :menu-props='{ maxHeight: 250 }'
                density='compact'
                class='ac-input-no-padding'
                variant='solo-filled'>
              <template v-slot:item='{ props, item }'>
                <v-list-item
                    v-bind='props'
                    prepend-icon='mdi-account'
                    :title='item.raw.username'
                    :subtitle='item.raw.name'></v-list-item>
              </template>
            </v-autocomplete>
          </v-container>
        </v-card-text>

        <v-divider style='margin-top: 10px'></v-divider>

        <v-card-actions style='padding: 0 24px'>
          <v-spacer></v-spacer>
          <v-btn color='error' variant='text' @click='on_insert_category_quit'>
            关闭
          </v-btn>
          <v-btn
              color='success'
              variant='text'
              @click='on_insert_category_save'>
            保存
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- #update category dialog -->
    <v-dialog v-model='dialog_update_category' persistent>
      <v-card class='mx-12 dialog-card' density='compact'>
        <v-card-title>修改分组</v-card-title>
        <v-card-text style='padding: 0 24px'>
          <v-container style='padding: 0'>
            <v-text-field
                style='padding: 0'
                v-model='selected_category.name'
                label='名称*'
                variant='solo-filled'
                clearable
                density='compact'
                :rules='[rules.required]'>
            </v-text-field>

            <v-select
                v-model='selected_category.sequence'
                label='选择分组优先级(用于排序,暂未实现)'
                :items='sequences'
                :menu-props='{ maxHeight: 250 }'
                density='compact'
                variant='solo-filled'>
            </v-select>

            <v-autocomplete
                v-model='selected_category.account_ids'
                label='关联的账号'
                :items='available_accounts'
                item-title='username'
                item-value='id'
                multiple
                chips
                closable-chips
                :menu-props='{ maxHeight: 250 }'
                density='compact'
                class='ac-input-no-padding'
                variant='solo-filled'>
              <template v-slot:item='{ props, item }'>
                <v-list-item
                    v-bind='props'
                    prepend-icon='mdi-account'
                    :title='item.raw.username'
                    :subtitle='item.raw.name'></v-list-item>
              </template>
            </v-autocomplete>
          </v-container>
        </v-card-text>

        <v-divider style='margin-top: 10px'></v-divider>

        <v-card-actions style='padding: 0 24px'>
          <v-spacer></v-spacer>
          <v-btn color='error' variant='text' @click='on_update_category_quit'>
            关闭
          </v-btn>
          <v-btn
              color='success'
              variant='text'
              @click='on_update_category_save'>
            保存
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>

  <!-- footbar -->
  <v-toolbar density='comfortable'>
    <template v-slot:prepend>
      <v-btn size='small' icon @click='dialog_insert = true'>
        <v-icon icon='mdi-account-plus'></v-icon>
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
        class='ms-5'
        size='small'
        @click='toggle_liked'></v-btn>
    <v-divider
        class='mx-3 align-self-center'
        length='24'
        thickness='2'
        vertical></v-divider>
    <v-btn icon='mdi-reload' @click='refresh' size='small'></v-btn>
  </v-toolbar>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  /* footbar 高55px */
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
