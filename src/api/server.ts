import { invoke } from '@tauri-apps/api/tauri';
import { Jx3Server } from '../entity/jx3Server';

export interface NetworkSyncSettings {
	enabled: boolean;
	prompted: boolean;
	lastSync: string | null;
}

interface NetworkSyncSettingsResponse {
	enabled: boolean;
	prompted: boolean;
	last_sync: string | null;
}

export async function queryAllJx3Servers(): Promise<Jx3Server[]> {
	const res = await invoke('query_all_jx3_servers');
	return Array.isArray(res) ? (res as Jx3Server[]) : [];
}

export async function syncJx3Servers(forceFallback = false): Promise<boolean> {
	const res = await invoke('sync_jx3_servers', { forceFallback });
	return !!res;
}

export async function getNetworkSyncSettings(): Promise<NetworkSyncSettings> {
	const result = await invoke<NetworkSyncSettingsResponse>(
		'get_network_sync_settings'
	);
	return {
		enabled: result.enabled,
		prompted: result.prompted,
		lastSync: result.last_sync
	};
}

export async function saveNetworkSyncSettings(
	enabled: boolean,
	prompted: boolean
): Promise<boolean> {
	const res = await invoke<boolean>('save_network_sync_settings', {
		enabled,
		prompted
	});
	return !!res;
}
