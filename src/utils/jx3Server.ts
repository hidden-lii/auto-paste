import { Jx3Server } from '../entity/jx3Server';

const STATUS_ORDER: Record<string, number> = {
	爆满: 0,
	拥挤: 1,
	正常: 2
};

function statusOrder(status: string): number {
	return STATUS_ORDER[status] ?? 99;
}

export function compareJx3Servers(a: Jx3Server, b: Jx3Server): number {
	const statusDiff = statusOrder(a.status) - statusOrder(b.status);
	if (statusDiff !== 0) {
		return statusDiff;
	}
	return a.server.localeCompare(b.server, 'zh-CN', { sensitivity: 'base' });
}

export function sortJx3Servers(servers: Jx3Server[]): Jx3Server[] {
	return [...servers].sort(compareJx3Servers);
}

export interface Jx3ServerOption {
	title: string;
	value: string;
	zone: string;
	status: string;
}

export function filterJx3ServerOption(
	_value: string,
	query: string,
	item?: { raw: Jx3ServerOption }
): boolean {
	const q = query.trim().toLowerCase();
	if (!q) {
		return true;
	}
	const raw = item?.raw;
	if (!raw) {
		return false;
	}
	const haystack = `${raw.value} ${raw.zone} ${raw.status}`.toLowerCase();
	return haystack.includes(q);
}
