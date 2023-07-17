export { Account };

class Account {
	public id: number | null = null;
	public name: string = '';
	public username: string = '';
	public password: string = '';
	public sequence: number = 1;
	public liked: boolean = false;
	public description: string | null = null;
	public last_update_time: string | null = null;
	public show: boolean = false;
	public account_category_ids: number[] = [];

	constructor() {}
}
