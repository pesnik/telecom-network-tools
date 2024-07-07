// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use polars::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};

fn add_more_header_column(file_path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let first_line_end = buffer.find('\n').unwrap_or(buffer.len());

    let mut new_header = String::from("site::MAC,vlanid,service");
    for i in 1..=50 {
        new_header.push_str(&format!(",i{},s{}", i, i));
    }

    let new_content = new_header + &buffer[first_line_end..];

    file.seek(SeekFrom::Start(0))?;

    let mut writer = BufWriter::new(&file);
    writer.write_all(new_content.as_bytes())?;
    writer.flush()?;

    println!("File successfully modified.");

    Ok(())
}

fn read_csv(file_path: &str) -> Result<DataFrame, PolarsError> {
    let df = CsvReadOptions::default()
        .with_parse_options(CsvParseOptions::default().with_separator(',' as u8))
        .with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some(file_path.into()))?
        .finish()?;

    Ok(df)
}

#[tauri::command]
fn greet(name: &str) -> String {
    let error_occured = add_more_header_column(name);
    match error_occured {
        Err(err) => {
            return format!("Error {:?} occured", err)
        }
        _ => {}
    }
    let dataframe = read_csv(name);
    match dataframe {
        Ok(df) => {
            println!("{:?}", df);
            format!("Hello, {}! You've been greeted from Rust!", df.height())
        }
        Err(err) => {
            println!("{:?}", err);
            format!("Error {:?} occured", err)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
