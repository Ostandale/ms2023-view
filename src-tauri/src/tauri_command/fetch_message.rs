use crate::config::SPREADSHEET_ID;
use crate::tauri_command::fetch_google_spreadsheet::read_spreadsheet;
use google_sheets4::Sheets;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    date: String,
    interval: String,
    option: String,
    source: String,
    update_date: String,
    message: String,
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn fetch_message(
    sheet: State<'_, Sheets<HttpsConnector<HttpConnector>>>,
) -> Result<Vec<String>, String> {
    let sheet = sheet.inner();

    let result = read_spreadsheet(sheet, SPREADSHEET_ID, "news!A2:F10").await;
    let (_, fetch_data) = match result {
        Ok(v) => v,
        Err(e) => return Err("data_file_err".to_string()),
    };

    if let Some(value) = fetch_data.values {
        let mut message_vec = Vec::new();
        for vec_element in value {
            let data = Message {
                date: vec_element[0].to_string(),
                interval: vec_element[1].to_string(),
                option: vec_element[2].to_string(),
                source: vec_element[3].to_string(),
                update_date: vec_element[4].to_string(),
                message: vec_element[5].to_string(),
            };
            message_vec.push(serde_json::to_string(&data).unwrap());
        }
        Ok(message_vec)
    } else {
        eprintln!("error");
        Err("エラー発生".to_string())
    }
}
