export class Jx3Server {
	public id: number | null = null;
	public zone: string = '';
	public server: string = '';
	public status: string = '';

	constructor(zone = '', server = '', status = '') {
		this.zone = zone;
		this.server = server;
		this.status = status;
	}
}
