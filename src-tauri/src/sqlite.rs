use crate::entity::account::Account;
use crate::entity::category::Category;
use crate::entity::jx3_server::Jx3Server;
use crate::entity::role::Role;
use crate::jx3_sync::{fallback_servers, fetch_servers_from_api};
use rusqlite::{params, Connection, Result, ToSql};
use std::cmp::Ordering;
use std::sync::Mutex;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_VERSION_KEY: &str = "app_version";

lazy_static::lazy_static! {
    static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(Connection::open("auto_paste.db").unwrap());
}

fn parse_version(version: &str) -> (u32, u32, u32) {
    let mut parts = version.split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    parse_version(left).cmp(&parse_version(right))
}

fn version_lt(left: &str, right: &str) -> bool {
    compare_versions(left, right) == Ordering::Less
}

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({table})"))?;
    let rows = stmt.query_map([], |row| {
        let name: String = row.get(1)?;
        Ok(name)
    })?;

    for row in rows {
        if row? == column {
            return Ok(true);
        }
    }

    Ok(false)
}

fn get_setting_with_conn(conn: &Connection, key: &str) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM app_setting WHERE key = ?1")?;
    let mut rows = stmt.query(params![key])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

fn set_setting_with_conn(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO app_setting (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

fn rebuild_account_table_without_priority(conn: &Connection, has_sequence: bool) -> Result<()> {
    let sequence_expr = if has_sequence {
        "CASE
            WHEN sequence IS NOT NULL AND sequence > 0 THEN sequence
            WHEN priority IS NOT NULL AND priority > 0 THEN priority
            ELSE id
        END"
    } else {
        "CASE
            WHEN priority IS NOT NULL AND priority > 0 THEN priority
            ELSE id
        END"
    };

    conn.execute_batch(&format!(
        "CREATE TABLE account_new (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            sequence INTEGER NOT NULL DEFAULT 1,
            liked INTEGER NOT NULL DEFAULT 0,
            description TEXT,
            last_update_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime'))
        );
        INSERT INTO account_new (id, name, username, password, sequence, liked, description, last_update_time)
        SELECT
            id,
            name,
            username,
            password,
            {sequence_expr},
            liked,
            description,
            last_update_time
        FROM account;
        DROP TABLE account;
        ALTER TABLE account_new RENAME TO account;"
    ))?;

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

    Ok(())
}

fn rebuild_category_table_without_priority(conn: &Connection, has_sequence: bool) -> Result<()> {
    let sequence_expr = if has_sequence {
        "CASE
            WHEN sequence IS NOT NULL AND sequence > 0 THEN sequence
            WHEN priority IS NOT NULL AND priority > 0 THEN priority
            ELSE id
        END"
    } else {
        "CASE
            WHEN priority IS NOT NULL AND priority > 0 THEN priority
            ELSE id
        END"
    };

    conn.execute_batch(&format!(
        "CREATE TABLE category_new (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            sequence INTEGER NOT NULL DEFAULT 1,
            last_update_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime'))
        );
        INSERT INTO category_new (id, name, sequence, last_update_time)
        SELECT
            id,
            name,
            {sequence_expr},
            last_update_time
        FROM category;
        DROP TABLE category;
        ALTER TABLE category_new RENAME TO category;"
    ))?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_group_last_update_time
            AFTER UPDATE ON category
            FOR EACH ROW
            BEGIN
            UPDATE category SET last_update_time = (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime')) WHERE id = OLD.id;
            END",
        [],
    )?;

    Ok(())
}

fn migrate_priority_to_sequence(conn: &Connection, table: &str) -> Result<()> {
    let has_priority = column_exists(conn, table, "priority")?;
    let has_sequence = column_exists(conn, table, "sequence")?;

    if has_priority {
        if table == "account" {
            rebuild_account_table_without_priority(conn, has_sequence)?;
        } else if table == "category" {
            rebuild_category_table_without_priority(conn, has_sequence)?;
        }
        return Ok(());
    }

    if !has_sequence {
        conn.execute(
            &format!("ALTER TABLE {table} ADD COLUMN sequence INTEGER NOT NULL DEFAULT 1"),
            [],
        )?;
        conn.execute(
            &format!("UPDATE {table} SET sequence = id WHERE sequence IS NULL OR sequence <= 0"),
            [],
        )?;
    }

    Ok(())
}

