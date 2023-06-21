<script setup lang="ts">
import {ref, onMounted} from 'vue';
import {invoke} from "@tauri-apps/api/tauri";
import {writeText} from '@tauri-apps/api/clipboard';
import {message} from '@tauri-apps/api/dialog';
import {Account} from "./account";

const items = ref<Account[]>([]);
const type = ref(['all', 'name', 'username']);
const selected_type = ref('all');
const liked = ref(false);
const dialog_insert = ref(false);
const sequences = ref(Array.from({length: 10}, (k, v) => v + 1))
const likes = ref([true, false])
const rules = ref({
  required: (v: string) => !!v || 'Field is required'
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
})

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
  }
}

function toggle_liked() {
  liked.value = !liked.value;
}

async function on_click(text: string | number) {
  await writeText(text.toString());
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
    await message('name is required', {title: 'Insert Account Error', type: 'error'});
    return;
  }
  if (!account.username) {
    await message('username is required', {title: 'Insert Account Error', type: 'error'});
    return;
  }
  if (!account.password) {
    await message('password is required', {title: 'Insert Account Error', type: 'error'});
    return;
  }
  console.log(account)
  await invoke("insert", {"account": account}).then((res) => {
    // 在这里处理返回的 Account 类型数据
    if (!!res && typeof res === 'boolean' && res) {
      dialog_insert.value = false;
      query_all();
      clear_insert_account_info();
    } else {
      message('insert failed', {title: 'Insert Account Error', type: 'error'});
    }
  }).catch((err: unknown) => {
    message(JSON.stringify(err), {title: 'Insert Account Error', type: 'error'});
  });
}

async function query_all() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("query_all").then((res: unknown) => {
    // 在这里处理返回的 Account[] 类型数据
    if (!!res && typeof res === 'object' && res instanceof Array) {
      items.value = res as Account[];
    }
    console.log(items)
  });
}

onMounted(() => {
  query_all();
});

// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
</script>

<template>
  <v-card class="mx-auto">
    <template v-slot:title>
      <v-select
          v-model="selected_type"
          :items="type"
          label="Search Type"
          variant="solo-filled"
      ></v-select>
      <v-text-field
          label="Input your search keyword"
          variant="solo-filled"
          hide-details
      >
        <template #append-inner>
          <v-btn>
            Search
          </v-btn>
        </template>
      </v-text-field>
    </template>

    <v-divider></v-divider>

    <v-virtual-scroll :items="items" height="567" width="auto">
      <template v-slot:default="{ item }">
        <v-card>
          <v-card-title>{{ item.name }}</v-card-title>

          <v-card-subtitle>
            {{ item.sequence }}
          </v-card-subtitle>

          <v-card-text>
            <v-row>
              <v-col cols="6">
                <v-btn variant="tonal" @click="on_click(item.username)" width="100%">
                  {{ item.username }}
                </v-btn>
              </v-col>
              <v-col cols="6">
                <v-btn variant="tonal" @click="on_click(item.password)" width="100%">
                  {{ item.password }}
                </v-btn>
              </v-col>
            </v-row>

          </v-card-text>

          <v-card-actions>
            <v-row justify="end">
              <v-col cols="auto">
                <v-btn icon="mdi-delete" size="x-small" variant="tonal"></v-btn>
              </v-col>
              <v-col cols="auto">
                <v-btn
                    :icon="item.liked ? 'mdi-heart' : 'mdi-heart-outline'"
                    size="x-small"
                    variant="tonal"
                    @click="toggle_liked">
                </v-btn>
              </v-col>
              <v-col cols="auto">
                <v-btn
                    :icon="item.show ? 'mdi-chevron-up' : 'mdi-chevron-down'"
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
        <v-divider></v-divider>
      </template>
    </v-virtual-scroll>
  </v-card>

  <v-toolbar>
    <template v-slot:prepend>
      <v-dialog
          v-model="dialog_insert"
          persistent
      >
        <template v-slot:activator="{ props }">
          <v-btn variant="tonal" v-bind="props">
            Create New
          </v-btn>
        </template>
        <v-card class="mx-auto">
          <v-card-title>
            CREATE NEW ACCOUNT INFO
          </v-card-title>
          <v-card-text>
            <v-container>
              <v-text-field
                  v-model="insert_account_info.name"
                  label="NAME*"
                  variant="solo-filled"
                  clearable
                  :rules="[rules.required]"
              ></v-text-field>

              <v-text-field
                  v-model="insert_account_info.username"
                  label="USERNAME*"
                  variant="solo-filled"
                  clearable
                  :rules="[rules.required]"
              ></v-text-field>

              <v-text-field
                  v-model="insert_account_info.password"
                  label="PASSWORD*"
                  variant="solo-filled"
                  clearable
                  :rules="[rules.required]"
              ></v-text-field>

              <v-text-field
                  v-model="insert_account_info.description"
                  label="DESCRIPTION"
                  variant="solo-filled"
                  clearable
              ></v-text-field>

              <v-select
                  v-model="insert_account_info.liked"
                  label="mark this account as favorite"
                  :items="likes"
                  item-title="item-title"
                  variant="solo-filled"
              ></v-select>

              <v-select
                  v-model="insert_account_info.sequence"
                  label="Select sequence of this account"
                  :items="sequences"
                  variant="solo-filled"
              ></v-select>
            </v-container>
            <small>*indicates required field</small>
          </v-card-text>

          <v-divider></v-divider>

          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn
                color="error"
                variant="text"
                @click="dialog_insert = false">
              Close
            </v-btn>
            <v-btn
                color="success"
                variant="text"
                @click="on_insert_save">
              Save
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </template>
    <v-btn :icon="liked ? 'mdi-heart' : 'mdi-heart-outline'" class="ms-5" @click="toggle_liked"></v-btn>
    <v-divider
        class="mx-3 align-self-center"
        length="24"
        thickness="2"
        vertical
    ></v-divider>
    <v-btn icon="mdi-reload" @click="query_all"></v-btn>
  </v-toolbar>
</template>

<style scoped>
</style>
