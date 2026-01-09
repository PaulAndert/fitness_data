use std::fs;
use chrono::Duration;
use chrono::{Local, DateTime, NaiveDate};
use dotenv::dotenv;

use crate::helper::args;
use crate::helper::args::YAxis;
use crate::helper::graph::{graph_duration, graph_f32};
use crate::database::concept2_db;
use crate::database::db;
use crate::models::concept2::Concept2;

pub async fn main() {
    load_data().await;
    // match args.sport {
    //     Some(args::Sport::Rowing) => {
    //         match args.workout {
    //             Some(args::Workout::Min1) => {
    //                 _ = plot_workout("1:00 row", args.y_axis).await;
    //             },
    //             Some(args::Workout::Min10) => {
    //                 _ = plot_workout("10:00 row", args.y_axis).await;
    //             },
    //             Some(args::Workout::Min15) => {
    //                 _ = plot_workout("15:00 row", args.y_axis).await;
    //             },
    //             Some(args::Workout::Meter1k) => {
    //                 _ = plot_workout("1000m row", args.y_axis).await;
    //             },
    //             Some(args::Workout::Meter2k) => {
    //                 _ = plot_workout("2000m row", args.y_axis).await;
    //             },
    //             Some(args::Workout::Meter5k) => { 
    //                 _ = plot_workout("5000m row", args.y_axis).await;
    //             },
    //             None => {
    //                 panic!("Error: Unknown Workout");
    //             }
    //         }
    //     },
    //     _ => {
    //         panic!("Error: Unknown Sport");
    //     }
    // }
}

async fn load_data() {
    // Checks if there is a new file in the Concept2 Folder and if that is the case, 
    // adds all the data from that File to the DB
    dotenv().ok();
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
    for line in splits {
        let mut values = Vec::new();
        for value in line.split(",") {
            values.push(value);
        }
        if values.len() > 1 {
            concept2_db::add_concept2_entry(values).await;
        }
    }
}

async fn plot_workout(workout: &str, y_axis: Option<YAxis>) {
    // Creates a Graph with the Data of the given Workout
    let data = concept2_db::get_concept2_workouts(workout).await;
    let title = get_title(workout, y_axis.clone());
    let destination = format!("plots/concept2_{}.png", workout.replace(" ", "_"));
    match y_axis {
        Some(YAxis::Duration) | Some(YAxis::Pace) => { 
            let datapoints: Vec<(NaiveDate, Duration)> = convert_data_to_points_duration(data, y_axis.clone());
            _ = graph_duration(destination, datapoints, title).await;
        },
        Some(YAxis::Distance) | Some(YAxis::StrokeRate) | 
        Some(YAxis::StrokeCount) | Some(YAxis::Watts) => {
            let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points_f32(data, y_axis.clone());
            _ = graph_f32(destination, datapoints, title).await;
        },
        _ => { 
            panic!("Error: unknown y-axis");
        }
    }
}