fn migrate_to_1_1_0(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_setting (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
        [],
    )?;

    ensure_setting_default(conn, "hide_username", "0")?;
    ensure_setting_default(conn, "hide_password", "1")?;
    Ok(())
}

fn migrate_to_1_1_2(conn: &Connection) -> Result<()> {
    migrate_priority_to_sequence(conn, "account")?;
    migrate_priority_to_sequence(conn, "category")?;
    Ok(())
}

fn migrate_to_1_1_4(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS role (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            role_id TEXT NOT NULL,
            server TEXT NOT NULL,
            last_update_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime'))
        );",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_role_account_id ON role(account_id)",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS jx3_server (
            id INTEGER PRIMARY KEY,
            zone TEXT NOT NULL,
            server TEXT NOT NULL UNIQUE,
            status TEXT NOT NULL,
            last_update_time TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime'))
        );",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_jx3_server_zone ON jx3_server(zone)",
        [],
    )?;

    upsert_jx3_servers_with_conn(conn, &fallback_servers())?;

    ensure_setting_default(conn, "export_fields", r#"["name","username","password","roles","description"]"#)?;
    ensure_setting_default(conn, "network_sync_enabled", "0")?;
    ensure_setting_default(conn, "network_sync_prompted", "0")?;
    ensure_setting_default(conn, "favorite_filter", "0")?;

    Ok(())
}

fn ensure_setting_default(conn: &Connection, key: &str, default_value: &str) -> Result<()> {
    if get_setting_with_conn(conn, key)?.is_none() {
        set_setting_with_conn(conn, key, default_value)?;
    }
    Ok(())
}

fn ensure_triggers(conn: &Connection) -> Result<()> {
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

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_group_last_update_time
            AFTER UPDATE ON category
            FOR EACH ROW
            BEGIN
            UPDATE category SET last_update_time = (strftime('%Y-%m-%d %H:%M:%f', 'now', 'localtime')) WHERE id = OLD.id;
            END",
        [],
    )?;

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

/// 合并修复：幂等操作，确保数据库结构与当前版本定义一致。
fn merge_database_schema(conn: &Connection) -> Result<()> {
    migrate_priority_to_sequence(conn, "account")?;
    migrate_priority_to_sequence(conn, "category")?;

    conn.execute(
        "UPDATE account SET sequence = id WHERE sequence IS NULL OR sequence <= 0",
        [],
    )?;
    conn.execute(
        "UPDATE category SET sequence = id WHERE sequence IS NULL OR sequence <= 0",
        [],
    )?;

    migrate_to_1_1_0(conn)?;
    migrate_to_1_1_4(conn)?;
    ensure_triggers(conn)?;

    Ok(())
}

type MigrationFn = fn(&Connection) -> Result<()>;

struct Migration {
    version: &'static str,
    description: &'static str,
    migrate: MigrationFn,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: "1.1.0",
        description: "添加应用设置表及默认显示配置",
        migrate: migrate_to_1_1_0,
    },
    Migration {
        version: "1.1.2",
        description: "优先级字段由 priority 迁移为 sequence",
        migrate: migrate_to_1_1_2,
    },
    Migration {
        version: "1.1.4",
        description: "添加角色区服表与 JX3 区服字典",
        migrate: migrate_to_1_1_4,
    },
];

fn get_stored_db_version(conn: &Connection) -> Result<String> {
    Ok(get_setting_with_conn(conn, APP_VERSION_KEY)?
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "0.0.0".to_string()))
}

