import { invoke } from '@tauri-apps/api/tauri';

export interface DisplaySettings {
	hideUsername: boolean;
	hidePassword: boolean;
}

interface DisplaySettingsResponse {
	hide_username: boolean;
	hide_password: boolean;
}

export async function getDisplaySettings(): Promise<DisplaySettings> {
	const result = await invoke<DisplaySettingsResponse>('get_display_settings');
	return {
		hideUsername: result.hide_username,
		hidePassword: result.hide_password
	};
}

export async function saveDisplaySettings(
	settings: DisplaySettings
): Promise<boolean> {
	const result = await invoke<boolean>('save_display_settings', {
		hideUsername: settings.hideUsername,
		hidePassword: settings.hidePassword
	});
	return !!result;
}

export async function getAppVersion(): Promise<string> {
	return await invoke<string>('get_app_version');
}

export async function getCurrentAppVersion(): Promise<string> {
	return await invoke<string>('get_current_app_version');
}

export async function getExportFields(): Promise<string[]> {
	const result = await invoke<string[]>('get_export_fields');
	return Array.isArray(result) && result.length > 0 ? result : [];
}

export async function saveExportFields(fields: string[]): Promise<boolean> {
	const result = await invoke<boolean>('save_export_fields', { fields });
	return !!result;
}

export async function getFavoriteFilter(): Promise<number> {
	const result = await invoke<number>('get_favorite_filter');
	return typeof result === 'number' ? result : 0;
}

export async function saveFavoriteFilter(value: number): Promise<boolean> {
	const result = await invoke<boolean>('save_favorite_filter', { value });
	return !!result;
}
