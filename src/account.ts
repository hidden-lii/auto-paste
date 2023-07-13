export {Account};

class Account {
    public id: number | null = null;
    public name: string | null = null;
    public username: string | null = null;
    public password: string | null = null;
    public sequence: number | null = null;
    public liked: boolean = false;
    public description: string | null = null;
    public last_update_time: string | null = null;
    public show: boolean = false;

    constructor(){
    }
}