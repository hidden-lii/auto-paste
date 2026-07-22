// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sqlite;

mod entity {
    pub mod account;
    pub mod account_category;
    pub mod category;
}

use crate::entity::account::Account;
use crate::entity::category::Category;
use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager, Size};
// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

#[derive(Debug, Serialize, Deserialize)]
struct WindowSize {
    width: u32,
    height: u32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn query_all_accounts() -> Vec<Account> {
    let result = sqlite::query_all_accounts();
    match result {
        Ok(accounts) => accounts,
        Err(e) => {
            println!("{}", format!("query_all_accounts error: {:?}", e));
            vec![]
        }
    }
}

#[tauri::command]
fn query_accounts_by_value(account: Account, with_liked: bool, category_id: i32) -> Vec<Account> {
    let result = sqlite::query_accounts_by_value(&account, with_liked, category_id);
    match result {
        Ok(accounts) => accounts,
        Err(e) => {
            println!("{}", format!("query_accounts_by_value error: {:?}", e));
            vec![]
        }
    }
}

#[tauri::command]
fn insert_account(account: Account) -> bool {
    if let Err(e) = sqlite::insert_account(&account) {
        println!("{}", format!("insert_account error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn update_account(account: Account) -> bool {
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
fn delete_account(id: i32) -> bool {
    if let Err(e) = sqlite::delete_by_id(id) {
        println!("{}", format!("delete error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn create_category(category: Category) -> bool {
    if let Err(e) = sqlite::create_category(&category) {
        println!("{}", format!("insert_category error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn query_all_category() -> Vec<Category> {
    let result = sqlite::query_all_categories();
    match result {
        Ok(categories) => categories,
        Err(e) => {
            println!("{}", format!("query_all_category error: {:?}", e));
            vec![]
        }
    }
}

#[tauri::command]
fn update_category(category: Category) -> bool {
    if let Err(e) = sqlite::update_category(&category) {
        println!("{}", format!("update_category error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn delete_category_by_id(id: i32) -> bool {
    if let Err(e) = sqlite::delete_category_by_id(id) {
        println!("{}", format!("delete_category_by_id error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn reorder_accounts(ids: Vec<i32>) -> bool {
    if let Err(e) = sqlite::reorder_accounts(&ids) {
        println!("{}", format!("reorder_accounts error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn reorder_categories(ids: Vec<i32>) -> bool {
    if let Err(e) = sqlite::reorder_categories(&ids) {
        println!("{}", format!("reorder_categories error: {:?}", e));
        return false;
    }

    true
}

#[tauri::command]
fn get_saved_window_size() -> Option<WindowSize> {
    match sqlite::get_window_size() {
        Ok(Some((width, height))) => Some(WindowSize { width, height }),
        Ok(None) => None,
        Err(e) => {
            println!("get_saved_window_size error: {:?}", e);
            None
        }
    }
}

#[tauri::command]
fn save_window_size(width: u32, height: u32) -> bool {
    if let Err(e) = sqlite::save_window_size(width, height) {
        println!("save_window_size error: {:?}", e);
        return false;
    }

    true
}

#[tauri::command]
fn get_default_window_size() -> WindowSize {
    #[cfg(target_os = "macos")]
    let height = 761u32;
    #[cfg(not(target_os = "macos"))]
    let height = 732u32;

    WindowSize {
        width: 460,
        height,
    }
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
        .setup(|app| {
            let window = app
                .get_window("main")
                .expect("main window not found");

            if let Ok(Some((width, height))) = sqlite::get_window_size() {
                let _ = window.set_size(Size::Logical(LogicalSize::new(
                    width as f64,
                    height as f64,
                )));
            }

            let _ = window.show();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            query_all_accounts,
            delete_account,
            query_accounts_by_value,
            update_like,
            insert_account,
            update_account,
            create_category,
            query_all_category,
            update_category,
            delete_category_by_id,
            reorder_accounts,
            reorder_categories,
            get_saved_window_size,
            save_window_size,
            get_default_window_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
