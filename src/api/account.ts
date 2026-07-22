import { invoke } from '@tauri-apps/api/tauri';
import { Account } from '../entity/account';

export async function queryAllAccounts(): Promise<Account[]> {
	const res = await invoke('query_all_accounts');
	return Array.isArray(res) ? (res as Account[]) : [];
}

export async function queryAccountsByValue(
	account: Account,
	withLiked: boolean,
	categoryId: number | null | undefined
): Promise<Account[]> {
	const res = await invoke('query_accounts_by_value', {
		account,
		withLiked,
		categoryId
	});
	return Array.isArray(res) ? (res as Account[]) : [];
}

export async function insertAccount(account: Account): Promise<boolean> {
	const res = await invoke('insert_account', { account });
	return !!res && typeof res === 'boolean' && res;
}

export async function updateAccount(account: Account): Promise<boolean> {
	const res = await invoke('update_account', { account });
	return !!res && typeof res === 'boolean' && res;
}

export async function updateLike(id: number, liked: boolean): Promise<boolean> {
	const res = await invoke('update_like', { id, liked });
	return !!res && typeof res === 'boolean' && res;
}

export async function deleteAccount(id: number): Promise<boolean> {
	const res = await invoke('delete_account', { id });
	return !!res && typeof res === 'boolean' && res;
}

export async function reorderAccounts(ids: number[]): Promise<boolean> {
	const res = await invoke('reorder_accounts', { ids });
	return !!res && typeof res === 'boolean' && res;
}
