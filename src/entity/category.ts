export { Category };

class Category {
	public id?: number | null;
	public name: string;
	public sequence?: number | null;
	public account_ids: number[] = [];

	constructor(id: number | null, name: string) {
		this.id = id;
		this.name = name;
	}
}
