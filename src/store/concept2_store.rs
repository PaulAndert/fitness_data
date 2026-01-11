use chrono::{DateTime, Local, NaiveDate};
use sqlx::{MySqlPool, Row};

use crate::store;
use crate::dto::concept2_dto::Concept2Dto;

pub async fn add_concept2_entries(all_values: Vec<Vec<&str>>) {
    let pool: MySqlPool = match store::common_store::create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    if all_values.len() == 0 {
        return;
    }

    let mut query: String = String::from("INSERT INTO concept2 (log_id, work_date, name, duration_sec, distance, stroke_rate, stroke_count, pace_sec, watts) VALUES ");

    for values in all_values {
        query += &format!("({}, '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}'), ", values[0], values[1].replace("\"", ""), values[2].replace("\"", ""), 
                                                                                    values[4], values[7], values[9],
                                                                                    values[10], convert_pace(values[11]), values[12]);
    }
    
    query.truncate(query.len() - 2);
    query += " AS NEW ON DUPLICATE KEY UPDATE work_date = NEW.work_date;";

    _ = sqlx::query(&query)
        .execute(&pool).await;
}

fn convert_pace(pace: &str) -> f32 {
    let pace_split: Vec<&str> = pace.split(":").collect();
    let minutes: f32 = pace_split[0].parse::<f32>().unwrap();
    let seconds: f32 = pace_split[1].parse::<f32>().unwrap();
    minutes * 60.0 + seconds
}

pub async fn get_concept2_data(workout: &str, range_start: Option<NaiveDate>, range_end: Option<NaiveDate>) -> Vec<Concept2Dto> {
    let mut all_data: Vec<Concept2Dto> = Vec::new();
    let pool: MySqlPool = match store::common_store::create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };

    let mut query = format!("SELECT * FROM concept2 WHERE name = '{}'", workout);
    match range_start {
        Some(date) => query = format!("{} AND work_date >= '{} 00:00:00'", query, date.format("%Y-%m-%d").to_string()),
        None => {}
    };
    match range_end {
        Some(date) => query = format!("{} AND work_date <= '{} 00:00:00'", query, date.format("%Y-%m-%d").to_string()),
        None => {}
    };
    query += " ORDER BY work_date ASC;";

    let rows_opt = sqlx::query(&query)
        .fetch_all(&pool)
        .await;

    match rows_opt {
        Ok(rows) => {
            for row in rows {
                let log_id: i32 = row.get("log_id");
                let work_date: DateTime<Local> = row.get("work_date");
                let name: String = row.get("name");

                let duration_sec: f32 = row.get("duration_sec");
                let distance: i32 = row.get("distance");
                let stroke_rate: i32 = row.get("stroke_rate");

                let stroke_count: i32 = row.get("stroke_count");
                let pace_sec: f32 = row.get("pace_sec");
                let watts: i32 = row.get("watts");

                all_data.push(Concept2Dto::create(log_id, work_date, name, duration_sec, distance, stroke_rate, stroke_count, pace_sec, watts));
            }
        },
        Err(e) => { panic!("Error: {}", e); }
    }
    return all_data;
}

#[allow(dead_code)]
pub async fn reset_concept2() {
    let pool: MySqlPool = match store::common_store::create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    _ = sqlx::query("delete from concept2")
        .execute(&pool)
        .await;
}