fn get_title(workout: &str, y_axis: Option<YAxis>) -> &str {
    match workout {
        "1:00 row" => {
            match y_axis {
                Some(YAxis::Duration) => { "Rowing 1 min, Times in Minutes" },
                Some(YAxis::Distance) => { "Rowing 1 min, Distances in Meters" },
                Some(YAxis::StrokeRate) => { "Rowing 1 min, Avg. Strokes per Minute" },
                Some(YAxis::StrokeCount) => { "Rowing 1 min, Strokecount" },
                Some(YAxis::Pace) => { "Rowing 1 min, Avg. Pace (Minutes per 500m)" },
                Some(YAxis::Watts) => { "Rowing 1 min, Avg. Watts" },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        "10:00 row" => {
            match y_axis {
                Some(YAxis::Duration) => { "Rowing 10 min, Times in Minutes" },
                Some(YAxis::Distance) => { "Rowing 10 min, Distances in Meters" },
                Some(YAxis::StrokeRate) => { "Rowing 10 min, Avg. Strokes per Minute" },
                Some(YAxis::StrokeCount) => { "Rowing 10 min, Strokecount" },
                Some(YAxis::Pace) => { "Rowing 10 min, Avg. Pace (Minutes per 500m)" },
                Some(YAxis::Watts) => { "Rowing 10 min, Avg. Watts" },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        "15:00 row" => {
            match y_axis {
                Some(YAxis::Duration) => { "Rowing 15 min, Times in Minutes" },
                Some(YAxis::Distance) => { "Rowing 15 min, Distances in Meters" },
                Some(YAxis::StrokeRate) => { "Rowing 15 min, Avg. Strokes per Minute" },
                Some(YAxis::StrokeCount) => { "Rowing 15 min, Strokecount" },
                Some(YAxis::Pace) => { "Rowing 15 min, Avg. Pace (Minutes per 500m)" },
                Some(YAxis::Watts) => { "Rowing 15 min, Avg. Watts" },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        "1000m row" => {
            match y_axis {
                Some(YAxis::Duration) => { "Rowing 1000 meter, Times in Minutes" },
                Some(YAxis::Distance) => { "Rowing 1000 meter, Distance in Meter" },
                Some(YAxis::StrokeRate) => { "Rowing 1000 meter, Avg. Strokes per Minute" },
                Some(YAxis::StrokeCount) => { "Rowing 1000 meter, Strokecount" },
                Some(YAxis::Pace) => { "Rowing 1000 meter, Avg. Pace (Minutes per 500m)" },
                Some(YAxis::Watts) => { "Rowing 000 meter, Avg. Watts" },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        "2000m row" => {
            match y_axis {
                Some(YAxis::Duration) => { "Rowing 2000 meter, Times in Minutes" },
                Some(YAxis::Distance) => { "Rowing 2000 meter, Distance in Meter" },
                Some(YAxis::StrokeRate) => { "Rowing 2000 meter, Avg. Strokes per Minute" },
                Some(YAxis::StrokeCount) => { "Rowing 2000 meter, Strokecount" },
                Some(YAxis::Pace) => { "Rowing 2000 meter, Avg. Pace (Minutes per 500m)" },
                Some(YAxis::Watts) => { "Rowing 2000 meter, Avg. Watts" },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        "5000m row" => {
            match y_axis {
                Some(YAxis::Duration) => { "Rowing 5000 meter, Times in Minutes" },
                Some(YAxis::Distance) => { "Rowing 5000 meter, Distance in Meter" },
                Some(YAxis::StrokeRate) => { "Rowing 5000 meter, Avg. Strokes per Minute" },
                Some(YAxis::StrokeCount) => { "Rowing 5000 meter, Strokecount" },
                Some(YAxis::Pace) => { "Rowing 5000 meter, Avg. Pace (Minutes per 500m)" },
                Some(YAxis::Watts) => { "Rowing 5000 meter, Avg. Watts" },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        _ => { 
            "Error: Unknown Workout"
        }
    }
}

fn convert_data_to_points_f32(data: Vec<Concept2>, y_axis: Option<YAxis>) -> Vec<(NaiveDate, f32)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,f32) Tuples
    match y_axis {
        Some(YAxis::Distance) => { 
            data.iter().map(|item| (item.work_date.date_naive(), item.distance as f32)).collect()
        },
        Some(YAxis::StrokeRate) => { 
            data.iter().map(|item| (item.work_date.date_naive(), item.stroke_rate as f32)).collect()
        },
        Some(YAxis::StrokeCount) => {
            data.iter().map(|item| (item.work_date.date_naive(), item.stroke_count as f32)).collect()
        },
        Some(YAxis::Watts) => {
            data.iter().map(|item| (item.work_date.date_naive(), item.watts as f32)).collect()
        },
        _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
    }
}

fn convert_data_to_points_duration(data: Vec<Concept2>, y_axis: Option<YAxis>) -> Vec<(NaiveDate, Duration)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,Duration) Tuples
    match y_axis {
        Some(YAxis::Duration) => { 
            data.iter().map(|item| (item.work_date.date_naive(), item.duration)).collect()
        },
        Some(YAxis::Pace) => {
            data.iter().map(|item| (item.work_date.date_naive(), item.pace )).collect()
        },
        _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
    }
}