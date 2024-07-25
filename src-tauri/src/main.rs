// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use polars::prelude::*;
use std::collections::HashSet;
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

fn add_unique_site_counts(df: &mut DataFrame) {
    let rows = df.height();
    let mut total_sites: Vec<i64> = Vec::new();

    for row in 0..rows {
        let el = df.get_row(row).unwrap();
        // println!("{:?}", el)
        let mut item = 0;
        let mut mp: HashSet<String>= HashSet::new();
        for col in el.0 {
            item += 1;
            if item <= 2 {
                continue;
            }
            match col {
                AnyValue::Null => {
                    break;
                },
                _ => {
                    mp.insert(col.to_string());
                }
            }
        }
        let total_site = mp.len() as i64;
        total_sites.push(total_site);
        // println!("{:?}, {total_site}", mp);
    }
    let series = polars::prelude::Series::new("dep_sites", total_sites.iter());
    df.insert_column(1, series).unwrap();
    println!("{:?}", df.tail(Some(100)));
}

#[tauri::command]
fn parse_and_find_dependencies(file_path: &str) -> String {
    add_more_header_column(file_path).unwrap();
    let mut df = CsvReadOptions::default()
        .with_parse_options(CsvParseOptions::default().with_separator(',' as u8))
        .with_infer_schema_length(Some(0))
        .try_into_reader_with_file_path(Some(file_path.into()))
        .unwrap()
        .finish()
        .unwrap();
    add_unique_site_counts(&mut df);
    println!("{:?}", df);
    format!("Hello, {}! You've been greeted from Rust!", df.height())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_and_find_dependencies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
