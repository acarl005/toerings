#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data_harvester;
mod utils;

use std::sync::Mutex;

use crate::utils::error;
use data_harvester::{Data, DataCollector};
use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu};

#[cfg(target_family = "windows")]
pub type Pid = usize;

#[cfg(target_family = "unix")]
pub type Pid = libc::pid_t;

#[tauri::command]
fn collect_data(data_state: tauri::State<Mutex<DataCollector>>) -> Data {
    futures::executor::block_on(data_state.lock().unwrap().update_data());
    data_state.lock().unwrap().data.clone()
}

fn main() {
    let mut data_state = DataCollector::new();
    data_state.init();

    let preferences = CustomMenuItem::new("preferences", "Open Preferences").accelerator("cmd+,");
    let submenu = Submenu::new(
        "Menu",
        Menu::new()
            .add_item(preferences)
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Quit)
            .add_native_item(MenuItem::About(
                "ToeRings".to_string(),
                AboutMetadata::new()
                    .version(env!("CARGO_PKG_VERSION"))
                    .authors(
                        env!("CARGO_PKG_AUTHORS")
                            .split(":")
                            .map(String::from)
                            .collect(),
                    )
                    .license("MIT"),
            )),
    );
    let menu = Menu::new().add_submenu(submenu);

    tauri::Builder::default()
        .manage(Mutex::new(data_state))
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "preferences" => event.window().emit("openPreferences", ()).unwrap(),
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![collect_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
