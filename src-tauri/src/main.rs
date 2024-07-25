// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use polars::prelude::*;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};

enum FileType {
    SiteDep,
    LinkDep,
}

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

fn add_unique_site_counts(df: &mut DataFrame, file_path: &str) {
    let rows = df.height();
    let mut total_sites: Vec<i64> = Vec::new();

    let root_path: Vec<&str> = file_path.split('/').collect();
    // root_path.truncate(root_path.len() - 1);

    let file_type = if root_path.last().unwrap().contains("site_dep") {
        FileType::SiteDep
    } else {
        FileType::LinkDep
    };

    for row in 0..rows {
        let el = df.get_row(row).unwrap();
        let mut item = 0;
        let mut mp: HashSet<String> = HashSet::new();
        for col in el.0 {
            item += 1;
            if item <= 2 {
                continue;
            }
            match col {
                AnyValue::Null => {
                    break;
                }
                _ => {
                    match file_type {
                        FileType::LinkDep => {
                            let col_str = col.to_string();

                            let new_col: Vec<&str> = col_str.split("::").collect();

                            mp.insert(new_col.get(0).unwrap().to_string());
                        }
                        FileType::SiteDep => {
                            mp.insert(col.to_string());
                        }
                    };
                }
            }
        }
        let total_site = mp.len() as i64;
        total_sites.push(total_site);
    }
    let series = polars::prelude::Series::new("dep_sites", total_sites.iter());
    df.insert_column(1, series).unwrap();

    let mut file = File::create(format!(
        "{}_{}",
        file_path.replace(".csv", ""),
        "site_count.csv".to_owned(),
    ))
    .expect("could not create file");

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(df)
        .unwrap();
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
    add_unique_site_counts(&mut df, file_path);
    format!("Completed")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_and_find_dependencies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
