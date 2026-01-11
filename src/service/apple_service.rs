use std::fs::File;
use std::io::BufReader;
use chrono::NaiveDate;
use xml::reader::{EventReader, XmlEvent};

use crate::helper;
use crate::store;
use crate::dto::range::Range;
use crate::dto::date_time_field_enum::DateTimeField;
use crate::dto::apple_record_dto::AppleRecordDto;

pub async fn main() {
    let path: &str = &std::env::var("PLOTS_PATH").expect("PLOTS_PATH must be set.");

    let options: Vec<&str> = vec!["daily kcal burn", "daily walking distance (km)"];
    let answer: usize = helper::io_helper::ask_choice_question("What data should be displayed?", options.clone());

    let range: Range = helper::io_helper::ask_range();
    
    let display_coice: &str = options[answer - 1];
    let mut title: String = format!("Apple Data, {}", display_coice);
    if range.start.is_some() && range.end.is_some() {
        title = format!("{}: {} - {}", title, range.start.unwrap(), range.end.unwrap());
    }

    let destination = format!("{}/apple_{}.png", path, display_coice.replace(" ", "_"));
    let record_type = match answer {
        1 => "HKQuantityTypeIdentifierActiveEnergyBurned",
        2 => "HKQuantityTypeIdentifierDistanceWalkingRunning",
        _ => panic!("The option specified is not valid: {}", answer)
    };

    let mut source_options: Vec<String> = store::apple_store::get_sources_by_type(record_type).await;
    source_options.insert(0, "No".to_string());
    let answer: usize = helper::io_helper::ask_choice_question("Shoud the data be filtered by source?", source_options.iter().map(|a| a.as_str()).collect());

    let source_filter: Option<String> = if answer == 1 {
        None
    } else {
        Some(source_options[answer - 1].clone())
    };

    let datapoints: Vec<(NaiveDate, f32)> = store::apple_store::get_data_daily_sumvalue(record_type, range, source_filter).await;
    _ = helper::graph::graph_f32(destination, datapoints, title.as_str()).await;
}

pub async fn load_data() {
    let path: &str = &std::env::var("APPLE_PATH").expect("APPLE_PATH must be set.");

    for (name, file) in helper::files::search_new_files(path, ".xml").await {
        if name != "Export.xml" {
            continue;
        }

        let mut records: Vec<AppleRecordDto> = Vec::new();
        let file = BufReader::new(File::open(file.path()).unwrap());
        let mut parser = EventReader::new(file);
        
        let mut cnt = 0;
        loop {
            match parser.next() {
                Ok(xml_event) => {
                    match xml_event {
                        XmlEvent::StartElement { name, attributes, .. } => {
                            match name.local_name.as_str() {
                                "Record" => {
                                    let mut apple_record: AppleRecordDto = AppleRecordDto::new();
                                    for attribute in attributes {
                                        match attribute.name.local_name.as_str() {
                                            "type" => apple_record.record_type = Some(attribute.value),
                                            "sourceName" => apple_record.source_name = Some(attribute.value),
                                            "startDate" => apple_record.set_work_date_from_str(attribute.value.as_str(), DateTimeField::Start).unwrap(),
                                            "endDate" => apple_record.set_work_date_from_str(attribute.value.as_str(), DateTimeField::End).unwrap(),
                                            "value" => apple_record.value = Some(attribute.value),
                                            "unit" => apple_record.unit = Some(attribute.value),
                                            _ => {}
                                        }
                                    }
                                    records.push(apple_record);
                                    // println!("C M: {} / {}", records.len(), cnt);
                                },
                                "Workout" => {
                                    let mut apple_record: AppleRecordDto = AppleRecordDto::new();
                                    for attribute in attributes {
                                        match attribute.name.local_name.as_str() {
                                            "workoutActivityType" => apple_record.record_type = Some(attribute.value),
                                            "sourceName" => apple_record.source_name = Some(attribute.value),
                                            "startDate" => apple_record.set_work_date_from_str(attribute.value.as_str(), DateTimeField::Start).unwrap(),
                                            "endDate" => apple_record.set_work_date_from_str(attribute.value.as_str(), DateTimeField::End).unwrap(),
                                            "value" => apple_record.value = Some(attribute.value),
                                            "unit" => apple_record.unit = Some(attribute.value),
                                            _ => {}
                                        }
                                    }
                                    records.push(apple_record);
                                    // println!("C M: {} / {}", records.len(), cnt);
                                },
                                _ => {}
                            }
                            
                            // if cnt == 5 {
                            //     break;
                            // }
                            cnt += 1;
                        },
                        // XmlEvent::Characters(text) => {
                        //     // println!("Text: {}", text);
                        // },
                        // XmlEvent::EndElement { name } => {
                        //     // println!("End: {}", name.local_name);
                        // },
                        XmlEvent::EndDocument => break,
                        _ => {}
                    }
                },
                Err(_) => break
            }
        }

        println!("C Ende: {} / {}", records.len(), cnt);

        store::apple_store::add_apple_record_entries(records).await;
        // apple_db::add_apple_workout_entries(all_workout_lines).await;

        println!("Loaded {}", name);
    }
}

