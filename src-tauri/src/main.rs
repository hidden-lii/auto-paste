// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sqlite;

mod entity {
    pub mod account;
    pub mod group;
    pub mod account_group;
}

use crate::entity::account::Account;
// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn query_all() -> Vec<Account> {
    let result = sqlite::query_all_accounts();
    match result {
        Ok(accounts) => {
            accounts
        }
        Err(e) => {
            println!("{}", format!("query_all_accounts error: {:?}", e));
            vec![]
        }
    }
}

#[tauri::command]
fn query_by_value(account: Account, with_liked: bool) -> Vec<Account> {
    let result = sqlite::query_accounts_by_value(&account, with_liked);
    match result {
        Ok(accounts) => {
            accounts
        }
        Err(e) => {
            println!("{}", format!("query_accounts_by_value error: {:?}", e));
            vec![]
        }
    }
}

#[tauri::command]
fn insert(account: Account) -> bool {
    if let Err(e) = sqlite::insert_account(&account) {
        println!("{}", format!("insert_account error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn update(account: Account) -> bool {
    if let Err(e) = sqlite::update_account(&account) {
        println!("{}", format!("update_account error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn update_like(id: i32, liked: bool) -> bool {
    if let Err(e) = sqlite::like_account(id, liked) {
        println!("like_account error: {:?}", e);
        return false;
    }

    true
}

#[tauri::command]
fn delete(id: i32) -> bool {
    if let Err(e) = sqlite::delete_by_id(id) {
        println!("{}", format!("delete error: {:?}", e));
        return false;
    }

    true
}

// fn main() {
//     match insert(Account {
//         id: None,
//         name: "123".to_string(),
//         username: "321".to_string(),
//         password: "123".to_string(),
//         sequence: Option::from(1),
//         liked: Option::from(false),
//         description: None,
//         last_update_time: None,
//     }) {
//         true => { println!("insert success") }
//         false => { println!("insert failed") }
//     }
// }

fn main() {
    // 先创建db文件
    sqlite::create_if_not_exists().expect("create_if_not_exists error");
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let close = CustomMenuItem::new("close".to_string(), "Close");
    // let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    // let menu = Menu::new()
    //     .add_native_item(MenuItem::Copy)
    //     .add_item(CustomMenuItem::new("hide", "Hide"))
    //     .add_submenu(submenu);

    tauri::Builder::default()
        // .menu(menu)
        // .on_menu_event(|event| {
        //     match event.menu_item_id() {
        //         "quit" => {
        //             std::process::exit(0);
        //         }
        //         "close" => {
        //             event.window().close().unwrap()
        //         }
        //         _ => {}
        //     }
        // })
        .invoke_handler(tauri::generate_handler![query_all,delete,query_by_value,update_like,insert,update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
