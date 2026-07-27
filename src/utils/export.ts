import { Account } from '../entity/account';
import { Jx3Server } from '../entity/jx3Server';

export const EXPORT_FIELD_OPTIONS = [
	{ value: 'name', label: '名称' },
	{ value: 'username', label: '账号' },
	{ value: 'password', label: '密码' },
	{ value: 'roles', label: '角色区服' },
	{ value: 'description', label: '备注' },
	{ value: 'liked', label: '收藏状态' }
] as const;

export type ExportField = (typeof EXPORT_FIELD_OPTIONS)[number]['value'];

const DEFAULT_FIELDS: ExportField[] = [
	'name',
	'username',
	'password',
	'roles',
	'description'
];

export function getDefaultExportFields(): ExportField[] {
	return [...DEFAULT_FIELDS];
}

function findServerMeta(
	serverName: string,
	servers: Jx3Server[]
): Jx3Server | undefined {
	return servers.find((item) => item.server === serverName);
}

function formatRoleLine(
	roleId: string,
	serverName: string,
	servers: Jx3Server[]
): string {
	const meta = findServerMeta(serverName, servers);
	if (meta) {
		return `  - ${roleId} @ ${meta.zone}·${meta.server} (${meta.status})`;
	}
	return `  - ${roleId} @ ${serverName}`;
}

export function formatAccountForShare(
	account: Account,
	fields: ExportField[],
	servers: Jx3Server[] = []
): string {
	const lines: string[] = [];
	const fieldSet = new Set(fields);

	if (fieldSet.has('name')) {
		lines.push(`名称: ${account.name}`);
	}
	if (fieldSet.has('username')) {
		lines.push(`账号: ${account.username}`);
	}
	if (fieldSet.has('password')) {
		lines.push(`密码: ${account.password}`);
	}
	if (fieldSet.has('liked')) {
		lines.push(`收藏状态: ${account.liked ? '已收藏' : '未收藏'}`);
	}
	if (fieldSet.has('roles')) {
		const roles = account.roles ?? [];
		if (roles.length > 0) {
			lines.push('角色区服:');
			for (const role of roles) {
				lines.push(formatRoleLine(role.role_id, role.server, servers));
			}
		} else {
			lines.push('角色区服: 无');
		}
	}
	if (fieldSet.has('description')) {
		lines.push(`备注: ${account.description ?? ''}`);
	}

	return lines.join('\n');
}
