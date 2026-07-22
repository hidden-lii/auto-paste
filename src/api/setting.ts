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
