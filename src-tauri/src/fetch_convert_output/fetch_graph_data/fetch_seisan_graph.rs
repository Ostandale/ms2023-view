use crate::fetch_convert_output::sheet_control::read_file;
use crate::struct_data::FetchSpreadSheetConfig;
use google_sheets4::Sheets;

//  todo
//  !   //-------------------------------------------------------------------
//  *   fetch_product1_graph_label
//  !   //-------------------------------------------------------------------
pub async fn fetch_seisan_graph_label(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
) -> Vec<String> {
    let sheet_range = format!("{}!{}", config.fetch_sheet_name, config.fetch_range_label);
    let result = read_file(hub, config.sheet_id, &sheet_range).await;

    match result {
        Ok((_, value_range)) => {
            let vec_values: Vec<String> = value_range.values.map_or(Vec::new(), |values| {
                values
                    .iter()
                    .flat_map(|row| {
                        row.iter().map(|value| {
                            let cell_content = value.to_string().replace('\"', "");
                            if cell_content.is_empty() {
                                String::from("_")
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
            eprintln!("{} ラベル名取得エラー： {}", config.output_sheet_name, err);
            Vec::<String>::new()
        }
    }
}
//  todo
//  !   //-------------------------------------------------------------------
//  *   fetch_product1_graph_data
//  !   //-------------------------------------------------------------------
pub async fn fetch_seisan_graph_data(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
) -> Vec<String> {
    //グラフデータを取得
    let sheet_range = format!("{}!{}", config.fetch_sheet_name, config.fetch_range_data);
    let result = read_file(hub, config.sheet_id, &sheet_range).await;

    match result {
        Ok((_, value_range)) => {
            let vec_values: Vec<String> = value_range.values.map_or(Vec::new(), |values| {
                values
                    .iter()
                    .flat_map(|row| {
                        row.iter().map(|value| {
                            let cell_content = value.to_string().replace(['\"', ','], "");
                            if cell_content.is_empty() {
                                String::from("0")
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
            eprintln!("{} データ取得エラー： {}", config.output_sheet_name, err);
            Vec::<String>::new()
        }
    }
}

//  todo
//  !   //-------------------------------------------------------------------
//  *   fetch_product1_graph_target
//  !   //-------------------------------------------------------------------
pub async fn fetch_seisan_graph_target(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
) -> Vec<String> {
    //デイリー目標を取得
    let sheet_range = format!(
        "{}!{}:{}",
        config.fetch_sheet_name, config.fetch_range_target_monthly, config.fetch_range_target_daily
    );
    let result = read_file(hub, config.sheet_id, &sheet_range).await;

    match result {
        Ok((_, value_range)) => {
            let vec_values: Vec<String> = value_range.values.map_or(Vec::new(), |values| {
                values
                    .iter()
                    .flat_map(|row| {
                        row.iter().map(|value| {
                            let cell_content = value.to_string().replace(['\"', ','], "");
                            if cell_content.is_empty() {
                                String::from("0")
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
            eprintln!("{} データ取得エラー： {}", config.output_sheet_name, err);
            Vec::<String>::new()
        }
    }
}
