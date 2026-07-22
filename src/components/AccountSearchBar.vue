<script setup lang="ts">
defineProps<{
	types: string[];
	selectedType: string;
	keyword: string;
}>();

const emit = defineEmits<{
	'update:selectedType': [value: string];
	'update:keyword': [value: string];
	search: [];
}>();
</script>

<template>
	<v-container class="search-container">
		<v-row>
			<v-col cols="4">
				<v-select
					:model-value="selectedType"
					:items="types"
					label="搜索类型"
					variant="solo-filled"
					hide-details
					density="compact"
					@update:model-value="emit('update:selectedType', $event)"
				/>
			</v-col>

			<v-col cols="8">
				<v-text-field
					label="输入关键词"
					variant="solo-filled"
					:model-value="keyword"
					hide-details
					density="compact"
					@update:model-value="emit('update:keyword', $event)"
					@keyup.enter="emit('search')"
				>
					<template #append-inner>
						<v-btn icon size="small" @click="emit('search')">
							<v-icon icon="mdi-magnify" />
						</v-btn>
					</template>
				</v-text-field>
			</v-col>
		</v-row>
	</v-container>
</template>
