use std::fs;
use chrono::{Local, DateTime, NaiveDate};
use plotters::prelude::*;
use dotenv::dotenv;

use crate::database::db;
use crate::database::fddb_db;
use crate::models::fddb::Fddb;
use crate::common;

pub async fn main() {
    load_data().await;
    _ = plot_graph().await;
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

async fn plot_graph() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<Fddb> = fddb_db::get_fddb_data().await;

    let datapoints: Vec<(NaiveDate, f32)> = convert_data_to_points(data);
    if datapoints.len() < 1 {
        panic!("Error: No Datapoints for that Workout");
    }
    let (x_low, x_high) = common::get_x_low_high(datapoints.iter().map(|item| item.0).collect());
    let (y_low, y_high) = common::get_y_low_high(datapoints.iter().map(|item| item.1).collect());

    let root = BitMapBackend::new(&"plots/fddb_weight.png", (2000, 750)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Weight Data", ("sans-serif", 50).into_font())
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
            + Text::new("", (10, 0), ("sans-serif", 25).into_font());
        },
    ))?;
    //TODO: mark highest and lowest points in graph with text value, and maybe other note-worthy metrics
    root.present()?;

    Ok(())
}

fn convert_data_to_points(data: Vec<Fddb>) -> Vec<(NaiveDate, f32)> {
    data.iter().map(|item| (item.work_date, item.weight as f32)).collect()
}