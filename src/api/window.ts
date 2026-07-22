import { invoke } from '@tauri-apps/api/tauri';
import type { WindowSize } from '../utils/window';

export async function getSavedWindowSize(): Promise<WindowSize | null> {
	const size = await invoke<WindowSize | null>('get_saved_window_size');
	if (
		size &&
		typeof size.width === 'number' &&
		typeof size.height === 'number' &&
		size.width > 0 &&
		size.height > 0
	) {
		return size;
	}
	return null;
}

export async function getDefaultWindowSize(): Promise<WindowSize> {
	return await invoke<WindowSize>('get_default_window_size');
}

export async function saveWindowSizeToDb(size: WindowSize): Promise<boolean> {
	const result = await invoke<boolean>('save_window_size', {
		width: size.width,
		height: size.height
	});
	return !!result;
}
