use crate::struct_data::FetchSpreadSheetConfig;

#[tauri::command]
pub fn fetch_struct_data() -> Vec<FetchSpreadSheetConfig> {
    FetchSpreadSheetConfig::new()
}
