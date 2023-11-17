use std::fs;
use chrono::Duration;
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
                Some(args::Workout::Min1) => {
                    _ = plot_workout("1:00 row", args.y_axis).await;
                },
                Some(args::Workout::Min10) => {
                    _ = plot_workout("10:00 row", args.y_axis).await;
                },
                Some(args::Workout::Min15) => {
                    _ = plot_workout("15:00 row", args.y_axis).await;
                },
                Some(args::Workout::Meter1k) => {
                    _ = plot_workout("1000m row", args.y_axis).await;
                },
                Some(args::Workout::Meter2k) => {
                    _ = plot_workout("2000m row", args.y_axis).await;
                },
                Some(args::Workout::Meter5k) => { 
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

async fn plot_workout(workout: &str, y_axis: Option<YAxis>) {
    // Creates a Graph with the Data of the given Workout
    match y_axis {
        Some(YAxis::Duration) | Some(YAxis::Pace) => { 
            _ = graph_duration(workout, y_axis).await;
        },
        Some(YAxis::Distance) | Some(YAxis::StrokeRate) | 
        Some(YAxis::StrokeCount) | Some(YAxis::Watts) => {
            _ = graph_f32(workout, y_axis).await;
        },
        _ => { 
            panic!("Error: unknown y-axis");
        }
    }
}

async fn graph_duration(workout: &str, y_axis: Option<YAxis>) -> Result<(), Box<dyn std::error::Error>> {
    let data = concept2_db::get_concept2_workouts(workout).await;
    let datapoints: Vec<(NaiveDate, Duration)> = convert_data_to_points_duration(data, y_axis.clone());

    if datapoints.len() < 1 {
        panic!("Error: No Datapoints for that Workout");
    }

    let (x_low, x_high) = common::get_x_low_high(datapoints.iter().map(|item| item.0).collect());
    
    let (y_low_date, y_low_val): (NaiveDate, Duration) = common::get_y_low::<Duration>(datapoints.clone());
    let (y_high_date, y_high_val): (NaiveDate, Duration) = common::get_y_high::<Duration>(datapoints.clone());
    let y_low_bound = Duration::seconds((y_low_val.num_seconds() as f32 * 0.95) as i64);
    let y_high_bound = Duration::seconds((y_high_val.num_seconds() as f32 * 1.05) as i64);

    let title = get_title(workout, y_axis);

    let path = format!("plots/concept2_{}.png", workout.replace(" ", "_"));
    let root = BitMapBackend::new(&path, (2000, 750)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_low..x_high, y_low_bound..y_high_bound)?;
    chart.configure_mesh()
        .light_line_style(&WHITE)
        .y_label_formatter(&|y| format!("{:02}: {:02}. {}", y.num_minutes(), y.num_seconds() % 60, get_mili_string(y.num_milliseconds())))
        .x_label_formatter(&|x| x.format("%d.%m.%Y").to_string())
        .draw()?;

    // Draw Line
    chart.draw_series(LineSeries::new(
        datapoints.clone(),
        &RED,
    ))?;
    // Draw Points
    chart.draw_series(PointSeries::of_element(
        datapoints,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new("", (10, 0), ("sans-serif", 25).into_font());
        },
    ))?;

    // Draw highest Point
    chart.draw_series(PointSeries::of_element(
        vec![(y_high_date, y_high_val)],
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new(format!("{:02}: {:02}. {}", y_low_val.num_minutes(), y_low_val.num_seconds() % 60, get_mili_string(y_low_val.num_milliseconds())), (0, -25), ("sans-serif", 25).into_font());
        },
    ))?;
    // Draw lowest Point
    chart.draw_series(PointSeries::of_element(
        vec![(y_low_date, y_low_val)],
        5,
        &GREEN,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new(format!("{:02}: {:02}. {}", y_high_val.num_minutes(), y_high_val.num_seconds() % 60, get_mili_string(y_high_val.num_milliseconds())), (0, 10), ("sans-serif", 25).into_font());
        },
    ))?;  
    root.present()?;
    Ok(())
}

async fn graph_f32(workout: &str, y_axis: Option<YAxis>) -> Result<(), Box<dyn std::error::Error>> {
    let data = concept2_db::get_concept2_workouts(workout).await;
    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points_f32(data, y_axis.clone());

    if datapoints.len() < 1 {
        panic!("Error: No Datapoints for that Workout");
    }

    let (x_low, x_high) = common::get_x_low_high(datapoints.iter().map(|item| item.0).collect());

    let (y_low_date, y_low_val): (NaiveDate, f32) = common::get_y_low::<f32>(datapoints.clone());
    let (y_high_date, y_high_val): (NaiveDate, f32) = common::get_y_high::<f32>(datapoints.clone());

    let title = get_title(workout, y_axis);

    let path = format!("plots/concept2_{}.png", workout.replace(" ", "_"));
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new(&path, (2000, 750)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_low..x_high, (y_low_val * 0.95)..(y_high_val * 1.05))?;
    chart.configure_mesh()
        .light_line_style(&WHITE)
        .y_label_formatter(&|y| y.to_string())
        .x_label_formatter(&|x| x.format("%d.%m.%Y").to_string())
        .draw()?;

    // Draw Line
    chart.draw_series(LineSeries::new(
        datapoints.clone(),
        &RED,
    ))?;
    // Draw Points
    chart.draw_series(PointSeries::of_element(
        datapoints,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new("", (10, 0), ("sans-serif", 25).into_font());
        },
    ))?;

    // Draw highest Point
    chart.draw_series(PointSeries::of_element(
        vec![(y_high_date, y_high_val)],
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new(format!("{:.2}", y_high_val), (0, -25), ("sans-serif", 25).into_font());
        },
    ))?;
    // Draw lowest Point
    chart.draw_series(PointSeries::of_element(
        vec![(y_low_date, y_low_val)],
        5,
        &GREEN,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new(format!("{:.2}", y_low_val), (0, 10), ("sans-serif", 25).into_font());
        },
    ))?;  
    root.present()?;
    Ok(())
}

fn get_mili_string(mili: i64) -> String {
    let display_milis: i64 = mili % 3600;
    let mut string_milis: String = format!("{}", display_milis);
    match string_milis.len() {
        len if len == 0 => {
            String::from("00")
        },
        len if len == 1 => {
            format!("{}0", string_milis)
        },
        len if len == 2 => {
            string_milis
        },
        len if len > 2 => {
            string_milis.drain(0..2).collect()
        },
        _ => {
            panic!("Error");
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