fn run_migrations(conn: &Connection) -> Result<()> {
    let stored_version = get_stored_db_version(conn)?;
    let current_version = APP_VERSION;

    if stored_version == current_version {
        return Ok(());
    }

    println!(
        "数据库版本({stored_version})与当前版本({current_version})不一致，开始执行合并..."
    );

    if version_lt(current_version, &stored_version) {
        println!(
            "警告: 数据库版本({stored_version})高于应用版本({current_version})，跳过结构迁移"
        );
        return Ok(());
    }

    for migration in MIGRATIONS {
        if version_lt(&stored_version, migration.version)
            && !version_lt(current_version, migration.version)
        {
            println!(
                "正在迁移至 {}: {}",
                migration.version, migration.description
            );
            (migration.migrate)(conn)?;
            set_setting_with_conn(conn, APP_VERSION_KEY, migration.version)?;
        }
    }

    merge_database_schema(conn)?;
    set_setting_with_conn(conn, APP_VERSION_KEY, current_version)?;
    println!("数据库合并完成，当前版本: {current_version}");

    Ok(())
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
    ensure_triggers(&conn)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_setting (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
        [],
    )?;

    run_migrations(&conn)?;

    Ok(())
}

/// 应用启动时调用：创建表结构并检查/合并数据库版本。
pub(crate) fn initialize_database() -> Result<()> {
    create_if_not_exists()
}

pub(crate) fn insert_account(account: &Account) -> Result<()> {
    let conn = &mut DB_CONNECTION.lock().unwrap();
    let batch = conn.transaction()?;

    let default_description = "这个人好懒,没有给他写备注信息呢┓(´∀`)┏".to_string();
    batch.execute(
        "INSERT INTO account (name, username, password, sequence, liked, description, last_update_time)
        VALUES (?, ?, ?, IFNULL(?, 1), ?, ?, datetime('now'))",
        params![
            account.name,
            account.username,
            account.password,
            account.sequence,
            account.liked.unwrap_or(false),
            if account.description.is_none() || account.description.clone().unwrap().is_empty() { default_description } else { account.description.clone().unwrap() },
        ],
    )?;

    let account_id = batch.last_insert_rowid() as i32;

    if let Some(account_category_ids) = &account.account_category_ids {
        for category_id in account_category_ids {
            batch.execute(
                "INSERT INTO account_category (account_id, category_id, last_update_time)
                    VALUES (?, ?, datetime('now'))",
                params![account_id, category_id],
            )?;
        }
    }

    insert_roles_in_tx(&batch, account_id, account.roles.as_ref())?;

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

        batch.execute("DELETE FROM role WHERE account_id = ?", params![account.id])?;
        insert_roles_in_tx(&batch, account.id.unwrap(), account.roles.as_ref())?;

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

    batch.execute("DELETE FROM role WHERE account_id = ?", params![id])?;
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
        SELECT
            a.id,
            a.name,
            a.username,
            a.password,
            a.sequence,
            a.liked,
            a.description,
            a.last_update_time,
            ac.account_category_ids
        FROM account a
        LEFT JOIN (
            SELECT account_id, GROUP_CONCAT(category_id) AS account_category_ids
            FROM account_category
            GROUP BY account_id
        ) AS ac ON a.id = ac.account_id
        ORDER BY a.sequence ASC, a.id ASC"
    )?;

    attach_roles_to_accounts(&conn, _do_query_accounts(&mut stmt, &[])?)
}

