<script setup lang="ts">
const open = defineModel<boolean>({ required: true });

const emit = defineEmits<{
	allow: [];
	deny: [];
}>();

function onAllow() {
	emit('allow');
	open.value = false;
}

function onDeny() {
	emit('deny');
	open.value = false;
}
</script>

<template>
	<v-dialog v-model="open" persistent max-width="400">
		<v-card class="mx-12 dialog-card" density="compact">
			<v-card-title>联网同步区服数据</v-card-title>
			<v-card-text style="padding: 0 24px">
				允许后将从 JX3API 获取最新区服及在线状态；拒绝则使用内置区服列表，不影响基本使用。
			</v-card-text>
			<v-card-actions style="padding: 0 24px 16px">
				<v-spacer />
				<v-btn variant="text" @click="onDeny">暂不联网</v-btn>
				<v-btn color="primary" variant="text" @click="onAllow">
					允许联网同步
				</v-btn>
			</v-card-actions>
		</v-card>
	</v-dialog>
</template>
