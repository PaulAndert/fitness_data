use chrono::NaiveDate;
use sqlx::{MySqlPool, Row};

use crate::database::db::*;
use crate::models::fddb::*;

pub async fn add_fddb_entry(values: Vec<&str>) {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    // convert the DE Date to a US Date
    let de_date = values[0].replace("\"", "");
    let date_split: Vec<&str> = de_date.split(".").collect();
    let us_date = format!("{}-{}-{}", date_split[2], date_split[1], date_split[0]);

    let sql = "select work_date from fddb where work_date = ?";
    let log_id = sqlx::query(sql)
        .bind(us_date.clone())
        .fetch_optional(&pool)
        .await;
    let sql = match log_id {
        Ok(Some(_)) => {
            // update
            "update fddb set weight = ? where work_date = ?"
        },
        Ok(None) => {
            // insert
            "insert into fddb (weight, work_date) values (?, ?)"
        },
        Err(e) => {
            // err
            panic!("Error: {}", e);
        }
    };
    _ = sqlx::query(sql)
        .bind(values[1].replace("\"", "").replace(",", ".")) // like that so that both insert and update work
        .bind(us_date)
        .execute(&pool).await;
}

pub async fn get_fddb_data() -> Vec<Fddb> {
    let mut all_data: Vec<Fddb> = Vec::new();
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };

    let rows_opt = sqlx::query("select * from fddb")
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