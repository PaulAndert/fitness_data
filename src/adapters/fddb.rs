use core::panic;
use std::fs;
use chrono::{Days, Months};
use chrono::{Local, DateTime, NaiveDate};

use crate::helper::graph::graph_f32;
use crate::database::db;
use crate::database::fddb_db;
use crate::helper::io_helper::*;
use crate::models::fddb::Fddb;

pub async fn main() {
    let path: &str = &std::env::var("PLOTS_PATH").expect("PLOTS_PATH must be set.");

    let options = vec!["No", "Yes, by date range", "Yes, by range description (last x weeks, ...)", "No, load new Fddb data"];
    let answer: usize = ask_choice_question("Do you want to filter the data?", options);

    let mut title: String = String::from("Weight Data");
    let data: Vec<Fddb> = match answer {
        1 => fddb_db::get_fddb_data(None, None).await,
        2 => {
            let answer_start: String = ask_input_question("Plese give the date from when the data should be used. (Plese use the format yyyy-mm-dd)");
            let range_start: NaiveDate = match NaiveDate::parse_from_str(&answer_start, "%Y-%m-%d") {
                Ok(date) => date,
                Err(e) => panic!("The Date specified is either invalid or in the wrong format: {}\n{}", answer_start, e)
            };
            let answer_end: String = ask_input_question("Plese give the date until when the data should be used. (Plese use the format yyyy-mm-dd)");
            let range_end: NaiveDate = match NaiveDate::parse_from_str(&answer_end, "%Y-%m-%d") {
                Ok(date) => date,
                Err(e) => panic!("The Date specified is either invalid or in the wrong format: {}\n{}", answer_end, e)
            };
            title = format!("{}: {} - {}", title, answer_start, answer_end);

            fddb_db::get_fddb_data(Some(range_start), Some(range_end)).await
        },
        3 => {
            let options = vec!["Days", "Weeks", "Months", "Quarters", "Years"];
            let answer_range_specifier = ask_choice_question("What range do you want to specify?", options.clone());

            if answer_range_specifier <= 0 && answer_range_specifier > options.len() {
                panic!("The Option specified is not valid: {}", answer_range_specifier);
            }

            let answer_range_size_string: String = ask_input_question(format!("How many {} of data should be used?", options[answer_range_specifier - 1]).as_str());
            let answer_range_size_number: u32 = match answer_range_size_string.parse::<u32>() {
                Ok(number) => number,
                Err(e) => panic!("The Number specified is either invalid or in the wrong format: {}\n{}", answer_range_size_string, e)
            };

            let today: NaiveDate = Local::now().date_naive();
            let range_start: NaiveDate = match answer_range_specifier {
                1 => match today.checked_sub_days(Days::new(answer_range_size_number as u64)) {
                    Some(date) => date,
                    None => panic!("Number of Days could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                2 => match today.checked_sub_days(Days::new((answer_range_size_number as u64) * 7)) {
                    Some(date) => date,
                    None => panic!("Number of Weeks could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                3 => match today.checked_sub_months(Months::new(answer_range_size_number)) {
                    Some(date) => date,
                    None => panic!("Number of Months could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                4 => match today.checked_sub_months(Months::new(answer_range_size_number * 3)) {
                    Some(date) => date,
                    None => panic!("Number of Quarters could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                5 => match today.checked_sub_months(Months::new(answer_range_size_number * 12)) {
                    Some(date) => date,
                    None => panic!("Number of Years could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                _ => panic!("The Option specified is not valid: {}", answer_range_specifier)
            };

            title = format!("{}: last {} {}", title, answer_range_size_number, options[answer_range_specifier - 1]);

            fddb_db::get_fddb_data(Some(range_start), Some(today)).await
        },
        4 => {
            load_data().await;
            return;
        },
        _ => panic!("The Option specified is not valid: {}", answer)
    };

    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points(data);
    _ = graph_f32(format!("{}/fddb_weight.png", path), datapoints, &title).await;
}

async fn load_data() {
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
        println!("{}, {}", filename, last_modified);
        match db::is_db_up_to_date(&filename, last_modified).await {
            Ok(bool) => { 
                match bool {
                    true => {}, // skip
                    false => read_file(&file).await // read new data
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