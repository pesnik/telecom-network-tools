// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod helper;

use helper::{add_necessary_header_column, add_unique_site_counts};
use polars::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    status: i32,
    message: String,
}

impl Response {
    fn new(status: i32, message: String) -> Response {
        Self { status, message }
    }
}
#[tauri::command]
fn parse_and_find_dependencies(file_path: &str) -> String {
    if let Err(err) = add_necessary_header_column(file_path) {
        let response = Response::new(1, err.to_string());
        return serde_json::to_string(&response)
            .unwrap_or_else(|_| "Error serializing response".to_string());
    }

    let mut df = match CsvReadOptions::default()
        .with_parse_options(CsvParseOptions::default().with_separator(',' as u8))
        .with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some(file_path.into()))
        .and_then(|reader| reader.finish())
    {
        Ok(df) => df,
        Err(err) => {
            let response = Response::new(1, err.to_string());
            return serde_json::to_string(&response)
                .unwrap_or_else(|_| "Error serializing response".to_string());
        }
    };

    if let Err(err) = add_unique_site_counts(&mut df, file_path) {
        let response = Response::new(1, err.to_string());
        return serde_json::to_string(&response)
            .unwrap_or_else(|_| "Error serializing response".to_string());
    }

    let response = Response::new(0, "Completed".to_string());
    serde_json::to_string(&response).unwrap_or_else(|_| "Error serializing response".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_and_find_dependencies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
