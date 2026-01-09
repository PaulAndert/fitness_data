use core::panic;
use std::fs;
use chrono::{Local, DateTime, NaiveDate};

use crate::helper::graph::graph_f32;
use crate::database::db;
use crate::database::fddb_db;
use crate::helper::io_helper::*;
use crate::models::fddb::Fddb;
use crate::models::range::Range;

pub async fn main() {
    let path: &str = &std::env::var("PLOTS_PATH").expect("PLOTS_PATH must be set.");

    let range: Range = ask_range();
    let mut title: String = String::from("Weight Data");
    if range.start.is_some() && range.end.is_some() {
        title = format!("{}: {} - {}", title, range.start.unwrap(), range.end.unwrap());
    }

    let data: Vec<Fddb> = fddb_db::get_fddb_data(range.start, range.end).await;
    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points(data);
    _ = graph_f32(format!("{}/fddb_weight.png", path), datapoints, &title).await;
}

pub async fn load_data() {
    // Checks if there is a new file in the fddb Folder and if that is the case, 
    // adds all the data from that File to the DB
    let path: &str = &std::env::var("FDDB_PATH").expect("FDDB_PATH must be set.");

    let dir_list = fs::read_dir(path).unwrap();

    for file_res in dir_list {
        let file = match file_res {
            Ok(file) => { file },
            Err(e) => { panic!("Error: {}", e); },
        };
        let metadata = match file.metadata() {
            Ok(meta) => { meta }
            Err(e)=> { panic!("Error: {:?}", e); }
        };
        let last_modified = match metadata.modified() {
            Ok(systime) => {
                let last_modified: DateTime<Local> = systime.clone().into();
                last_modified
            },
            Err(e)=> { panic!("Error: {:?}", e); }
        };
        let filename = match file.file_name().into_string() {
            Ok(name) => { name },
            Err(e) => { panic!("Error: {:?}", e); },
        };
        match db::is_db_up_to_date(&filename, last_modified).await {
            Ok(bool) => { 
                match bool {
                    true => {}, // skip
                    false => {  // read new data
                        read_file(&file).await;
                        println!("Loaded {}", filename);
                    },
                }    
            },
            Err(e) => { panic!("Error: {:?}", e); },
        };
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

    fddb_db::add_fddb_entries(all_splited_lines).await;
}

fn convert_data_to_points(data: Vec<Fddb>) -> Vec<(NaiveDate, f32)> {
    data.iter().map(|item| (item.work_date, item.weight as f32)).collect()
}