pub(crate) fn query_accounts_by_value(
    account: &Account,
    with_liked: bool,
    category_id: i32,
) -> Result<Vec<Account>> {
    let mut query = "
        SELECT
            a.id,
            a.name,
            a.username,
            a.password,
            a.sequence,
            a.liked,
            a.description,
            a.last_update_time,
            ac.account_category_ids
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

    query += " ORDER BY a.sequence ASC, a.id ASC";

    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare(&query)?;

    attach_roles_to_accounts(&conn, _do_query_accounts(&mut stmt, &params)?)
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
            roles: Some(Vec::new()),
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
        SELECT
            c.id,
            c.name,
            c.sequence,
            c.last_update_time,
            ac.account_category_ids
        FROM category c
        LEFT JOIN (
            SELECT category_id, GROUP_CONCAT(account_id) AS account_category_ids
            FROM account_category
            GROUP BY category_id
        ) AS ac ON c.id = ac.category_id
        WHERE 1 = 1
        ORDER BY c.sequence ASC, c.id ASC
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

pub(crate) fn reorder_accounts(ids: &[i32]) -> Result<()> {
    let conn = &mut DB_CONNECTION.lock().unwrap();
    let batch = conn.transaction()?;

    for (index, id) in ids.iter().enumerate() {
        batch.execute(
            "UPDATE account SET sequence = ? WHERE id = ?",
            params![(index + 1) as i32, id],
        )?;
    }

    batch.commit()?;

    Ok(())
}

pub(crate) fn reorder_categories(ids: &[i32]) -> Result<()> {
    let conn = &mut DB_CONNECTION.lock().unwrap();
    let batch = conn.transaction()?;

    for (index, id) in ids.iter().enumerate() {
        batch.execute(
            "UPDATE category SET sequence = ? WHERE id = ?",
            params![(index + 1) as i32, id],
        )?;
    }

    batch.commit()?;

    Ok(())
}

pub(crate) fn get_setting(key: &str) -> Result<Option<String>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT value FROM app_setting WHERE key = ?1")?;
    let mut rows = stmt.query(params![key])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub(crate) fn set_setting(key: &str, value: &str) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute(
        "INSERT INTO app_setting (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

pub(crate) fn get_window_size() -> Result<Option<(u32, u32)>> {
    let width = get_setting("window_width")?.and_then(|value| value.parse().ok());
    let height = get_setting("window_height")?.and_then(|value| value.parse().ok());

    match (width, height) {
        (Some(width), Some(height)) if width > 0 && height > 0 => Ok(Some((width, height))),
        _ => Ok(None),
    }
}

pub(crate) fn save_window_size(width: u32, height: u32) -> Result<()> {
    set_setting("window_width", &width.to_string())?;
    set_setting("window_height", &height.to_string())?;
    Ok(())
}

pub(crate) fn get_display_settings() -> Result<(bool, bool)> {
    let hide_username = get_setting("hide_username")?
        .map(|value| value == "1")
        .unwrap_or(false);
    let hide_password = get_setting("hide_password")?
        .map(|value| value == "1")
        .unwrap_or(true);

    Ok((hide_username, hide_password))
}

pub(crate) fn save_display_settings(hide_username: bool, hide_password: bool) -> Result<()> {
    set_setting("hide_username", if hide_username { "1" } else { "0" })?;
    set_setting("hide_password", if hide_password { "1" } else { "0" })?;
    Ok(())
}

pub(crate) fn get_app_version() -> Result<String> {
    get_stored_db_version(&DB_CONNECTION.lock().unwrap())
}

pub(crate) fn get_current_app_version() -> &'static str {
    APP_VERSION
}

fn insert_roles_in_tx(
    batch: &rusqlite::Transaction,
    account_id: i32,
    roles: Option<&Vec<Role>>,
) -> Result<()> {
    if let Some(roles) = roles {
        for role in roles {
            if role.role_id.trim().is_empty() || role.server.trim().is_empty() {
                continue;
            }
            batch.execute(
                "INSERT INTO role (account_id, role_id, server, last_update_time)
                 VALUES (?, ?, ?, datetime('now'))",
                params![account_id, role.role_id.trim(), role.server.trim()],
            )?;
        }
    }
    Ok(())
}

fn load_all_roles_map(conn: &Connection) -> Result<std::collections::HashMap<i32, Vec<Role>>> {
    let mut stmt = conn.prepare(
        "SELECT id, account_id, role_id, server, last_update_time FROM role ORDER BY id ASC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Role {
            id: row.get(0)?,
            account_id: row.get(1)?,
            role_id: row.get(2)?,
            server: row.get(3)?,
            last_update_time: row.get(4)?,
        })
    })?;

    let mut map: std::collections::HashMap<i32, Vec<Role>> = std::collections::HashMap::new();
    for row in rows {
        let role = row?;
        if let Some(account_id) = role.account_id {
            map.entry(account_id).or_default().push(role);
        }
    }
    Ok(map)
}

