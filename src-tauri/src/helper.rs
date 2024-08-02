use polars::lazy::dsl::col;
use polars::lazy::dsl::GetOutput;
use polars::lazy::prelude::*;
use polars::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use tauri::regex::Regex;

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
    for i in 1..=((max_col_needed - 3) as f64 / 2.0).ceil() as usize {
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

fn get_site_dep_df(df: &mut DataFrame) -> Result<DataFrame, PolarsError> {
    df.clone()
        .lazy()
        .select([
            col("site::MAC")
                .str()
                .replace(lit(r"_NE_.*"), lit(""), false),
            col("*").exclude(["site::MAC"]),
        ])
        .group_by(["site::MAC"])
        .agg([
            concat_str([col("*").exclude(["site::MAC", "vlanid"])], ",", true).alias("Dep. Sites"),
        ])
        .with_column(col("Dep. Sites").apply(
            |s: Series| {
                let list_series = s.list()?;
                let mut result = Vec::new();
                let re = Regex::new(r"_NE_.*").unwrap();

                for opt_list in list_series.into_iter() {
                    if let Some(list) = opt_list {
                        let mut set = HashSet::new();
                        for value in list.iter() {
                            if let Some(site) = value.get_str() {
                                site.split(",").into_iter().for_each(|candidate| {
                                    let modified = re.replace(candidate, "").into_owned();
                                    set.insert(modified);
                                });
                            }
                        }
                        result.push(set.into_iter().collect::<Vec<String>>().join(","));
                    } else {
                        result.push(String::new());
                    }
                }

                Ok(Series::new("Sites", result).into())
            },
            GetOutput::from_type(DataType::String),
        ))
        .with_column(
            col("Dep. Sites")
                .apply(
                    |s: Series| {
                        let list_series = s.str()?;
                        let mut result: Vec<String> = Vec::new();
                        for site_list in list_series.into_iter() {
                            if let Some(sites) = site_list {
                                result.push(sites.to_string().split(",").count().to_string());
                            } else {
                                result.push(0.to_string());
                            }
                        }
                        Ok(Series::new("Dep. Sites", result).into())
                    },
                    GetOutput::from_type(DataType::String),
                )
                .alias("Dep. Site Count"),
        )
        .select([
            col("site::MAC").alias("Sites"),
            col("Dep. Site Count").cast(DataType::Int64),
            col("Dep. Sites"),
        ])
        .sort(
            ["Dep. Site Count"],
            SortMultipleOptions::new().with_order_descending(true),
        )
        .collect()
}

fn get_link_dep_df(df: &mut DataFrame) -> Result<DataFrame, PolarsError> {
    df.clone()
        .lazy()
        .group_by([col("site::MAC")])
        .agg([
            concat_str([col("*").exclude(["site::MAC", "vlanid"])], ",", true).alias("Dep. Sites"),
        ])
        .with_column(col("Dep. Sites").apply(
            |s: Series| {
                let list_series = s.list()?;
                let mut result = Vec::new();
                let re = Regex::new(r"_NE_.*").unwrap();

                for opt_list in list_series.into_iter() {
                    if let Some(list) = opt_list {
                        let mut set = HashSet::new();
                        for value in list.iter() {
                            if let Some(value_str) = value.get_str() {
                                let col_str = value_str.to_string();
                                col_str.split(",").into_iter().for_each(|site| {
                                    let modified = re.replace(site, "").into_owned();
                                    set.insert(modified);
                                });
                            }
                        }
                        result.push(set.into_iter().collect::<Vec<String>>().join(","));
                    } else {
                        result.push(String::new());
                    }
                }

                Ok(Series::new("Sites", result).into())
            },
            GetOutput::from_type(DataType::String),
        ))
        .with_column(
            col("Dep. Sites")
                .apply(
                    |s: Series| {
                        let list_series = s.str()?;
                        let mut result: Vec<String> = Vec::new();
                        for site_list in list_series.into_iter() {
                            if let Some(sites) = site_list {
                                result.push(sites.to_string().split(",").count().to_string());
                            } else {
                                result.push(0.to_string());
                            }
                        }
                        Ok(Series::new("Dep. Sites", result).into())
                    },
                    GetOutput::from_type(DataType::String),
                )
                .alias("Dep. Site Count"),
        )
        .select([
            col("site::MAC").alias("Sites"),
            col("Dep. Site Count").cast(DataType::Int64),
            col("Dep. Sites"),
        ])
        .sort(
            ["Dep. Site Count"],
            SortMultipleOptions::new().with_order_descending(true),
        )
        .collect()
}

fn get_processed_dataframe(
    df: &mut DataFrame,
    file_type: FileType,
) -> Result<DataFrame, PolarsError> {
    match file_type {
        FileType::SiteDep => get_site_dep_df(df),
        FileType::LinkDep => get_link_dep_df(df),
    }
}

pub fn add_unique_site_counts(df: &mut DataFrame, file_path: &str) -> Result<(), PolarsError> {
    let file_type = if file_path.contains("site_dep") {
        FileType::SiteDep
    } else {
        FileType::LinkDep
    };

    let mut new_df = get_processed_dataframe(df, file_type).unwrap();
    let mut file = File::create(format!(
        "{}_{}",
        file_path.replace(".csv", ""),
        "site_count.csv"
    ))
    .expect("could not create file");

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut new_df)?;

    Ok(())
}
