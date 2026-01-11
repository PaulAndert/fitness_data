use std::fs;
use chrono::Duration;
use chrono::{Local, DateTime, NaiveDate};

use crate::helper;
use crate::store;
use crate::dto::yaxis_enum::YAxis;
use crate::dto::concept2_dto::Concept2Dto;
use crate::dto::range::Range;

pub async fn main() {
    let path: &str = &std::env::var("PLOTS_PATH").expect("PLOTS_PATH must be set.");

    let workout: String = helper::io_helper::ask_input_question("State the name of the workout to filter. (2000m row, ...)");

    let range: Range = helper::io_helper::ask_range();
    let mut title: String = format!("Concept2 Data, {}, ", workout);

    let options: Vec<&str> = vec!["duration", "distance", "stroke rate", "stroke count", "pace", "watts"];
    let answer: usize = helper::io_helper::ask_choice_question("What data should be displayed on the Y-Axis?", options);
    let y_axis: YAxis = match answer {
        1 => YAxis::Duration,
        2 => YAxis::Distance,
        3 => YAxis::StrokeRate,
        4 => YAxis::StrokeCount,
        5 => YAxis::Pace,
        6 => YAxis::Watts,
        _ => panic!("The option specified is not valid: {}", answer)
    };

    let data = store::concept2_store::get_concept2_data(workout.as_str(), range.start, range.end).await;
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
            _ = helper::graph::graph_duration(destination, datapoints, title.as_str()).await;
        },
        YAxis::Distance | YAxis::StrokeRate | 
        YAxis::StrokeCount | YAxis::Watts => {
            let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points_f32(data, y_axis.clone());
            _ = helper::graph::graph_f32(destination, datapoints, title.as_str()).await;
        }
    }
}

pub async fn load_data() {
    let path: &str = &std::env::var("CONCEPT2_PATH").expect("CONCEPT2_PATH must be set.");
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
        .map(|line: &str| line.split(",").collect())
        .filter(|line_parts: &Vec<&str>| line_parts.len() > 1)
        .collect();

    store::concept2_store::add_concept2_entries(all_splited_lines).await;
}

fn convert_data_to_points_f32(data: Vec<Concept2Dto>, y_axis: YAxis) -> Vec<(NaiveDate, f32)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,f32) Tuples
    match y_axis {
        YAxis::Distance => data.iter().map(|item| (item.work_date.date_naive(), item.distance as f32)).collect(),
        YAxis::StrokeRate => data.iter().map(|item| (item.work_date.date_naive(), item.stroke_rate as f32)).collect(),
        YAxis::StrokeCount => data.iter().map(|item| (item.work_date.date_naive(), item.stroke_count as f32)).collect(),
        YAxis::Watts => data.iter().map(|item| (item.work_date.date_naive(), item.watts as f32)).collect(),
        _ => panic!("Error: no y-axis specified. {:?}", y_axis)
    }
}

fn convert_data_to_points_duration(data: Vec<Concept2Dto>, y_axis: YAxis) -> Vec<(NaiveDate, Duration)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,Duration) Tuples
    match y_axis {
        YAxis::Duration => data.iter().map(|item| (item.work_date.date_naive(), item.duration)).collect(),
        YAxis::Pace => data.iter().map(|item| (item.work_date.date_naive(), item.pace )).collect(),
        _ => panic!("Error: no y-axis specified. {:?}", y_axis)
    }
}