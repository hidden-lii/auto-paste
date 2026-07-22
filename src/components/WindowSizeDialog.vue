<script setup lang="ts">
import { restoreDefaultWindowSize } from '../utils/window';
import { useFeedback } from '../utils/feedback';

const open = defineModel<boolean>({ required: true });

defineProps<{
	width: number;
	height: number;
}>();

const emit = defineEmits<{
	restored: [];
}>();

const { showSnackbar } = useFeedback();

async function onRestoreDefault() {
	await restoreDefaultWindowSize();
	showSnackbar('已恢复默认窗口大小', 'success');
	emit('restored');
}
</script>

<template>
	<v-dialog v-model="open" max-width="360">
		<v-card class="mx-12 dialog-card" density="compact">
			<v-card-title>窗口大小</v-card-title>
			<v-card-text style="padding: 0 24px">
				<v-container style="padding: 0">
					<v-text-field
						:model-value="width"
						label="宽度"
						variant="solo-filled"
						density="compact"
						readonly
						suffix="px"
					/>
					<v-text-field
						:model-value="height"
						label="高度"
						variant="solo-filled"
						density="compact"
						readonly
						suffix="px"
					/>
					<div class="text-caption text-medium-emphasis">
						拖动窗口边缘可调整大小，数值会实时更新
					</div>
				</v-container>
			</v-card-text>

			<v-divider style="margin-top: 10px" />

			<v-card-actions style="padding: 0 24px">
				<v-btn color="warning" variant="text" @click="onRestoreDefault">
					恢复默认大小
				</v-btn>
				<v-spacer />
				<v-btn color="primary" variant="text" @click="open = false">关闭</v-btn>
			</v-card-actions>
		</v-card>
	</v-dialog>
</template>
