use std::{fs, path::PathBuf};
use chrono::Utc;
use chrono::{DateTime, NaiveDate};
use plotters::prelude::*;
use dotenv::dotenv;

use crate::args;
use crate::db;

pub fn main(args: args::Args) {
    load_data();
    match args.sport {
        Some(args::Sport::Rowing) => {
            match args.workout {
                Some(args::Workout::Min1) => { },
                Some(args::Workout::Min10) => { },
                Some(args::Workout::Min15) => { },
                Some(args::Workout::Meter1000) => { },
                Some(args::Workout::Meter2000) => { },
                Some(args::Workout::Meter5000) => { },
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

fn load_data() {
    dotenv().ok();
    let path: &str = &std::env::var("CONCEPT2_PATH").expect("CONCEPT2_PATH must be set.");
    println!("{}", path);
    let dir_list = fs::read_dir(path).unwrap();
    for file_res in dir_list {
        match file_res {
            Ok(file) => {
                match file.metadata() {
                    Ok(metadata) => {
                        match metadata.modified() {
                            Ok(systime) => {
                                let last_modified: DateTime<Utc> = systime.clone().into();
                                
                            },
                            Err(e)=> {
                                println!("{:?}", e);
                            }
                        }
                    }
                    Err(e)=> {
                        println!("{:?}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            },
        }
    }
    //db::parse_to_db();
}

// //"Log ID",Date,Description,"Work Time (Formatted)","Work Time (Seconds)","Rest Time (Formatted)","Rest Time (Seconds)","Work Distance","Rest Distance","Stroke Rate/Cadence","Stroke Count",Pace,"Avg Watts",Cal/Hour,"Total Cal","Avg Heart Rate","Drag Factor",Age,Weight,Type,Ranked,Comments,"Date Entered"

// fn process_content(contents: String, mut liste: Vec<Point>) -> Vec<Point>{
//     let mut splits: Vec<&str> = contents.split("\n").collect();
    
//     let names: Vec<&str> = splits[0].split(",").collect();
//     splits.remove(0);
    
//     for split in splits {
//         let data: Vec<&str> = split.split(",").collect();
//         if data.len() == names.len() {
//             liste.push(Point{ 
//                 name: format!("{}", data[2]),
//                 date: format!("{}", data[1]),
//                 time: format!("{}", data[4]),
//                 distance: format!("{}", data[7]),
//                 stroke_rate: format!("{}", data[9]),
//                 stroke_count: format!("{}", data[10]),
//                 pace: format!("{}", data[11]),
//                 watts: format!("{}", data[12]),
//             });
//         }
//     }
//     return liste;
// }

// fn create_file(liste: Vec<(NaiveDate, f32)>, input: &str) -> Result<(), Box<dyn std::error::Error>> {

//     let mut x_lowest: NaiveDate = liste[0].0;
//     let mut x_highest: NaiveDate = liste[0].0;
//     for item in liste.clone() {
//         if item.0 < x_lowest {
//             x_lowest = item.0;
//         }else if item.0 > x_highest {
//             x_highest = item.0
//         }
//     }
//     x_lowest = x_lowest.checked_sub_days(chrono::Days::new(3)).unwrap();
//     x_highest = x_highest.checked_add_days(chrono::Days::new(3)).unwrap();
//     // let x_lowest: NaiveDate = get_date("2023-04-01 00:00:00 +0100").unwrap();
//     // let x_highest: NaiveDate = get_date("2023-11-01 00:00:00 +0100").unwrap();

//     let y_lowest: f32;
//     let y_highest: f32;
//     let title: &str;
//     match input {
//         "5000m" => {
//             title = "Rowing 5000 meter Times";
//             y_lowest = 22.0;
//             y_highest = 24.0;
//         },
//         "15min" => {
//             title = "Rowing 15 min Distances";
//             y_lowest = 2000.0;
//             y_highest = 4000.0;
//         },
//         "10min" => {
//             title = "Rowing 10 min Distances";
//             y_lowest = 1800.0;
//             y_highest = 2400.0;
//         },
//         _ => { 
//             title = ""; 
//             y_lowest = 0.0;
//             y_highest = 1.0;
//         }
//     };

//     let root = BitMapBackend::new("plots/rowing_2.png", (2000, 750)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .caption(title, ("sans-serif", 50).into_font())
//         .margin(15)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(x_lowest..x_highest, y_lowest..y_highest)?;
//     chart.configure_mesh().light_line_style(&WHITE).x_label_formatter(&|x| x.to_string()).draw()?;

//     chart.draw_series(LineSeries::new(
//         liste.clone(),
//         &RED,
//     ))?;
//     chart.draw_series(PointSeries::of_element(
//         liste,
//         5,
//         &RED,
//         &|c, s, st| {
//             return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
//             + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
//             + Text::new(format!("{}", c.1), (10, 0), ("sans-serif", 10).into_font());
//         },
//     ))?;
//     root.present()?;

//     Ok(())
// }

// pub fn create_row_graph(path: &str, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    
//     let paths = fs::read_dir(path).unwrap();
//     let mut liste: Vec<Point> = Vec::new();
//     for path in paths {
//         match path {
//             Ok(file) => {
//                 let contents = fs::read_to_string(file.path()).expect("Should have been able to read the file");
//                 liste = process_content(contents, liste);
//             },
//             Err(e) => {
//                 eprintln!("Error: {}", e);
//             },
//         }
//     }

//     let tuple_liste = create_tuple_liste(liste, input);
//     _ = create_file(tuple_liste, input);

//     Ok(())
// }

// fn create_tuple_liste(liste: Vec<Point>, input: &str) -> Vec<(NaiveDate, f32)> {
//     let mut ret: Vec<(NaiveDate, f32)> = Vec::new();

//     match input {
//         "5000m" => {
//             for item in liste {
//                 //println!("{}", item.name.as_str());
//                 if item.name.as_str() == "\"5000m row\"" {
//                     ret.push((get_date(&format!("{} +0100", item.date.replace('"', ""))).unwrap(), item.time.parse::<f32>().unwrap() / 60.0));
//                 }
//             }
//         },
//         "15min" => {

//         },
//         "10min" => {
//             for item in liste {
//                 //println!("{}", item.name.as_str());
//                 if item.name.as_str() == "\"10:00 row\"" {
//                     ret.push((get_date(&format!("{} +0100", item.date.replace('"', ""))).unwrap(), item.distance.parse::<f32>().unwrap()));
//                 }
//             }
//         },
//         _ => {}
//     }
//     return ret;
// }

// fn get_date (datetime: &str) -> Option<NaiveDate> {
//     match DateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S %z"){
//         Ok(date_time) => {
//             return Some(date_time.date_naive());
//         }, 
//         Err(error) => {
//             println!("A {}", datetime);
//             println!("E {}", error);
//             return None;
//         },
//     };
// }