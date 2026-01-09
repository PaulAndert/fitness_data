use std::fs;
use chrono::Duration;
use chrono::{Local, DateTime, NaiveDate};

use crate::helper::args::YAxis;
use crate::helper::graph::{graph_duration, graph_f32};
use crate::database::concept2_db;
use crate::database::db;
use crate::helper::io_helper::*;
use crate::models::concept2::Concept2;
use crate::models::range::Range;

pub async fn main() {
    let path: &str = &std::env::var("PLOTS_PATH").expect("PLOTS_PATH must be set.");

    let workout: String = ask_input_question("State the name of the workout to filter. (2000m row, ...)");

    let range: Range = ask_range();
    let mut title: String = format!("Concept2 Data, {}, ", workout);

    let options: Vec<&str> = vec!["duration", "distance", "stroke rate", "stroke count", "pace", "watts"];
    let answer: usize = ask_choice_question("What data should be displayed on the Y-Axis?", options);
    let y_axis: YAxis = match answer {
        1 => YAxis::Duration,
        2 => YAxis::Distance,
        3 => YAxis::StrokeRate,
        4 => YAxis::StrokeCount,
        5 => YAxis::Pace,
        6 => YAxis::Watts,
        _ => panic!("The option specified is not valid: {}", answer)
    };

    let data = concept2_db::get_concept2_data(workout.as_str(), range.start, range.end).await;
    title += match y_axis {
        YAxis::Duration => "Times in Minutes",
        YAxis::Distance => "Distances in Meters",
        YAxis::StrokeRate => "Avg. Strokes per Minute",
        YAxis::StrokeCount => "Strokecount",
        YAxis::Pace => "Avg. Pace (Minutes per 500m)",
        YAxis::Watts => "Avg. Watts"
    };
    if range.start.is_some() && range.end.is_some() {
        title = format!("{}: {} - {}", title, range.start.unwrap(), range.end.unwrap());
    }

    let destination = format!("{}/concept2_{}.png", path, workout.replace(" ", "_"));
    match y_axis {
        YAxis::Duration | YAxis::Pace => { 
            let datapoints: Vec<(NaiveDate, Duration)> = convert_data_to_points_duration(data, y_axis.clone());
            _ = graph_duration(destination, datapoints, title.as_str()).await;
        },
        YAxis::Distance | YAxis::StrokeRate | 
        YAxis::StrokeCount | YAxis::Watts => {
            let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points_f32(data, y_axis.clone());
            _ = graph_f32(destination, datapoints, title.as_str()).await;
        }
    }
}

pub async fn load_data() {
    // Checks if there is a new file in the Concept2 Folder and if that is the case, 
    // adds all the data from that File to the DB
    let path: &str = &std::env::var("CONCEPT2_PATH").expect("CONCEPT2_PATH must be set.");

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
        .map(|line: &str| line.split(",").collect())
        .filter(|line_parts: &Vec<&str>| line_parts.len() > 1)
        .collect();

    concept2_db::add_concept2_entries(all_splited_lines).await;
}

fn convert_data_to_points_f32(data: Vec<Concept2>, y_axis: YAxis) -> Vec<(NaiveDate, f32)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,f32) Tuples
    match y_axis {
        YAxis::Distance => data.iter().map(|item| (item.work_date.date_naive(), item.distance as f32)).collect(),
        YAxis::StrokeRate => data.iter().map(|item| (item.work_date.date_naive(), item.stroke_rate as f32)).collect(),
        YAxis::StrokeCount => data.iter().map(|item| (item.work_date.date_naive(), item.stroke_count as f32)).collect(),
        YAxis::Watts => data.iter().map(|item| (item.work_date.date_naive(), item.watts as f32)).collect(),
        _ => panic!("Error: no y-axis specified. {:?}", y_axis)
    }
}

fn convert_data_to_points_duration(data: Vec<Concept2>, y_axis: YAxis) -> Vec<(NaiveDate, Duration)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,Duration) Tuples
    match y_axis {
        YAxis::Duration => data.iter().map(|item| (item.work_date.date_naive(), item.duration)).collect(),
        YAxis::Pace => data.iter().map(|item| (item.work_date.date_naive(), item.pace )).collect(),
        _ => panic!("Error: no y-axis specified. {:?}", y_axis)
    }
}