import { onMounted, onUnmounted, ref } from 'vue';
import { appWindow } from '@tauri-apps/api/window';
import {
	getCurrentWindowSize,
	saveWindowSize,
	type WindowSize
} from '../utils/window';

function debounce<T extends (...args: never[]) => void>(fn: T, delay: number) {
	let timer: ReturnType<typeof setTimeout> | undefined;
	return (...args: Parameters<T>) => {
		clearTimeout(timer);
		timer = setTimeout(() => fn(...args), delay);
	};
}

export function useWindowSize() {
	const width = ref(0);
	const height = ref(0);
	let unlisten: (() => void) | null = null;

	async function refresh() {
		const size = await getCurrentWindowSize();
		width.value = size.width;
		height.value = size.height;
		return size;
	}

	const persistSize = debounce((size: WindowSize) => {
		void saveWindowSize(size);
	}, 300);

	onMounted(async () => {
		await refresh();
		unlisten = await appWindow.onResized(async () => {
			const size = await refresh();
			persistSize(size);
		});
	});

	onUnmounted(() => {
		unlisten?.();
	});

	return { width, height, refresh };
}
