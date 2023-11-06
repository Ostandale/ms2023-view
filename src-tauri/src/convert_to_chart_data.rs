use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::DATA_REFRESH_INTERVAL;
use crate::fetch_convert_output::process_and_output_data::process_and_output_data;
use crate::struct_data::FetchSpreadSheetConfig;

use google_sheets4::Sheets;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

//  !   情報ソース（スプレッドシート）からデータを取得し
//  !   グラフ用にデータを整形し
//  !   グラフ描画DB（スプレッドシート）に出力する

pub async fn convert_to_chart_data(sheet: Sheets<HttpsConnector<HttpConnector>>) {
    //  描画するグラフ情報を集めたベクターの作成
    let spreadsheet_config = FetchSpreadSheetConfig::new();
    //  描画するグラフの更新時間のベクターの作成
    let max_graph_number = spreadsheet_config.len();
    let mut last_update: Vec<u64> = vec![0u64; max_graph_number];

    loop {
        //  現在時間の取得
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let epoch_time = since_epoch.as_secs();

        //  更新時間が指定したリフレッシュ時間を超えていたら、データを取得、変換、出力する
        for (last_update_time, config) in last_update.iter_mut().zip(spreadsheet_config.iter()) {
            //  更新条件
            //
            let should_update = *last_update_time == 0
                || (*last_update_time > 0
                    && config.update_interval != -1
                    && (*last_update_time + config.update_interval as u64) < epoch_time);

            if should_update {
                *last_update_time = epoch_time;
                let result = process_and_output_data(&sheet, config).await;
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "グラフデータ準備プログラム実行中にエラーが発生しました{}",
                            e
                        );
                    }
                }
            }
        }

        //  データ監視間隔
        tokio::time::sleep(std::time::Duration::from_secs(DATA_REFRESH_INTERVAL)).await;
    }
}