export class Role {
	public id: number | null = null;
	public account_id: number | null = null;
	public role_id: string = '';
	public server: string = '';

	constructor(roleId = '', server = '') {
		this.role_id = roleId;
		this.server = server;
	}
}
