<script setup lang="ts">
import { ref, watch } from 'vue';
import {
	EXPORT_FIELD_OPTIONS,
	ExportField,
	getDefaultExportFields
} from '../utils/export';
import { getExportFields, saveExportFields } from '../api/setting';
import { useFeedback } from '../utils/feedback';

const open = defineModel<boolean>({ required: true });

const { showSnackbar } = useFeedback();
const selectedFields = ref<ExportField[]>(getDefaultExportFields());

watch(open, async (value) => {
	if (!value) {
		return;
	}
	const fields = await getExportFields();
	selectedFields.value =
		fields.length > 0
			? (fields as ExportField[])
			: getDefaultExportFields();
});

function onClose() {
	open.value = false;
}

async function onSave() {
	if (selectedFields.value.length === 0) {
		showSnackbar('请至少选择一个导出字段', 'error');
		return;
	}
	const success = await saveExportFields(selectedFields.value);
	if (success) {
		showSnackbar('导出设置已保存', 'success');
		open.value = false;
	} else {
		showSnackbar('保存失败', 'error');
	}
}
</script>

<template>
	<v-dialog v-model="open" max-width="360">
		<v-card class="mx-12 dialog-card" density="compact">
			<v-card-title>导出设置</v-card-title>
			<v-card-text style="padding: 0 24px">
				<v-container style="padding: 0">
					<div class="text-caption text-medium-emphasis mb-2">
						选择分享时复制的字段
					</div>
					<v-checkbox
						v-for="option in EXPORT_FIELD_OPTIONS"
						:key="option.value"
						v-model="selectedFields"
						:label="option.label"
						:value="option.value"
						density="compact"
						hide-details
					/>
				</v-container>
			</v-card-text>
			<v-divider style="margin-top: 10px" />
			<v-card-actions style="padding: 0 24px">
				<v-spacer />
				<v-btn color="error" variant="text" @click="onClose">关闭</v-btn>
				<v-btn color="success" variant="text" @click="onSave">保存</v-btn>
			</v-card-actions>
		</v-card>
	</v-dialog>
</template>
