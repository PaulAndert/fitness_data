use std::fs;
use chrono::{Local, DateTime, NaiveDate};
use plotters::prelude::*;
use dotenv::dotenv;

use crate::common;
use crate::args;
use crate::args::YAxis;
use crate::database::concept2_db;
use crate::database::db;
use crate::models::concept2::Concept2;

pub async fn main(args: args::Args) {
    load_data().await;
    match args.sport {
        Some(args::Sport::Rowing) => {
            match args.workout {
                // TODO: min1
                Some(args::Workout::Min1) => { },
                Some(args::Workout::Min10) => {
                    _ = plot_workout("10:00 row", args.y_axis).await;
                },
                Some(args::Workout::Min15) => {
                    _ = plot_workout("15:00 row", args.y_axis).await;
                },
                // TODO: meter1000
                Some(args::Workout::Meter1000) => { },
                // TODO: meter2000
                Some(args::Workout::Meter2000) => { },
                Some(args::Workout::Meter5000) => { 
                    //TODO : change name to meter5k, also other meter types
                    _ = plot_workout("5000m row", args.y_axis).await;
                },
                None => {
                    panic!("Error: Unknown Workout");
                }
            }
        },
        _ => {
            panic!("Error: Unknown Sport");
        }
    }
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
        for value in line.split(",") {
            values.push(value);
        }
        if values.len() > 1 {
            concept2_db::add_concept2_entry(values).await;
        }
    }
}

async fn plot_workout(workout: &str, y_axis: Option<YAxis>) -> Result<(), Box<dyn std::error::Error>> {
    // Creates a Graph with the Data of the given Workout
    let data = concept2_db::get_concept2_workouts(workout).await;

    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points(data, y_axis.clone());
    if datapoints.len() < 1 {
        panic!("Error: No Datapoints for that Workout");
    }

    let (x_low, x_high) = common::get_x_low_high(datapoints.iter().map(|item| item.0).collect());
    let (y_low, y_high) = common::get_y_low_high(datapoints.iter().map(|item| item.1).collect());
    let title = get_title(workout, y_axis);

    let path = format!("plots/concept2_{}.png", workout.replace(" ", "_"));
    let root = BitMapBackend::new(&path, (2000, 750)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_low..x_high, y_low..y_high)?;
    chart.configure_mesh().light_line_style(&WHITE).x_label_formatter(&|x| x.to_string()).draw()?;

    chart.draw_series(LineSeries::new(
        datapoints.clone(),
        &RED,
    ))?;
    chart.draw_series(PointSeries::of_element(
        datapoints,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new(format!("{:.2}", c.1), (10, 0), ("sans-serif", 25).into_font());
        },
    ))?;
    root.present()?;

    Ok(())
}

fn get_title(workout: &str, y_axis: Option<YAxis>) -> &str {
    match workout {
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
        _ => { 
            "Error: Unknown Workout"
        }
    }
}

fn convert_data_to_points(data: Vec<Concept2>, y_axis: Option<YAxis>) -> Vec<(NaiveDate, f32)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,f32) Tuples
    match y_axis {
        Some(YAxis::Duration) => { 
            data.iter().map(|item| (item.work_date.date_naive(), sec_to_min(item.duration_sec))).collect()
        },
        Some(YAxis::Distance) => { 
            data.iter().map(|item| (item.work_date.date_naive(), item.distance as f32)).collect()
        },
        Some(YAxis::StrokeRate) => { 
            data.iter().map(|item| (item.work_date.date_naive(), item.stroke_rate as f32)).collect()
        },
        Some(YAxis::StrokeCount) => {
            data.iter().map(|item| (item.work_date.date_naive(), item.stroke_count as f32)).collect()
        },
        Some(YAxis::Pace) => {
            data.iter().map(|item| (item.work_date.date_naive(), sec_to_min(item.pace_sec))).collect()
        },
        Some(YAxis::Watts) => {
            data.iter().map(|item| (item.work_date.date_naive(), item.watts as f32)).collect()
        },
        _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
    }
}

fn sec_to_min(sec: f32) -> f32 {
    ((sec / 60.0) as i32) as f32 + (sec % 60.0) / 100.0
}