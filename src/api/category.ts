import { invoke } from '@tauri-apps/api/tauri';
import { Category } from '../entity/category';

export async function queryAllCategories(): Promise<Category[]> {
	const res = await invoke('query_all_category');
	return Array.isArray(res) ? (res as Category[]) : [];
}

export async function createCategory(category: Category): Promise<boolean> {
	const res = await invoke('create_category', { category });
	return !!res && typeof res === 'boolean' && res;
}

export async function updateCategory(category: Category): Promise<boolean> {
	const res = await invoke('update_category', { category });
	return !!res && typeof res === 'boolean' && res;
}

export async function deleteCategoryById(id: number): Promise<boolean> {
	const res = await invoke('delete_category_by_id', { id });
	return !!res && typeof res === 'boolean' && res;
}

export async function reorderCategories(ids: number[]): Promise<boolean> {
	const res = await invoke('reorder_categories', { ids });
	return !!res && typeof res === 'boolean' && res;
}
