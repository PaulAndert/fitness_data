

use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppleWorkoutWalkingDto {
    pub log_id: Option<i32>,
    pub workout_type: Option<String>,
    pub source_name: Option<String>,
    pub datetime_start: Option<DateTime<Local>>,
    pub datetime_end: Option<DateTime<Local>>,
    pub duration: Option<String>,
    pub unit: Option<String>,
}