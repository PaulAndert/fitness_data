use std::fs;
use chrono::NaiveDate;

use crate::helper;
use crate::store;
use crate::dto::{fddb_dto::FddbDto, range::Range};

pub async fn main() {
    let path: &str = &std::env::var("PLOTS_PATH").expect("PLOTS_PATH must be set.");

    let range: Range = helper::io_helper::ask_range();
    let mut title: String = String::from("Weight Data");
    if range.start.is_some() && range.end.is_some() {
        title = format!("{}: {} - {}", title, range.start.unwrap(), range.end.unwrap());
    }

    let data: Vec<FddbDto> = store::fddb_store::get_fddb_data(range.start, range.end).await;
    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points(data);
    _ = helper::graph::graph_f32(format!("{}/fddb_weight.png", path), datapoints, &title).await;
}

pub async fn load_data() {
    let path: &str = &std::env::var("FDDB_PATH").expect("FDDB_PATH must be set.");
        for (name, file) in helper::files::search_new_files(path, ".csv").await {
        read_file(&file).await;
        println!("Loaded {}", name);
    }
}

async fn read_file(file: &std::fs::DirEntry) {
    // read content from given file and add all Datapoints to the DB
    let contents = fs::read_to_string(file.path()).expect("Should have been able to read the file");
    let mut splits = contents.split("\n");
    
    // ignore first line
    splits.next();

    let all_splited_lines: Vec<Vec<&str>> = splits
        .into_iter()
        .map(|line: &str| line.split(";").collect())
        .filter(|line_parts: &Vec<&str>| line_parts.len() > 1)
        .collect();

    store::fddb_store::add_fddb_entries(all_splited_lines).await;
}

fn convert_data_to_points(data: Vec<FddbDto>) -> Vec<(NaiveDate, f32)> {
    data.iter().map(|item| (item.work_date, item.weight as f32)).collect()
}