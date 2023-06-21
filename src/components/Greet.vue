<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {writeText} from '@tauri-apps/api/clipboard';

const greetMsg = ref("");
const name = ref("");
const clickMsg = ref("");

async function say_hello() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("say_hello", {name: name.value});
}

async function handleButtonClick(buttonIndex: any) {
  // 处理按钮点击事件的逻辑
  clickMsg.value = '按钮点击：' + buttonIndex.toString();
  await writeText('按钮点击：' + buttonIndex.toString());
}
</script>

<template>

</template>

<style scoped>
</style>
