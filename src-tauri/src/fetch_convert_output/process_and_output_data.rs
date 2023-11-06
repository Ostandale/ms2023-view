use crate::fetch_convert_output::sheet_control::{read_file, update_value};
use crate::struct_data::FetchSpreadSheetConfig;

//  *   生産高シートからデータを取得、変換、出力するための関数
use crate::fetch_convert_output::convert_graph_data::convert_seisan_srs1a_graph::{
    convert_data, convert_option, convert_target,
};
use crate::fetch_convert_output::fetch_graph_data::fetch_seisan_graph::{
    fetch_seisan_graph_data, fetch_seisan_graph_label, fetch_seisan_graph_target,
};

use google_sheets4::Sheets;
use std::io::Error;
use tokio::io::ErrorKind;

//  !   //-------------------------------------------------------------------
//  !   //-------------------------------------------------------------------
//  *   スプレッドシートからデータを取得してグラフデータを作成する
//  !   //-------------------------------------------------------------------
//  !   //-------------------------------------------------------------------
pub async fn process_and_output_data(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
) -> core::result::Result<(), Error> {
    let mut results: Vec<std::result::Result<(), std::io::Error>> = Vec::new();

    let (fetch_label, fetch_data, fetch_target) = match config.data_type {
        "seisan" => (
            fetch_seisan_graph_label(hub, config).await,
            fetch_seisan_graph_data(hub, config).await,
            fetch_seisan_graph_target(hub, config).await,
        ),
        &_ => (
            vec!["".to_string()],
            vec!["".to_string()],
            vec!["".to_string()],
        ),
    };

    match config.data_type {
        "seisan" => {
            //  ラベルと数値のベクタから実際に使用するグラフのデータ形式に変換したファイル内容＝文字列ベクタを取得
            let data_files = convert_data(fetch_label, fetch_data, fetch_target.clone()).await;

            //  グラフ用オプションファイルの作成
            let option_file = convert_option(config, fetch_target.clone()).await;

            let target_file = convert_target(config, fetch_target.clone()).await;

            //  todo    スプレッドシートに書き込み

            for (index, data_file) in data_files.iter().enumerate() {
                let result = write_spreadsheet_data_file(
                    hub,
                    data_file.clone(),
                    option_file.clone(),
                    target_file.clone(),
                    config,
                    index,
                )
                .await;
                results.push(result);
            }
        }
        &_ => {}
    };

    //  ファイルに書き込み
    // let result = write_javascript_data_file(data_file, option_file, config, data_path).await;
    // results.push(result);

    if results.iter().any(|result| result.is_err()) {
        return Err(Error::new(ErrorKind::Other, "書き込みエラーが発生しました"));
    }

    Ok(())
}

//  !   //-------------------------------------------------------------------
//  *   指定したシートの１行目から文字列としてラベルを取得する
//  !   //-------------------------------------------------------------------
pub async fn fetch_graph_label(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    config: &FetchSpreadSheetConfig,
) -> Vec<String> {
    let sheet_range = format!("{}!A1:Z1", config.fetch_sheet_name);
    let result = read_file(hub, spreadsheet_id, &sheet_range).await;

    match result {
        Ok((_, value_range)) => {
            let vec_values: Vec<String> = value_range.values.map_or(Vec::new(), |values| {
                values
                    .iter()
                    .flat_map(|row| {
                        row.iter().map(|value| {
                            let cell_content = value.to_string().replace('\"', "");
                            if cell_content.is_empty() {
                                String::from("")
                            } else {
                                cell_content
                            }
                        })
                    })
                    .collect()
            });
            vec_values
        }
        Err(err) => {
            eprintln!("ラベル名取得エラー： {}", err);
            Vec::<String>::new()
        }
    }
}

