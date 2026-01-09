use chrono::NaiveDate;
use sqlx::{MySqlPool, Row, query};

use crate::database::db::*;
use crate::models::fddb::*;

pub async fn add_fddb_entries(all_values: Vec<Vec<&str>>) {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    if all_values.len() == 0 {
        return;
    }

    let mut query: String = String::from("INSERT INTO fddb (work_date, weight) VALUES ");
    
    for values in all_values {
        // convert the DE Date to a US Date
        let work_date: NaiveDate = match NaiveDate::parse_from_str(&values[0].replace("\"", ""), "%d.%m.%Y") {
            Ok(date) => date,
            Err(e) => panic!("The Date specified is either invalid or in the wrong format: {}\n{}", values[0].replace("\"", ""), e)
        };

        query += &format!("('{}', '{}'), ", work_date.format("%y-%m-%d").to_string(), values[1].replace("\"", ""));
    }

    query.truncate(query.len() - 2);
    query += " AS NEW ON DUPLICATE KEY UPDATE work_date = NEW.work_date;";

    println!("{}", query);

    let a = sqlx::query(&query)
        .execute(&pool).await;
    println!("{:?}", a);
}

pub async fn get_fddb_data(range_start: Option<NaiveDate>, range_end: Option<NaiveDate>) -> Vec<Fddb> {
    let mut all_data: Vec<Fddb> = Vec::new();
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };

    let mut query = String::from("SELECT * FROM fddb");
    match range_start {
        Some(date) => query = format!("{} WHERE work_date >= '{}'", query, date.format("%Y-%m-%d").to_string()),
        None => {}
    };
    match range_end {
        Some(date) => {
            if query.contains("WHERE") {
                query = format!("{} AND work_date <= '{}'", query, date.format("%Y-%m-%d").to_string())
            } else {
                query = format!("{} WHERE work_date <= '{}'", query, date.format("%Y-%m-%d").to_string())
            }
        },
        None => {}
    };

    let rows_opt = sqlx::query(&query)
        .fetch_all(&pool)
        .await;

    match rows_opt {
        Ok(rows) => {
            for row in rows {
                let work_date: NaiveDate = row.get("work_date");
                let weight: f32 = row.get("weight");

                all_data.push(Fddb::create(work_date, weight));
            }
        },
        Err(e) => { panic!("Error: {}", e); }
    }
    return all_data;
}

#[allow(dead_code)]
pub async fn reset_fddb() {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    _ = sqlx::query("delete from fddb")
        .execute(&pool)
        .await;
}