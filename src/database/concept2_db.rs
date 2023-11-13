use chrono::{Local, DateTime, NaiveTime};
use sqlx::{MySqlPool, Row};

use crate::database::db::*;
use crate::models::concept2::*;

pub async fn add_concept2_entry(values: Vec<&str>) {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    let sql = "select log_id from concept2 where log_id = ?";
    let log_id = sqlx::query(sql)
        .bind(values[0])
        .fetch_optional(&pool)
        .await;
    let sql = match log_id {
        Ok(Some(id)) => {
            let aa: i32 = id.get(0);
            println!("ID: {}", aa);
            // update ??
            "update concept2 set work_date = ?, name = ?, duration_sec = ?, distance = ?, stroke_rate = ?, stroke_count = ?, pace_sec = ?, watts = ? where log_id = ?"
        },
        Ok(None) => {
            // insert
            "insert into concept2 (work_date, name, duration_sec, distance, stroke_rate, stroke_count, pace_sec, watts, log_id) values (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        },
        Err(e) => {
            // err
            panic!("Error: {}", e);
        }
    };
    _ = sqlx::query(sql)
        .bind(values[1].replace("\"", ""))
        .bind(values[2].replace("\"", ""))
        .bind(values[4])
        .bind(values[7])
        .bind(values[9])
        .bind(values[10])
        .bind(convert_pace(values[11]))
        .bind(values[12])
        .bind(values[0]) // like that so that both insert and update work
        .execute(&pool).await;
}

fn convert_pace(pace: &str) -> f32 {
    let pace_split: Vec<&str> = pace.split(":").collect();
    let minutes: f32 = pace_split[0].parse::<f32>().unwrap();
    let seconds: f32 = pace_split[1].parse::<f32>().unwrap();
    minutes * 60.0 + seconds
}

pub async fn get_concept2_workouts(workout: &str) -> Vec<Concept2> {
    let mut all_workouts: Vec<Concept2> = Vec::new();
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };

    let rows_opt = sqlx::query("select * from concept2 where name = ?")
        .bind(workout)
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

                all_workouts.push(Concept2::create(log_id, work_date, name, duration_sec, distance, stroke_rate, stroke_count, pace_sec, watts));
            }
        },
        Err(e) => { panic!("Error: {}", e); }
    }
    return all_workouts;
}

#[allow(dead_code)]
pub async fn reset_concept2() {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    _ = sqlx::query("delete from concept2")
        .execute(&pool)
        .await;
}