use crate::struct_data::FetchSpreadSheetConfig;

use google_sheets4::api::{UpdateValuesResponse, ValueRange};
use google_sheets4::Sheets;
//  todo   //-------------------------------------------------------------------
//  スプレッドシートから読み込む
//  todo   //-------------------------------------------------------------------
pub async fn read_file(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    range: &str,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), google_sheets4::Error> {
    hub.spreadsheets()
        .values_get(spreadsheet_id, range)
        .doit()
        .await
}

//  todo    //-------------------------------------------------------------------
//  スプレッドシートへ値を書き込む
//  todo    //-------------------------------------------------------------------
pub async fn update_value(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
    graph_data: &str,
    options_data: &str,
    target_data: &str,
    index: usize,
) -> Result<(hyper::Response<hyper::Body>, UpdateValuesResponse), google_sheets4::Error> {
    let range = if index == 0 {
        format!("{}!A1:C1", config.output_sheet_name)
    } else {
        format!("{}!A2:C2", config.output_sheet_name)
    };

    let req = ValueRange {
        major_dimension: Some("ROWS".to_string()),
        range: Some(range.clone()),
        values: Some(vec![vec![
            serde_json::value::Value::String(graph_data.to_string()),
            serde_json::value::Value::String(options_data.to_string()),
            serde_json::value::Value::String(target_data.to_string()),
        ]]),
    };

    let res = hub
        .spreadsheets()
        .values_update(req, config.sheet_id, &range)
        .value_input_option("RAW")
        .doit()
        .await;

    match res {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("エラー{}", e);
            Err(e)
        }
    }
}
