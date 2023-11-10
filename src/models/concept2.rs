use chrono::{DateTime, Local, NaiveTime};

#[derive(Debug, Clone)]
pub struct Concept2 {
    pub log_id: i32,
    pub work_date: DateTime<Local>,
    pub name: String,
    pub duration_sec: i32, 
    pub distance: i32, 
    pub stroke_rate: i32, 
    pub stroke_count: i32, 
    pub pace: NaiveTime, 
    pub watts: i32
}

impl Concept2 {
    pub fn create(log_id: i32, work_date: DateTime<Local>, name: String, duration_sec: i32,
            distance: i32, stroke_rate: i32, stroke_count: i32, pace: NaiveTime, watts: i32) -> Concept2 {
        return Concept2 { 
            log_id,
            work_date,
            name,
            duration_sec,
            distance,
            stroke_rate,
            stroke_count,
            pace,
            watts
        };
    }
}