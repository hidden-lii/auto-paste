<script setup lang="ts">
import { computed, ref } from 'vue';

const likeType = defineModel<number>('likeType', { required: true });

const menuOpen = ref(false);

const heartIcon = computed(() => {
	switch (likeType.value) {
		case 1:
			return 'mdi-heart';
		case 2:
			return 'mdi-heart-off';
		default:
			return 'mdi-heart-outline';
	}
});

function onSelect(value: number | null) {
	if (value === null) {
		return;
	}
	likeType.value = value;
	menuOpen.value = false;
}
</script>

<template>
	<v-menu
		v-model="menuOpen"
		location="top start"
		:close-on-content-click="false"
	>
		<template #activator="{ props: menuProps }">
			<v-btn v-bind="menuProps" size="small" icon class="ms-5">
				<v-icon :icon="heartIcon" />
			</v-btn>
		</template>

		<v-list density="compact" nav class="favorite-filter-menu">
			<v-list-subheader class="favorite-filter-subheader">
				收藏筛选
			</v-list-subheader>
			<v-list-item class="favorite-filter-radio">
				<v-radio-group
					:model-value="likeType"
					density="compact"
					hide-details
					@update:model-value="onSelect"
				>
					<v-radio label="全部" :value="0" />
					<v-radio label="收藏" :value="1" />
					<v-radio label="未收藏" :value="2" />
				</v-radio-group>
			</v-list-item>
		</v-list>
	</v-menu>
</template>

<style scoped>
.favorite-filter-menu {
	width: 160px;
	padding: 0;
}

.favorite-filter-subheader {
	min-height: 28px;
	padding-top: 4px;
	padding-bottom: 0;
	font-size: 0.75rem;
}

.favorite-filter-radio {
	min-height: auto;
	padding-top: 0;
	padding-bottom: 8px;
}

.favorite-filter-radio :deep(.v-radio) {
	min-height: 32px;
}
</style>
