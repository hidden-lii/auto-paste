use crate::entity::account::Account;
use crate::entity::category::Category;
use rusqlite::{params, Connection, Result, ToSql};
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
    let conn = &mut DB_CONNECTION.lock().unwrap();

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

    let account_id = conn.last_insert_rowid() as i32;

    let batch = conn.transaction()?;

    if let Some(account_category_ids) = &account.account_category_ids {
        for category_id in account_category_ids {
            batch.execute(
                "INSERT INTO account_category (account_id, category_id, last_update_time)
                    VALUES (?, ?, datetime('now'))",
                params![account_id, category_id],
            )?;
        }
    }

    batch.commit()?;

    Ok(())
}

pub(crate) fn update_account(account: &Account) -> Result<()> {
    if account.id.is_none() {
        insert_account(account)?;
    } else {
        let conn = &mut DB_CONNECTION.lock().unwrap();
        let batch = conn.transaction()?;

        let default_description = "这个人好懒,没有给他写备注信息呢┓(´∀`)┏".to_string();
        batch.execute(
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

        batch.execute(
            "DELETE FROM account_category WHERE account_id = ?",
            params![account.id],
        )?;

        if let Some(account_category_ids) = &account.account_category_ids {
            for category_id in account_category_ids {
                batch.execute(
                    "INSERT INTO account_category (account_id, category_id, last_update_time)
                        VALUES (?, ?, datetime('now'))",
                    params![account.id, category_id],
                )?;
            }
        }

        batch.commit()?;
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
    let conn = &mut DB_CONNECTION.lock().unwrap();
    let batch = conn.transaction()?;

    batch.execute("DELETE FROM account WHERE id = ?", params![id])?;

    batch.execute(
        "DELETE FROM account_category WHERE account_id = ?",
        params![id],
    )?;

    batch.commit()?;

    Ok(())
}

pub(crate) fn query_all_accounts() -> Result<Vec<Account>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("
        SELECT a.*, ac.account_category_ids
        FROM account a
        LEFT JOIN (
            SELECT account_id, GROUP_CONCAT(category_id) AS account_category_ids
            FROM account_category
            GROUP BY account_id
        ) AS ac ON a.id = ac.account_id"
    )?;

    Ok(_do_query_accounts(&mut stmt, &[])?)
}

pub(crate) fn query_accounts_by_value(
    account: &Account,
    with_liked: bool,
    category_id: i32,
) -> Result<Vec<Account>> {
    let mut query = "
        SELECT a.*, ac.account_category_ids
        FROM account a
        LEFT JOIN (
            SELECT account_id, GROUP_CONCAT(category_id) AS account_category_ids
            FROM account_category
            GROUP BY account_id
        ) AS ac ON a.id = ac.account_id
        WHERE 1 = 1
    "
    .to_string();

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

    if category_id > 0 {
        query +=
            " AND id IN (SELECT account_id FROM account_category WHERE category_id = :category_id)";
        params.push((":category_id", &category_id));
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
        let account_category_ids: Result<Option<String>> = row.get(8);
        let account_category_ids: Option<Vec<i32>> = match account_category_ids {
            Ok(Some(ids_str)) => {
                let ids: Vec<i32> = ids_str.split(',').filter_map(|s| s.parse().ok()).collect();
                Some(ids)
            }
            _ => Some(Vec::new()),
        };

        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
            sequence: row.get(4)?,
            liked: row.get(5)?,
            description: row.get(6)?,
            last_update_time: row.get(7)?,
            account_category_ids,
        })
    })?;

    let mut accounts = Vec::new();
    for row in rows {
        accounts.push(row?);
    }

    Ok(accounts)
}

pub(crate) fn create_category(category: &Category) -> Result<()> {
    let conn = &mut DB_CONNECTION.lock().unwrap();
    conn.execute(
        "INSERT INTO category (name, sequence, last_update_time)
        VALUES (?, IFNULL(?, 1), datetime('now'))",
        params![category.name, category.sequence],
    )?;

    let category_id = conn.last_insert_rowid() as i32;

    let batch = conn.transaction()?;

    if let Some(account_ids) = &category.account_ids {
        for account_id in account_ids {
            batch.execute(
                "INSERT INTO account_category (account_id, category_id, last_update_time)
                    VALUES (?, ?, datetime('now'))",
                params![account_id, category_id],
            )?;
        }
    }

    batch.commit()?;

    Ok(())
}

pub(crate) fn query_all_categories() -> Result<Vec<Category>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("
        SELECT c.*, ac.account_category_ids
        FROM category c
        LEFT JOIN (
            SELECT category_id, GROUP_CONCAT(account_id) AS account_category_ids
            FROM account_category
            GROUP BY category_id
        ) AS ac ON c.id = ac.category_id
        WHERE 1 = 1
    ")?;

    let rows = stmt.query_map([], |row| {
        let account_category_ids: Result<Option<String>> = row.get(4);
        let account_category_ids: Option<Vec<i32>> = match account_category_ids {
            Ok(Some(ids_str)) => {
                let ids: Vec<i32> = ids_str.split(',').filter_map(|s| s.parse().ok()).collect();
                Some(ids)
            }
            _ => Some(Vec::new()),
        };

        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            sequence: row.get(2)?,
            last_update_time: row.get(3)?,
            account_ids: account_category_ids,
        })
    })?;

    let mut categories = Vec::new();
    for row in rows {
        categories.push(row?);
    }

    Ok(categories)
}

pub(crate) fn update_category(category: &Category) -> Result<()> {
    if category.id.is_none() {
        create_category(category)?;
    } else {
        let conn =&mut DB_CONNECTION.lock().unwrap();
        let batch = conn.transaction()?;

        batch.execute(
            "UPDATE category SET name = ?, sequence = ? WHERE id = ?",
            params![category.name, category.sequence, category.id],
        )?;

        batch.execute(
            "DELETE FROM account_category WHERE category_id = ?",
            params![category.id],
        )?;

        if let Some(account_ids) = &category.account_ids {
            for account_id in account_ids {
                batch.execute(
                    "INSERT INTO account_category (account_id, category_id, last_update_time)
                        VALUES (?, ?, datetime('now'))",
                    params![account_id, category.id],
                )?;
            }
        }

        batch.commit()?;
    }

    Ok(())
}

pub(crate) fn delete_category_by_id(id: i32) -> Result<()> {
    let conn = &mut DB_CONNECTION.lock().unwrap();
    let batch = conn.transaction()?;
    // delete account_category first
    batch.execute(
        "DELETE FROM account_category WHERE category_id = ?",
        params![id],
    )?;
    // then delete category
    batch.execute("DELETE FROM category WHERE id = ?", params![id])?;

    batch.commit()?;

    Ok(())
}

