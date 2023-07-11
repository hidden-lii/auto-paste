use crate::entity::account::Account;
use crate::entity::category::Category;
use rusqlite::{named_params, params, Connection, Result, ToSql};
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

    // 创建 category 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS category (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            sequence INTEGER NOT NULL DEFAULT 1,
            last_update_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime'))
        );",
        [],
    )?;

    // 创建 account_category 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS account_category (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            category_id INTEGER NOT NULL,
            last_update_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime'))
        );",
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

    // 创建 category 表的触发器
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_group_last_update_time
            AFTER UPDATE ON category
            FOR EACH ROW
            BEGIN
            UPDATE category SET last_update_time = (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime')) WHERE id = OLD.id;
            END",
        [],
    )?;

    // 创建 account_category 表的触发器
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_account_group_last_update_time
         AFTER UPDATE ON account_category
         FOR EACH ROW
         BEGIN
            UPDATE account_category SET last_update_time = (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime')) WHERE id = OLD.id;
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

    Ok(_do_query_accounts(&mut stmt, &[])?)
}

pub(crate) fn query_accounts_by_value(account: &Account, with_liked: bool) -> Result<Vec<Account>> {
    let mut query = "SELECT * FROM account WHERE 1 = 1".to_string();

    let mut sub_queries = Vec::new();
    let mut params: Vec<(&str, &dyn ToSql)> = Vec::new();

    if !account.name.is_empty() {
        sub_queries.push("name LIKE '%' || :name || '%'");
        params.push((":name", &account.name));
    }

    if !account.username.is_empty() {
        sub_queries.push("username LIKE '%' || :username || '%'");
        params.push((":username", &account.username));
    }

    if !sub_queries.is_empty() {
        query += &format!(" AND ( {} )", &sub_queries.join(" OR "));
    }

    if with_liked {
        query += &format!(" AND liked = {}", account.liked.unwrap());
    }

    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare(&query)?;

    Ok(_do_query_accounts(&mut stmt, &params)?)
}

fn _do_query_accounts(
    stmt: &mut rusqlite::Statement,
    params: &[(&str, &dyn ToSql)],
) -> Result<Vec<Account>> {
    let rows = stmt.query_map(params, |row| {
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

pub(crate) fn create_category(category: &Category) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute(
        "INSERT INTO category (name, sequence, last_update_time)
        VALUES (?, IFNULL(?, 1), datetime('now'))",
        params![category.name, category.sequence],
    )?;

    Ok(())
}

pub(crate) fn query_all_categories() -> Result<Vec<Category>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM category")?;

    Ok(_do_query_categories(&mut stmt, &[])?)
}

fn _do_query_categories(
    stmt: &mut rusqlite::Statement,
    params: &[(&str, &dyn ToSql)],
) -> Result<Vec<Category>> {
    let rows = stmt.query_map(params, |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            sequence: row.get(2)?,
            last_update_time: row.get(3)?,
        })
    })?;

    let mut groups = Vec::new();
    for row in rows {
        groups.push(row?);
    }

    Ok(groups)
}

pub(crate) fn update_category(category: &Category) -> Result<()> {
    if category.id.is_none() {
        create_category(category)?;
    } else {
        let conn = DB_CONNECTION.lock().unwrap();
        conn.execute(
            "UPDATE category SET name = ?, sequence = ? WHERE id = ?",
            params![category.name, category.sequence, category.id],
        )?;
    }

    Ok(())
}

pub(crate) fn delete_category_by_id(id: i32) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute("DELETE FROM category WHERE id = ?", params![id])?;

    Ok(())
}

pub(crate) fn batch_create_account_categories(
    account_id: i32,
    category_ids: Vec<i32>,
) -> Result<()> {
    let conn = &mut DB_CONNECTION.lock().unwrap();
    let batch = conn.transaction()?;

    for category_id in category_ids {
        batch.execute(
            "INSERT INTO account_category (account_id, category_id, last_update_time)
            VALUES (?, ?, datetime('now'))",
            params![account_id, category_id],
        )?;
    }
    batch.commit()?;

    Ok(())
}

pub(crate) fn batch_delete_account_categories(account_category_ids: Vec<i32>) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();

    conn.execute(
        &format!(
            "DELETE FROM account_category WHERE id in ({})",
            account_category_ids
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<String>>()
                .join(",")
        ),
        [],
    )?;

    Ok(())
}

pub(crate) fn query_categries_by_account_id(account_id: i32) -> Result<Vec<Category>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT g.id, g.name, g.sequence, g.last_update_time FROM category g
        INNER JOIN account_category ag ON g.id = ag.category_id
        WHERE ag.account_id = :account_id",
    )?;

    Ok(_do_query_categories(
        &mut stmt,
        named_params! {":acount":account_id},
    )?)
}

pub(crate) fn query_accounts_by_category_id(category_id: i32) -> Result<Vec<Account>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT a.id, a.name, a.username, a.password, a.sequence, a.liked, a.description, a.last_update_time FROM account a
        INNER JOIN account_category ag ON a.id = ag.account_id
        WHERE ag.category_id = :category_id",
    )?;

    Ok(_do_query_accounts(
        &mut stmt,
        named_params! {":category_id":category_id},
    )?)
}
