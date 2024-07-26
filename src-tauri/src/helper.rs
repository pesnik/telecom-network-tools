use polars::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};

pub fn add_necessary_header_column(file_path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let first_line_end = buffer.find('\n').unwrap_or(buffer.len());

    let mut max_col_needed = 0;
    buffer.split('\n').into_iter().for_each(|x| {
        let current_cnt: Vec<&str> = x.split(',').collect();
        max_col_needed = max_col_needed.max(current_cnt.len());
    });

    let mut new_header = String::from("site::MAC,vlanid,service");
    for i in 1..=((max_col_needed - 3) / 2) {
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

enum FileType {
    SiteDep,
    LinkDep,
}

pub fn add_unique_site_counts(df: &mut DataFrame, file_path: &str) {
    let rows = df.height();
    let mut total_sites: Vec<i64> = Vec::new();

    let file_type = if file_path.contains("site_dep") {
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
                    let target_column_value = match file_type {
                        FileType::LinkDep => {
                            let col_str = col.to_string();

                            let new_col: Vec<&str> = col_str.split("::").collect();

                            new_col.get(0).unwrap().to_string()
                        }
                        FileType::SiteDep => col.to_string(),
                    };
                    mp.insert(target_column_value);
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
