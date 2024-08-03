use polars::lazy::dsl::col;
use polars::lazy::dsl::GetOutput;
use polars::lazy::prelude::*;
use polars::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use tauri::regex::Regex;

#[derive(Debug, Clone, Copy)]
enum FileType {
    SiteDep,
    LinkDep,
}

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

fn process_dep_sites(s: Series, file_type: FileType) -> Result<Option<Series>, PolarsError> {
    let re = Regex::new(r"_NE_.*").unwrap();

    let result: Vec<String> = s
        .list()?
        .into_iter()
        .map(|opt_list| {
            opt_list.map_or_else(
                || String::new(),
                |list| {
                    let mut sites: Vec<String> = list
                        .iter()
                        .filter_map(|value| value.get_str().map(|s| s.to_string()))
                        .flat_map(|site| site.split(',').map(String::from).collect::<Vec<String>>())
                        .collect();

                    if let FileType::LinkDep = file_type {
                        sites = sites
                            .into_iter()
                            .map(|candidate| re.replace(&candidate, "").into_owned())
                            .collect();
                    }

                    sites
                        .into_iter()
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect::<Vec<String>>()
                        .join(",")
                },
            )
        })
        .collect();

    Ok(Some(Series::new("Sites", result)))
}

fn count_dep_sites(s: Series) -> Result<Option<Series>, PolarsError> {
    let result: Vec<String> = s
        .str()?
        .into_iter()
        .map(|site_list| {
            site_list.map_or_else(
                || 0.to_string(),
                |sites| sites.split(',').count().to_string(),
            )
        })
        .collect();

    Ok(Some(Series::new("Dep. Site Count", result)))
}

fn get_dep_df(df: &mut DataFrame, file_type: FileType) -> Result<DataFrame, PolarsError> {
    let df = df
        .clone()
        .lazy()
        .select(match file_type {
            FileType::SiteDep => [
                col("site::MAC")
                    .str()
                    .replace(lit(r"_NE_.*"), lit(""), false),
                col("*").exclude(["site::MAC"]),
            ]
            .to_vec(),
            FileType::LinkDep => [col("*")].to_vec(),
        })
        .group_by([col("site::MAC")])
        .agg([
            concat_str([col("*").exclude(["site::MAC", "vlanid"])], ",", true).alias("Dep. Sites"),
        ])
        .with_column(col("Dep. Sites").apply(
            move |s| process_dep_sites(s, file_type),
            GetOutput::from_type(DataType::String),
        ))
        .with_column(
            col("Dep. Sites").apply(count_dep_sites, GetOutput::from_type(DataType::String)).alias("Dep. Site Count"),
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
        .collect();

    df
}

fn get_processed_dataframe(
    df: &mut DataFrame,
    file_type: FileType,
) -> Result<DataFrame, PolarsError> {
    get_dep_df(df, file_type)
}

pub fn add_unique_site_counts(df: &mut DataFrame, file_path: &str) -> Result<(), PolarsError> {
    let file_type = if file_path.contains("site_dep") {
        FileType::SiteDep
    } else {
        FileType::LinkDep
    };

    let mut new_df = get_processed_dataframe(df, file_type)?;
    let file_name = format!("{}_{}", file_path.replace(".csv", ""), "site_count.csv");
    let mut file = File::create(file_name).expect("could not create file");

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut new_df)?;

    Ok(())
}
