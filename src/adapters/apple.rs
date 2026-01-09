// use std::fs::File;
// use std::io::BufReader;
// use xml::{reader::{EventReader, XmlEvent}, attribute::OwnedAttribute};
// use plotters::prelude::*;
// use chrono::{DateTime, NaiveDate};

use crate::helper::args;

pub async fn main() {
    // load_data();
    // match args.sport {
    //     Some(args::Sport::Rowing) => {
    //         //rowing(args);
    //     },
    //     Some(args::Sport::Walking) => { },
    //     None => {
    //         panic!("Error: Unknown Sport");
    //     }
    // }
}

// fn load_data() {
//     // put data into DB
// }

// fn rowing(args: args::Args) {
//     match args.workout {
//         Some(args::Workout::Min1) => { },
//         Some(args::Workout::Min10) => { },
//         Some(args::Workout::Min15) => { },
//         Some(args::Workout::Meter1000) => { },
//         Some(args::Workout::Meter2000) => { },
//         Some(args::Workout::Meter5000) => { },
//         None => {
//             panic!("Error: Unknown Workout");
//         }
//     }
// }




// fn get_date (datetime: &str) -> Option<NaiveDate> {
//     match DateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S %z"){
//         Ok(date_time) => {
//             return Some(date_time.date_naive());
//         }, 
//         Err(error) => {
//             println!("E {}", error);
//             return None;
//         },
//     };
// }

// fn record(attributes: Vec<OwnedAttribute>) {
//     if attributes[0].name.local_name != String::from("type") {
//         println!("{} != type", attributes[0].name);
//         return ;
//     }
//     match attributes[0].value.as_str() {
//         // what type is it
//         "HKQuantityTypeIdentifierHeight" => {
//         }
//         "HKQuantityTypeIdentifierStepCount" => {
//         },
//         "HKQuantityTypeIdentifierDistanceWalkingRunning" => {
//         },
//         "HKQuantityTypeIdentifierBasalEnergyBurned" => {
//         },
//         "HKQuantityTypeIdentifierActiveEnergyBurned" => {
//         },
//         "HKQuantityTypeIdentifierFlightsClimbed" => {
//         },
//         "HKQuantityTypeIdentifierAppleExerciseTime" => {
//         },
//         "HKQuantityTypeIdentifierRestingHeartRate" => {
//         },
//         "HKQuantityTypeIdentifierVO2Max" => {
//         },
//         "HKQuantityTypeIdentifierWalkingHeartRateAverage" => {
//         },
//         "HKQuantityTypeIdentifierEnvironmentalAudioExposure" => {
//         },
//         "HKQuantityTypeIdentifierHeadphoneAudioExposure" => {
//         },
//         "HKQuantityTypeIdentifierWalkingDoubleSupportPercentage" => {
//         },
//         "HKQuantityTypeIdentifierSixMinuteWalkTestDistance" => {
//         },
//         "HKQuantityTypeIdentifierAppleStandTime" => {
//         },
//         "HKQuantityTypeIdentifierWalkingSpeed" => {
//         },
//         "HKQuantityTypeIdentifierWalkingStepLength" => {
//         },
//         "HKQuantityTypeIdentifierWalkingAsymmetryPercentage" => {
//         },
//         "HKQuantityTypeIdentifierStairAscentSpeed" => {
//         },
//         "HKQuantityTypeIdentifierStairDescentSpeed" => {
//         },
//         "HKQuantityTypeIdentifierAppleWalkingSteadiness" => {
//         },
//         "HKQuantityTypeIdentifierRunningStrideLength" => {
//         },
//         "HKQuantityTypeIdentifierRunningVerticalOscillation" => {
//         },
//         "HKQuantityTypeIdentifierRunningGroundContactTime" => {
//         },
//         "HKQuantityTypeIdentifierHeartRateRecoveryOneMinute" => {
//         },
//         "HKQuantityTypeIdentifierRunningPower" => {
//         },
//         "HKQuantityTypeIdentifierRunningSpeed" => {
//         },
//         "HKQuantityTypeIdentifierPhysicalEffort" => {
//         },
//         "HKCategoryTypeIdentifierAppleStandHour" => {
//         },
//         "HKCategoryTypeIdentifierAudioExposureEvent" => {
//         },
//         "HKCategoryTypeIdentifierHeadphoneAudioExposureEvent" => {
//         },
//         ////////////////////////
//         "HKQuantityTypeIdentifierHeartRateVariabilitySDNN" => {
//         },
//         _ => {}
//     }
// }

// fn workout(attributes: Vec<OwnedAttribute>, mut liste: Vec<(NaiveDate, f32)>) -> Vec<(NaiveDate, f32)>{
//     if attributes[0].name.local_name != String::from("workoutActivityType") {
//         println!("{} != workoutActivityType", attributes[0].name);
//         return liste;
//     }
//     match attributes[0].value.as_str() {
//         // what type is it
//         "HKWorkoutActivityTypeWalking" => {
//         },
//         "HKWorkoutActivityTypeRowing" => {
//             let empty: String = String::from("");
//             let mut tuple: (String, String) = (empty.clone(), empty);
//             for attribute in attributes {
//                 match attribute.name.local_name.as_str() {
//                     "startDate" => {
//                         tuple.0 = attribute.value;
//                     },
//                     "duration" => {
//                         tuple.1 = attribute.value;
//                     },
//                     _ => {},
//                 }
//             }
//             // add 
//             liste.push((get_date(tuple.0.as_str()).unwrap(), tuple.1.parse::<f32>().unwrap()));
//             println!("{:?}", tuple);
//         },
//         "HKWorkoutActivityTypeTraditionalStrengthTraining" => {
//         },
//         "HKWorkoutActivityTypeRunning" => {
//         },
//         _ => {
//             println!("Ex {}", attributes[0].value);
//         },
//     }
//     return liste;
// }

// pub fn create_row_graph(path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let file: File = File::open(path)?;
//     let file: BufReader<File> = BufReader::new(file); 
//     let parser: EventReader<BufReader<File>> = EventReader::new(file);

//     let start = get_date("2023-04-05 00:00:00 +0100").unwrap();
//     let until = get_date("2023-10-30 00:00:00 +0100").unwrap();

//     let root = BitMapBackend::new("plots/rowing.png", (2000, 750)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Rowing Data", ("sans-serif", 50).into_font())
//         .margin(5)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(start..until, 1f32..25f32)?;
//     chart.configure_mesh().light_line_style(&WHITE).x_label_formatter(&|x| x.to_string()).draw()?;

//     let mut cnt: u32 = 0;
//     let mut liste: Vec<(NaiveDate, f32)> = Vec::new();
//     for e in parser {
//         match e {
//             Ok(XmlEvent::StartElement { name, attributes, .. }) => {
//                 match name.local_name.as_str() {
//                     "Record" => {
//                         record(attributes);
//                     },
//                     "Workout" => {
//                         liste = workout(attributes, liste);
//                     },
//                     "ActivitySummary" => {

//                     },

//                     _ => {},
//                 }
//             },
//             Ok(XmlEvent::Whitespace(_)) => {},
//             Err(e) => {
//                 eprintln!("Error: {e}");
//                 break;
//             },
//             _ => {}
//         }
//         cnt += 1;
//         if cnt % 100000 == 0 { println!("{}", cnt); }
//     }

//     chart.draw_series(LineSeries::new(
//         liste,
//         //vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
//         &RED,
//     ))?;
//     root.present()?;

//     Ok(())
// }