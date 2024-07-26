// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod helper;

use helper::{add_necessary_header_column, add_unique_site_counts};
use polars::prelude::*;

#[tauri::command]
fn parse_and_find_dependencies(file_path: &str) -> String {
    add_necessary_header_column(file_path).unwrap();
    let mut df = CsvReadOptions::default()
        .with_parse_options(CsvParseOptions::default().with_separator(',' as u8))
        .with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some(file_path.into()))
        .unwrap()
        .finish()
        .unwrap();
    add_unique_site_counts(&mut df, file_path);
    format!("Completed")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_and_find_dependencies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
