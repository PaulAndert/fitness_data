use std::fs;
use chrono::NaiveTime;
use chrono::{Local, DateTime, NaiveDate};
use plotters::prelude::*;
use dotenv::dotenv;

use crate::args;
use crate::args::YAxis;
use crate::database::concept2_db::*;
use crate::database::db::*;
use crate::models::concept2::Concept2;

pub async fn main(args: args::Args) {
    // TODO: DEV tools
    // db::reset_known_files().await;
    // db::reset_concept2().await;
    load_data().await;
    match args.sport {
        Some(args::Sport::Rowing) => {
            match args.workout {
                Some(args::Workout::Min1) => { },
                Some(args::Workout::Min10) => {
                    _ = plot_workout("10:00 row", args.y_axis).await;
                },
                Some(args::Workout::Min15) => { },
                Some(args::Workout::Meter1000) => { },
                Some(args::Workout::Meter2000) => { },
                Some(args::Workout::Meter5000) => { 
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
        match is_db_up_to_date(&filename, last_modified).await {
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
            add_concept2_entry(values).await;
        }
    }
}

fn get_low_high(data: Vec<(NaiveDate, f32)>) -> (NaiveDate, NaiveDate) {
    // returns a tuple of the first and last Date from the given Vec of (Date,f32) tuples
    let mut x_lowest: NaiveDate = data[0].0;
    let mut x_highest: NaiveDate = data[0].0;
    for item in data {
        if item.0 < x_lowest {
            x_lowest = item.0;
        }else if item.0 > x_highest {
            x_highest = item.0;
        }
    }
    // this is so that the graph has some padding to the sides
    x_lowest = x_lowest.checked_sub_days(chrono::Days::new(3)).unwrap();
    x_highest = x_highest.checked_add_days(chrono::Days::new(3)).unwrap();

    return (x_lowest, x_highest);
}

async fn plot_workout(workout: &str, y_axis: Option<YAxis>) -> Result<(), Box<dyn std::error::Error>> {
    // Creates a Graph with the Data of the given Workout
    let data = get_concept2_workouts(workout).await;

    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points(data, y_axis.clone());
    if datapoints.len() < 1 {
        panic!("Error: No Datapoints for that Workout");
    }
    let (x_lowest, x_highest) = get_low_high(datapoints.clone());
    
    let (title, y_lowest, y_highest): (&str, f32, f32) = get_values(workout, y_axis);

    let path = format!("plots/concept2_{}.png", workout.replace(" ", "_"));
    let root = BitMapBackend::new(&path, (2000, 750)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_lowest..x_highest, y_lowest..y_highest)?;
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

fn get_values(workout: &str, y_axis: Option<YAxis>) -> (&str, f32, f32) {
    match workout {
        "5000m row" => {
            match y_axis {
                Some(YAxis::Duration) => { ("Rowing 5000 meter, Times in Minutes", 22.0, 24.0) },
                Some(YAxis::Distance) => { ("Rowing 5000 meter, Distance in Meter", 4900.0, 5100.0) },
                Some(YAxis::StrokeRate) => { ("Rowing 5000 meter, Avg. Strokes per Minute", 15.0, 20.0) },
                Some(YAxis::StrokeCount) => { ("Rowing 5000 meter, Strokecount", 350.0, 450.0) },
                Some(YAxis::Pace) => { ("Rowing 5000 meter, Avg. Pace (Minutes per 500m)", 2.0, 2.5) },
                Some(YAxis::Watts) => { ("Rowing 5000 meter, Avg. Watts", 100.0, 150.0) },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        "15:00 row" => {
            let title: &str = "Rowing 15 min, Distances in Meters";
            match y_axis {
                // Some(YAxis::Duration) => { (title, 22.0, 24.0) },
                Some(YAxis::Distance) => { (title, 2000.0, 4000.0) },
                // Some(YAxis::StrokeRate) => { (title, 22.0, 24.0) },
                // Some(YAxis::StrokeCount) => { (title, 22.0, 24.0) },
                // Some(YAxis::Pace) => { },
                // Some(YAxis::Watts) => { (title, 22.0, 24.0) },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        "10:00 row" => {
            let title: &str = "Rowing 10 min, Distances in Meters";
            match y_axis {
                // Some(YAxis::Duration) => { (title, 22.0, 24.0) },
                Some(YAxis::Distance) => { (title, 1800.0, 2400.0) },
                // Some(YAxis::StrokeRate) => { (title, 22.0, 24.0) },
                // Some(YAxis::StrokeCount) => { (title, 22.0, 24.0) },
                // Some(YAxis::Pace) => { },
                // Some(YAxis::Watts) => { (title, 22.0, 24.0) },
                _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
            }
        },
        _ => { 
            ("Error: Unknown Workout", 0.0, 1.0)
        }
    }
}


fn convert_data_to_points(data: Vec<Concept2>, y_axis: Option<YAxis>) -> Vec<(NaiveDate, f32)> {
    // Takes a Vec of Cencept2 Structs and returned a Vec of (Date,f32) Tuples
    match y_axis {
        Some(YAxis::Duration) => { 
            data.iter().map(|item| (item.work_date.date_naive(), item.duration_sec as f32 / 60.0)).collect()
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
            data.iter().map(|item| (item.work_date.date_naive(), convert_pace(item.pace))).collect()
        },
        Some(YAxis::Watts) => {
            data.iter().map(|item| (item.work_date.date_naive(), item.watts as f32)).collect()
        },
        _ => { panic!("Error: no y-axis specified. {:?}", y_axis); }
    }
}

fn convert_pace(pace: NaiveTime) -> f32 {
    // converts Pace from Naivetime to a f32 representation
    let binding: String = pace.format("%H:%M").to_string();
    let str_vec: Vec<&str> = binding.split(":").collect();

    if str_vec.len() == 2 {
        // all minutes normally and seconds with a division by 100 to put it after the comma
        return str_vec[0].parse::<f32>().unwrap() + str_vec[1].parse::<f32>().unwrap() / 100.0;
    }else {
        panic!("Error: Pace is weird: {}", pace);
    }
}