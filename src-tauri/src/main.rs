// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod convert_to_chart_data;
mod credential;
mod fetch_convert_output;
mod struct_data;
mod tauri_command;

use config::{DEV_DATA_PATH, DEV_ROOT_PATH, IS_RELEASE, RELEASE_DATA_PATH, RELEASE_ROOT_PATH};
use convert_to_chart_data::convert_to_chart_data;
use struct_data::FetchSpreadSheetConfig;
use tauri_command::fetch_google_spreadsheet::fetch_spreadsheet_data2;
use tauri_command::fetch_message::fetch_message;
use tauri_command::fetch_struct_data::fetch_struct_data;

use credential::auth::auth;
use dotenvy::dotenv;
use std::env;

use google_sheets4::Sheets;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tokio::main]
async fn main() {
    dotenv().ok();

    //  root_path, data_pathを開発ビルド用とリリース用で分ける
    let (root_path, data_path) = if IS_RELEASE {
        (RELEASE_ROOT_PATH, RELEASE_DATA_PATH)
    } else {
        (DEV_ROOT_PATH, DEV_DATA_PATH)
    };

    //   *   GCP認証手続き
    let (auth, client) = match auth(root_path).await {
        Ok((v1, v2)) => (v1, v2),
        Err(e) => {
            eprint!("認証失敗エラー発生{}", e);
            panic!();
        }
    };
    let sheet = Sheets::new(client, auth);
    let state_sheet = sheet.clone();
    let sheet_config = FetchSpreadSheetConfig::new();

    let _thread = tokio::spawn(async move {
        convert_to_chart_data(sheet.clone()).await;
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_spreadsheet_data2,
            fetch_struct_data,
            fetch_message,
        ])
        .manage(sheet_config)
        .manage(state_sheet)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
