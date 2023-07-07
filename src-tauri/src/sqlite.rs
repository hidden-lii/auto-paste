use crate::entity::account::Account;
use crate::entity::account_group::AccountGroup;
use crate::entity::group::Group;
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(Connection::open("auto_paste.db").unwrap());
}

pub(crate) fn create_if_not_exists() -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    // 创建 account 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS account (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            sequence INTEGER NOT NULL DEFAULT 1,
            liked INTEGER NOT NULL DEFAULT 0,
            description TEXT,
            last_update_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime'))
        );",
        [],
    )?;

    // 创建 group 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS group (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            sequence INTEGER,
            last_update_time TEXT)",
        [],
    )?;

    // 创建 AccountGroup 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS account_group (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            group_id INTEGER NOT NULL,
            last_update_time TEXT)",
        [],
    )?;

    // 创建 account 触发器
    conn.execute(
        "
        CREATE TRIGGER IF NOT EXISTS update_timestamp
        AFTER UPDATE ON account
        FOR EACH ROW
        BEGIN
            UPDATE account SET last_update_time = (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime')) WHERE id = OLD.id;
        END;
        ",
        [],
    )?;

    // 创建 AccountGroup 表的触发器
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_account_group_last_update_time
         AFTER UPDATE ON account_group
         FOR EACH ROW
         BEGIN
            UPDATE account_group SET last_update_time = (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime')) WHERE id = OLD.id;
         END",
        [],
    )?;

    // 创建 Group 表的触发器
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_group_last_update_time
         AFTER UPDATE ON group
         FOR EACH ROW
         BEGIN
            UPDATE Group SET last_update_time = (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime')) WHERE id = OLD.id;
         END",
        [],
    )?;

    Ok(())
}

pub(crate) fn insert_account(account: &Account) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    let default_description = "这个人好懒,没有给他写备注信息呢┓(´∀`)┏".to_string();
    conn.execute(
        "INSERT INTO account (name, username, password, sequence, liked, description, last_update_time)
        VALUES (?, ?, ?, IFNULL(?, 1), ?, ?, datetime('now'))",
        params![
            account.name,
            account.username,
            account.password,
            account.sequence,
            account.liked.unwrap(),
            if account.description.is_none() || account.description.clone().unwrap().is_empty() { default_description } else { account.description.clone().unwrap() },
        ],
    )?;

    Ok(())
}

pub(crate) fn update_account(account: &Account) -> Result<()> {
    if account.id.is_none() {
        insert_account(account)?;
    } else {
        let conn = DB_CONNECTION.lock().unwrap();
        let default_description = "这个人好懒,没有给他写备注信息呢┓(´∀`)┏".to_string();
        conn.execute(
            "UPDATE account SET name = ?, username = ?, password = ?, sequence = ?, liked = ?, description = ? WHERE id = ?",
            params![
                account.name,
                account.username,
                account.password,
                account.sequence,
                account.liked,
                if account.description.is_none() || account.description.clone().unwrap().is_empty() { default_description } else { account.description.clone().unwrap() },
                account.id,
            ],
        )?;
    }
    Ok(())
}

pub(crate) fn like_account(id: i32, liked: bool) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute(
        "UPDATE account SET liked = ? WHERE id = ?",
        params![liked, id],
    )?;

    Ok(())
}

pub(crate) fn delete_by_id(id: i32) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute("DELETE FROM account WHERE id = ?", params![id])?;

    Ok(())
}

pub(crate) fn query_all_accounts() -> Result<Vec<Account>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM account")?;

    Ok(_do_query(&mut stmt)?)
}

pub(crate) fn query_accounts_by_value(account: &Account, with_liked: bool) -> Result<Vec<Account>> {
    let mut query = "SELECT * FROM account WHERE 1 = 1".to_string();

    let mut sub_queries = Vec::new();

    if !account.name.is_empty() {
        sub_queries.push(format!("name LIKE '%{}%'", account.name));
    }

    if !account.username.is_empty() {
        sub_queries.push(format!("username LIKE '%{}%'", account.username));
    }

    if !sub_queries.is_empty() {
        query += &*format!(" AND ( {} )", &sub_queries.join(" OR "));
    }

    if with_liked {
        query += &*format!(" AND liked = {}", account.liked.unwrap());
    }

    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare(&query)?;

    Ok(_do_query(&mut stmt)?)
}

fn _do_query(stmt: &mut rusqlite::Statement) -> Result<Vec<Account>> {
    let rows = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
            sequence: row.get(4)?,
            liked: row.get(5)?,
            description: row.get(6)?,
            last_update_time: row.get(7)?,
        })
    })?;

    let mut accounts = Vec::new();
    for row in rows {
        accounts.push(row?);
    }

    Ok(accounts)
}
