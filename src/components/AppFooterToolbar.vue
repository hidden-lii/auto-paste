<script setup lang="ts">
import FavoriteFilterPanel from './FavoriteFilterPanel.vue';

const props = defineProps<{
	alwaysOnTop: boolean;
}>();

const likeType = defineModel<number>('likeType', { required: true });

const emit = defineEmits<{
	'insert-account': [];
	'toggle-always-on-top': [];
	refresh: [];
}>();

const alwaysOnTopTooltip = (alwaysOnTop: boolean) =>
	alwaysOnTop ? '取消置顶' : '窗口置顶';
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

		<FavoriteFilterPanel v-model:like-type="likeType" />

		<v-divider
			class="mx-3 align-self-center"
			length="24"
			thickness="2"
			vertical
		/>

		<v-tooltip :text="alwaysOnTopTooltip(props.alwaysOnTop)" location="top">
			<template #activator="{ props: tooltipProps }">
				<v-btn
					v-bind="tooltipProps"
					size="small"
					icon
					@click="emit('toggle-always-on-top')"
				>
					<v-icon :icon="props.alwaysOnTop ? 'mdi-pin' : 'mdi-pin-off'" />
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
