use chrono::{Duration, NaiveDate};
use plotters::prelude::*;

use crate::helper;

pub async fn graph_duration(destination: String, datapoints: Vec<(NaiveDate, Duration)>, title: &str) -> Result<(), Box<dyn std::error::Error>> {

    if datapoints.len() < 1 {
        panic!("Error: No Datapoints for that Workout");
    }

    let (x_low, x_high) = helper::common::get_x_low_high(datapoints.iter().map(|item| item.0).collect());
    
    let (y_low_date, y_low_val): (NaiveDate, Duration) = helper::common::get_y_low::<Duration>(datapoints.clone());
    let (y_high_date, y_high_val): (NaiveDate, Duration) = helper::common::get_y_high::<Duration>(datapoints.clone());
    
    // Add Padding to the Graph
    let y_low_bound = Duration::seconds((y_low_val.num_seconds() as f32 * 0.95) as i64);
    let y_high_bound = Duration::seconds((y_high_val.num_seconds() as f32 * 1.05) as i64);

    let root = BitMapBackend::new(&destination, (2000, 750)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(30)
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
            + Text::new(format!("{:02}:{:02}.{}", y_high_val.num_minutes(), y_high_val.num_seconds() % 60, get_mili_string(y_high_val.num_milliseconds())), (0, -25), ("sans-serif", 25).into_font());
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
            + Text::new(format!("{:02}:{:02}.{}", y_low_val.num_minutes(), y_low_val.num_seconds() % 60, get_mili_string(y_low_val.num_milliseconds())), (0, 10), ("sans-serif", 25).into_font());
        },
    ))?;  
    root.present()?;
    Ok(())
}

pub async fn graph_f32(destination: String, datapoints: Vec<(NaiveDate, f32)>, title: &str) -> Result<(), Box<dyn std::error::Error>> {

    if datapoints.len() < 1 {
        panic!("Error: No Datapoints for that Workout");
    }

    let (x_low, x_high): (NaiveDate, NaiveDate) = helper::common::get_x_low_high(datapoints.iter().map(|item| item.0).collect());

    let (y_low_date, y_low_val): (NaiveDate, f32) = helper::common::get_y_low::<f32>(datapoints.clone());
    let (y_high_date, y_high_val): (NaiveDate, f32) = helper::common::get_y_high::<f32>(datapoints.clone());

    // Add Padding to the Graph
    let y_low_bound: f32 = y_low_val * 0.95;
    let y_high_bound: f32 = y_high_val * 1.05;

    //let path = format!("plots/concept2_{}.png", workout.replace(" ", "_"));
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new(&destination, (2000, 750)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_low..x_high, y_low_bound..y_high_bound)?;
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