fn attach_roles_to_accounts(conn: &Connection, mut accounts: Vec<Account>) -> Result<Vec<Account>> {
    if accounts.is_empty() {
        return Ok(accounts);
    }
    let roles_map = load_all_roles_map(conn)?;
    for account in &mut accounts {
        if let Some(id) = account.id {
            account.roles = Some(roles_map.get(&id).cloned().unwrap_or_default());
        } else {
            account.roles = Some(Vec::new());
        }
    }
    Ok(accounts)
}

pub(crate) fn upsert_jx3_servers_with_conn(
    conn: &Connection,
    servers: &[Jx3Server],
) -> Result<()> {
    for server in servers {
        conn.execute(
            "INSERT INTO jx3_server (zone, server, status, last_update_time)
             VALUES (?1, ?2, ?3, datetime('now'))
             ON CONFLICT(server) DO UPDATE SET
                zone = excluded.zone,
                status = excluded.status,
                last_update_time = datetime('now')",
            params![server.zone, server.server, server.status],
        )?;
    }
    Ok(())
}

pub(crate) fn query_all_jx3_servers() -> Result<Vec<Jx3Server>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, zone, server, status, last_update_time FROM jx3_server ORDER BY zone ASC, server ASC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Jx3Server {
            id: row.get(0)?,
            zone: row.get(1)?,
            server: row.get(2)?,
            status: row.get(3)?,
            last_update_time: row.get(4)?,
        })
    })?;

    let mut servers = Vec::new();
    for row in rows {
        servers.push(row?);
    }
    Ok(servers)
}

pub(crate) fn sync_jx3_servers(force_fallback: bool) -> Result<bool> {
    let network_enabled = get_setting("network_sync_enabled")?
        .map(|v| v == "1")
        .unwrap_or(false);

    let servers = if !force_fallback && network_enabled {
        match fetch_servers_from_api() {
            Ok(list) => list,
            Err(err) => {
                println!("JX3API 同步失败，使用兜底数据: {err}");
                fallback_servers()
            }
        }
    } else {
        fallback_servers()
    };

    let conn = DB_CONNECTION.lock().unwrap();
    upsert_jx3_servers_with_conn(&conn, &servers)?;
    set_setting_with_conn(&conn, "jx3_server_last_sync", &chrono_lite_now())?;
    Ok(network_enabled && !force_fallback)
}

fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    secs.to_string()
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct NetworkSyncSettings {
    pub(crate) enabled: bool,
    pub(crate) prompted: bool,
    pub(crate) last_sync: Option<String>,
}

pub(crate) fn get_network_sync_settings() -> Result<NetworkSyncSettings> {
    Ok(NetworkSyncSettings {
        enabled: get_setting("network_sync_enabled")?
            .map(|v| v == "1")
            .unwrap_or(false),
        prompted: get_setting("network_sync_prompted")?
            .map(|v| v == "1")
            .unwrap_or(false),
        last_sync: get_setting("jx3_server_last_sync")?,
    })
}

pub(crate) fn save_network_sync_settings(enabled: bool, prompted: bool) -> Result<()> {
    set_setting("network_sync_enabled", if enabled { "1" } else { "0" })?;
    set_setting("network_sync_prompted", if prompted { "1" } else { "0" })?;
    Ok(())
}

pub(crate) fn get_export_fields() -> Result<Vec<String>> {
    let default = vec![
        "name".to_string(),
        "username".to_string(),
        "password".to_string(),
        "roles".to_string(),
        "description".to_string(),
    ];
    match get_setting("export_fields")? {
        Some(value) => {
            let parsed: Vec<String> = serde_json::from_str(&value).unwrap_or(default.clone());
            if parsed.is_empty() {
                Ok(default)
            } else {
                Ok(parsed)
            }
        }
        None => Ok(default),
    }
}

pub(crate) fn save_export_fields(fields: &[String]) -> Result<()> {
    let value = serde_json::to_string(fields).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    set_setting("export_fields", &value)
}

pub(crate) fn get_favorite_filter() -> Result<i32> {
    Ok(get_setting("favorite_filter")?
        .and_then(|v| v.parse().ok())
        .unwrap_or(0))
}

pub(crate) fn save_favorite_filter(value: i32) -> Result<()> {
    set_setting("favorite_filter", &value.to_string())
}

