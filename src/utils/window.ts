import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import {
	getDefaultWindowSize as fetchDefaultWindowSize,
	getSavedWindowSize,
	saveWindowSizeToDb
} from '../api/window';

const LEGACY_STORAGE_KEY = 'window-size';

export const DEFAULT_WINDOW_WIDTH = 460;

export interface WindowSize {
	width: number;
	height: number;
}

export async function getDefaultWindowSize(): Promise<WindowSize> {
	return await fetchDefaultWindowSize();
}

export async function saveWindowSize(size: WindowSize): Promise<void> {
	await saveWindowSizeToDb(size);
}

export async function applyWindowSize(width: number, height: number): Promise<void> {
	await appWindow.setSize(new LogicalSize(width, height));
}

export async function getCurrentWindowSize(): Promise<WindowSize> {
	const size = await appWindow.outerSize();
	const scaleFactor = await appWindow.scaleFactor();
	return {
		width: Math.round(size.width / scaleFactor),
		height: Math.round(size.height / scaleFactor)
	};
}

export async function restoreDefaultWindowSize(): Promise<WindowSize> {
	const size = await getDefaultWindowSize();
	await applyWindowSize(size.width, size.height);
	await saveWindowSize(size);
	return size;
}

export async function migrateWindowSizeFromLocalStorage(): Promise<void> {
	const saved = await getSavedWindowSize();
	if (saved) {
		return;
	}

	try {
		const raw = localStorage.getItem(LEGACY_STORAGE_KEY);
		if (!raw) {
			return;
		}

		const parsed = JSON.parse(raw) as WindowSize;
		if (
			typeof parsed.width === 'number' &&
			typeof parsed.height === 'number' &&
			parsed.width > 0 &&
			parsed.height > 0
		) {
			await saveWindowSize(parsed);
			localStorage.removeItem(LEGACY_STORAGE_KEY);
		}
	} catch {
		// ignore invalid legacy value
	}
}

/** 将旧版以物理像素保存的尺寸迁移为逻辑像素 */
export async function normalizeSavedWindowSize(): Promise<void> {
	const saved = await getSavedWindowSize();
	if (!saved) {
		return;
	}

	const scaleFactor = await appWindow.scaleFactor();
	const threshold = DEFAULT_WINDOW_WIDTH * 1.5;
	if (saved.width <= threshold && saved.height <= threshold * 1.6) {
		return;
	}

	const normalized = {
		width: Math.round(saved.width / scaleFactor),
		height: Math.round(saved.height / scaleFactor)
	};
	await saveWindowSize(normalized);
	await applyWindowSize(normalized.width, normalized.height);
}
