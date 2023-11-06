use crate::config::DISPLAY_SIZE;
use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

use serde::ser::{SerializeMap, SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::json;

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    生産高グラフ　通常グラフ、積算グラフデータ作成
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

//  JSON用構造体
#[derive(Serialize, Deserialize, Clone, Debug)]
struct MyData1 {
    group: String,
    date: String,
    value: i32,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct MyData2 {
    group: String,
    date: String,
    value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TargetStruct {
    product_name: &'static str,
    monthly_target: i32,
    daily_target: i32,
}

pub async fn convert_data(
    fetch_label: Vec<String>,
    fetch_data: Vec<String>,
    fetch_target: Vec<String>,
) -> Vec<String> {
    let mut graph_data: Vec<serde_json::Value> = Vec::new();
    let mut total_graph_data: Vec<serde_json::Value> = Vec::new();
    let mut total_value: i32 = 0;

    let data_length = fetch_label.len();
    for (index, label) in fetch_label.iter().enumerate() {
        //  * 通常グラフデータ作成
        //棒グラフ作成
        let value = fetch_data[index].parse::<i32>().unwrap();
        let daily_target = fetch_target[1].parse::<i32>().unwrap();
        let monthly_target = fetch_target[0].parse::<i32>().unwrap();
        let date = format!("{}\n{}", label, value);

        let data = MyData1 {
            group: "product".to_string(),
            date: date.clone(),
            value: value,
        };
        let json_obj = serde_json::to_value(data).unwrap();
        graph_data.push(json_obj);

        //線グラフ作成　目標値

        let data2 = MyData2 {
            group: "targetValue".to_string(),
            date: date.clone(),
            value: daily_target,
        };
        let json_obj = serde_json::to_value(data2).unwrap();
        graph_data.push(json_obj);

        //  todo
        //  todo  積算グラフデータ作成
        //  todo

        total_value += value;
        let date = format!("{}\n{}", label, total_value);
        //棒グラフ作成
        let data = MyData1 {
            group: "product".to_string(),
            date: date.clone(),
            value: total_value,
        };
        let json_obj = serde_json::to_value(data).unwrap();
        total_graph_data.push(json_obj);

        //線グラフ作成　目標値

        let data2 = MyData2 {
            group: "targetValue".to_string(),
            date: date.clone(),
            value: monthly_target,
        };
        let json_obj = serde_json::to_value(data2).unwrap();
        total_graph_data.push(json_obj);
    }
    // for (index

    vec![
        serde_json::to_string(&graph_data).unwrap(),
        serde_json::to_string(&total_graph_data).unwrap(),
    ]
}

//  todo    //-------------------------------------------------------------------
//  *   コンボのオプション作成
//  todo    //-------------------------------------------------------------------

pub async fn convert_option(config: &FetchSpreadSheetConfig, fetch_target: Vec<String>) -> String {
    let mut target_num: Vec<String> = Vec::new();

    for index in fetch_target.iter() {
        let origin_number = index.parse::<i32>().unwrap();
        target_num.push(format!("{:0>3}", origin_number));
    }

    // let title = format!(
    //     "{} 生産高　　月次目標：{}　　日次目標：{}",
    //     config.output_sheet_name, target_num[0], target_num[1]
    // );
    let title = "".to_string();
    let option_data_head = r#"{
    "title": ""#
        .to_string();

    let option_data_mid = r#"",
    "axes": {
        "left": {
          "mapsTo": "value",
          "scaleType": "linear"
        },
        "right": {
          "correspondingDatasets" : [
            "targetValue"
          ]
        },
        "bottom": {
          "mapsTo": "date",
          "scaleType": "labels"
        }
    },
    "comboChartTypes": [
    {
      "type": "simple-bar",
      "correspondingDatasets": [
        "product"
      ]
    },
    {
      "type": "line",
      "options": {
        "points": {
          "radius":  2 
        }
      },
      "correspondingDatasets": [
        "targetValue"
      ]
    }
  ],
    "bars": {
        "spacingFactor": 0.15,
        "maxWidth": 300
    },
    "height": ""#;

    let option_data_last = r#"",
        "theme": "g90"
    }
    "#;

    let draw_height = match DISPLAY_SIZE {
        "1600" => "300px",
        "2K" => "600px",
        "4K" => "700px",
        &_ => "300px",
    };
    option_data_head + &title + option_data_mid + draw_height + option_data_last
}

pub async fn convert_target(config: &FetchSpreadSheetConfig, fetch_target: Vec<String>) -> String {
    let data = TargetStruct {
        product_name: config.output_sheet_name,
        monthly_target: fetch_target[0].parse::<i32>().unwrap(),
        daily_target: fetch_target[1].parse::<i32>().unwrap(),
    };

    serde_json::to_string(&data).unwrap()
}
