import { getDisplaySettings, saveDisplaySettings } from '../api/setting';

export const DEFAULT_HIDE_USERNAME = false;
export const DEFAULT_HIDE_PASSWORD = true;

export async function loadDisplaySettings(): Promise<{
	hideUsername: boolean;
	hidePassword: boolean;
}> {
	try {
		return await getDisplaySettings();
	} catch {
		return {
			hideUsername: DEFAULT_HIDE_USERNAME,
			hidePassword: DEFAULT_HIDE_PASSWORD
		};
	}
}

export async function persistDisplaySettings(
	hideUsername: boolean,
	hidePassword: boolean
): Promise<void> {
	await saveDisplaySettings({ hideUsername, hidePassword });
}
