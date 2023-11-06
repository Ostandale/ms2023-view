export interface ConfigStruct {
    sheet_id: string;
    fetch_sheet_name: string;
    fetch_range: number;
    data_type: string;
    graph_pattern: string;
    save_graph_data_name: string;
    update_interval: number;
    output_sheet_name: string;
    output_sheet_range: string;
    fetch_range_label: string;
    fetch_range_data: string;
    fetch_range_target_monthly: string;
    fetch_range_target_daily: string;
}