//  !   //-------------------------------------------------------------------
//  *   指定したシートの２行目以降から数値としてデータを取得する
//  !   //-------------------------------------------------------------------
pub async fn fetch_graph_data(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    config: &FetchSpreadSheetConfig,
) -> Vec<String> {
    let sheet_range = format!("{}!A2:Z{}", config.fetch_sheet_name, config.fetch_range);
    let result = read_file(hub, spreadsheet_id, &sheet_range).await;

    match result {
        Ok((_, value_range)) => {
            let vec_values: Vec<String> = value_range.values.map_or(Vec::new(), |values| {
                values
                    .iter()
                    .flat_map(|row| {
                        row.iter().map(|value| {
                            let cell_content = value.to_string().replace(['\"', ','], "");
                            if cell_content.is_empty() {
                                String::from("")
                            } else {
                                cell_content
                            }
                        })
                    })
                    .collect()
            });
            vec_values
        }
        Err(err) => {
            eprintln!("データ取得エラー： {}", err);
            Vec::<String>::new()
        }
    }
}

//  !   //-------------------------------------------------------------------
//  todo
//  *   スプレッドシートのデータとラベルを受け取ってグラフ用のデータに成形する
//  !   //-------------------------------------------------------------------
//

//  !   //-------------------------------------------------------------------
//  todo
//  *   オプションファイルの作成
//  !   //-------------------------------------------------------------------
// async fn make_option_file_data(config: &FetchSpreadSheetConfig, update_epoch_time: u64) -> String {
//     match config.graph_pattern {
//         "bar" => make_option_bar(config, update_epoch_time).await,
//         // "horibar" => make_option_horibar(config, update_epoch_time).await,
//         // "floathoribar" => make_option_float_hori_bar(config, update_epoch_time).await,
//         // "bullet" => make_option_bullet(config, update_epoch_time).await,
//         // "gauge" => make_option_gauge(config, update_epoch_time).await,
//         "combo" => make_option_combo_srs1a(config, update_epoch_time).await,
//         "line" => String::from(""),
//         "circle" => String::from(""),

//         &_ => String::from("value"),
//     }
// }

//  !   //-------------------------------------------------------------------
//  *   グラフ用データファイルの書き込み処理
//  !   //-------------------------------------------------------------------
async fn write_javascript_data_file(
    graph_data: String,
    option_data: String,
    config: &FetchSpreadSheetConfig,
    data_path: &str,
) -> std::result::Result<(), Error> {
    //  データファイルの書き込み
    let result_data_write = write_file(
        &format!("data_{}", config.save_graph_data_name),
        &graph_data,
        data_path,
    )
    .await;
    let result_option_write = write_file(
        &format!("options_{}", config.save_graph_data_name),
        &option_data,
        data_path,
    )
    .await;

    if result_data_write.is_ok() && result_option_write.is_ok() {
        Ok(())
    } else if result_data_write.is_err() {
        result_data_write
    } else {
        result_option_write
    }
}

//  !   //-------------------------------------------------------------------
//  *   実際にデータを書き込む
//  !   //-------------------------------------------------------------------
async fn write_file(file_name: &str, data: &str, data_path: &str) -> Result<(), Error> {
    let file_path = format!("{}{}", data_path, file_name);

    //  !   ファイル書き込み
    let result = tokio::fs::write(&file_path, data).await;

    //  !   スプレッドシート書き込み
    if result.is_ok() {
    } else if let Err(e) = &result {
        eprintln!("ファイル書き込みエラー： {}", e);
        eprintln!("ファイルパス{}", &file_path);
    }
    result
}

//  todo
//  todo
//  todo

//  !   //-------------------------------------------------------------------
//  *   グラフ用データファイルの書き込み処理
//  !   //-------------------------------------------------------------------
async fn write_spreadsheet_data_file(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    graph_data: String,
    option_data: String,
    target_data: String,
    config: &FetchSpreadSheetConfig,
    index: usize,
) -> std::result::Result<(), Error> {
    //  データファイルの書き込み
    let result_data_write =
        update_value(hub, config, &graph_data, &option_data, &target_data, index).await;
    // let result_option_write = update_spreadsheet(hub, config, &option_data, "options").await;

    if result_data_write.is_ok() {
        println!("書き込み成功 {}", config.output_sheet_name);
        Ok(())
    } else {
        eprintln!("書き込み失敗");
        Err(Error::new(ErrorKind::Other, "エラー".to_string()))
    }
}

//  !   //-------------------------------------------------------------------
//  *   実際にデータを書き込む
//  !   //-------------------------------------------------------------------
//
