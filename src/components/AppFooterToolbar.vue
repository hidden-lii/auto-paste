<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
	likeType: number;
	alwaysOnTop: boolean;
}>();

const likedTooltip = computed(() => {
	switch (props.likeType) {
		case 0:
			return '仅显示收藏';
		case 1:
			return '仅显示未收藏';
		case 2:
			return '显示全部账号';
		default:
			return '切换收藏筛选';
	}
});
const alwaysOnTopTooltip = computed(() =>
	props.alwaysOnTop ? '取消置顶' : '窗口置顶'
);

const emit = defineEmits<{
	'insert-account': [];
	'toggle-liked': [];
	'toggle-always-on-top': [];
	refresh: [];
}>();
</script>

<template>
	<v-toolbar density="comfortable">
		<template #prepend>
			<v-tooltip text="添加账号" location="top">
				<template #activator="{ props: tooltipProps }">
					<v-btn
						v-bind="tooltipProps"
						size="small"
						icon
						@click="emit('insert-account')"
					>
						<v-icon icon="mdi-account-plus" />
					</v-btn>
				</template>
			</v-tooltip>
		</template>

		<v-tooltip :text="likedTooltip" location="top">
			<template #activator="{ props: tooltipProps }">
				<v-btn
					v-bind="tooltipProps"
					:icon="
						likeType === 0
							? 'mdi-heart-off'
							: likeType === 1
								? 'mdi-heart'
								: 'mdi-heart-outline'
					"
					class="ms-5"
					size="small"
					@click="emit('toggle-liked')"
				/>
			</template>
		</v-tooltip>

		<v-divider
			class="mx-3 align-self-center"
			length="24"
			thickness="2"
			vertical
		/>

		<v-tooltip :text="alwaysOnTopTooltip" location="top">
			<template #activator="{ props: tooltipProps }">
				<v-btn
					v-bind="tooltipProps"
					size="small"
					icon
					@click="emit('toggle-always-on-top')"
				>
					<v-icon :icon="alwaysOnTop ? 'mdi-pin' : 'mdi-pin-off'" />
				</v-btn>
			</template>
		</v-tooltip>

		<v-tooltip text="刷新列表" location="top">
			<template #activator="{ props: tooltipProps }">
				<v-btn
					v-bind="tooltipProps"
					icon="mdi-reload"
					size="small"
					@click="emit('refresh')"
				/>
			</template>
		</v-tooltip>
	</v-toolbar>
</template>
