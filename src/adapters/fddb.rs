use std::fs;
use chrono::{Local, DateTime, NaiveDate};
use dotenv::dotenv;

use crate::database::db;
use crate::database::fddb_db;
use crate::graph::graph_f32;
use crate::models::fddb::Fddb;

pub async fn main() {
    load_data().await;
    //_ = plot_graph().await;
    let data: Vec<Fddb> = fddb_db::get_fddb_data().await;
    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points(data);
    let title: &str = "Weight Data";
    _ = graph_f32(String::from("plots/fddb_weight.png"), datapoints, title).await;
}

async fn load_data() {
    // Checks if there is a new file in the fddb Folder and if that is the case, 
    // adds all the data from that File to the DB
    dotenv().ok();
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
            Ok(boo) => { 
                match boo {
                    true => { // read new data
                        read_file(&file).await;
                    },
                    false => { }, // skip file
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
    for line in splits {
        let mut values = Vec::new();
        for value in line.split(";") {
            values.push(value);
        }
        if values.len() > 1 {
            fddb_db::add_fddb_entry(values).await;
        }
    }
}

fn convert_data_to_points(data: Vec<Fddb>) -> Vec<(NaiveDate, f32)> {
    data.iter().map(|item| (item.work_date, item.weight as f32)).collect()
}