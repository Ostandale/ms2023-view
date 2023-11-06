use crate::config::DISPLAY_SIZE;
use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    縦棒グラフ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

//  JSON用構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct JsonData {
    group: String,
    value: i32,
}

pub async fn make_data_bar(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: Vec<JsonData> = Vec::new();

    for (label, data) in fetch_label.iter().zip(fetch_data.iter()) {
        let check_number = data.replace(',', "").parse::<i32>();
        match check_number {
            Ok(number) => {
                let data = JsonData {
                    group: label.to_string(),
                    value: number,
                };
                graph_data.push(data);
            }
            Err(e) => {
                eprintln!("パース失敗 : {}", e);
                eprintln!("データ{}", data);
            }
        }
    }
    let json_data = serde_json::to_string(&graph_data).unwrap();
    json_data
}

//  todo    //-------------------------------------------------------------------
//  *   縦棒グラフ
//  todo    //-------------------------------------------------------------------

pub async fn make_option_bar(config: &FetchSpreadSheetConfig, update_epoch_time: u64) -> String {
    let japan_time = Utc
        .timestamp_opt(update_epoch_time as i64, 0)
        .unwrap()
        .with_timezone(&chrono::offset::FixedOffset::east_opt(9 * 3600).unwrap());
    let format_time = japan_time
        .format(" 最終更新時間 %Y-%m-%d %H:%M:%S")
        .to_string();

    let option_data_head = r#"{
        "title": ""#
        .to_string();

    let option_data_mid = r#"",
        "axes": {
            "left": {
                "mapsTo": "value"
            },
            "bottom": {
                "scaleType": "labels",
                "mapsTo": "group",
                "title": ""#;

    let option_data_foot = r#""
            }
        },
        "bars": {
            "spacingFactor": 0.6,
            "maxWidth": 100
        },
        "height": ""#;
    let option_data_last = r#"",
        "theme": "g90"
    }
    "#;
    let draw_height = match DISPLAY_SIZE {
        "1600" => "300px",
        "2K" => "500px",
        "4K" => "700px",
        &_ => "300px",
    };
    let option_str = option_data_head
        + config.fetch_sheet_name
        + option_data_mid
        + &format_time
        + option_data_foot
        + draw_height
        + option_data_last;
    serde_json::to_string(&option_str).unwrap()
}
