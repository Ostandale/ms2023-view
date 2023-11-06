use crate::struct_data::FetchSpreadSheetConfig;
use google_sheets4::api::ValueRange;
use google_sheets4::Sheets;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tauri::State;

// #[tauri::command]
// pub async fn fetch_spreadsheet_data(file_name: &str) -> Result<(String, String), String> {
//     let (root_path, data_path) = if IS_RELEASE {
//         (RELEASE_ROOT_PATH, RELEASE_DATA_PATH)
//     } else {
//         (DEV_ROOT_PATH, DEV_DATA_PATH)
//     };

//     let data_file_path = format!("{}data_{}", data_path, file_name);
//     let options_file_path = format!("{}options_{}", data_path, file_name);

//     let result = tokio::fs::read_to_string(data_file_path).await;
//     let data_file = match result {
//         Ok(v) => serde_json::to_string(&v).unwrap(),
//         Err(e) => return Err("data_file_err".to_string()),
//     };
//     let result = tokio::fs::read_to_string(options_file_path).await;
//     let options_file = match result {
//         Ok(v) => serde_json::to_string(&v).unwrap(),
//         Err(e) => return Err("option_file_err".to_string()),
//     };

//     Ok((data_file, options_file))
// }

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn fetch_spreadsheet_data2(
    sheet: State<'_, Sheets<HttpsConnector<HttpConnector>>>,
    configs: State<'_, Vec<FetchSpreadSheetConfig>>,
    config_num: usize,
    data_type: &str,
) -> Result<(String, String, String), String> {
    //  引数の型からStateを取り外す
    let configs_inner = configs.inner();
    let config = configs_inner[config_num].clone();
    let sheet = sheet.inner();

    let range = if data_type == "nomal" || data_type.is_empty() {
        format!("{}!{}", config.output_sheet_name, config.output_sheet_range)
    } else if data_type == "total" {
        format!("{}!A2:C2", config.output_sheet_name)
    } else {
        format!("{}!A1:C1", config.output_sheet_name)
    };

    let result = read_spreadsheet(sheet, config.sheet_id, &range).await;
    let (_, fetch_data) = match result {
        Ok(v) => v,
        Err(e) => return Err("data_file_err".to_string()),
    };

    for row in fetch_data.values {
        if let Some(data) = row.into_iter().next() {
            let value1 = data[0].to_string();
            let value2 = data[1].to_string();
            let value3 = data[2].to_string();
            return Ok((value1, value2, value3));
        }
    }

    Ok(("".to_string(), "".to_string(), "".to_string()))
}

pub async fn read_spreadsheet(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    range: &str,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), google_sheets4::Error> {
    hub.spreadsheets()
        .values_get(spreadsheet_id, range)
        .doit()
        .await
}
