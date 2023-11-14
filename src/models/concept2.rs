use chrono::{DateTime, Local, Duration};

#[derive(Debug, Clone)]
pub struct Concept2 {
    pub log_id: i32,
    pub work_date: DateTime<Local>,
    pub name: String,
    pub duration: Duration, 
    pub distance: i32, 
    pub stroke_rate: i32, 
    pub stroke_count: i32, 
    pub pace: Duration, 
    pub watts: i32
}

impl Concept2 {
    pub fn create(log_id: i32, work_date: DateTime<Local>, name: String, duration_sec: f32,
            distance: i32, stroke_rate: i32, stroke_count: i32, pace_sec: f32, watts: i32) -> Concept2 {

        let pace = Duration::seconds(pace_sec as i64);
        pace.checked_add(&Duration::milliseconds((pace_sec * 100.0 % 100.0) as i64)).unwrap();

        let duration = Duration::seconds(duration_sec as i64);
        duration.checked_add(&Duration::milliseconds((duration_sec * 100.0 % 100.0) as i64)).unwrap();
        
        return Concept2 { 
            log_id,
            work_date,
            name,
            duration,
            distance,
            stroke_rate,
            stroke_count,
            pace,
            watts
        };
    }
}