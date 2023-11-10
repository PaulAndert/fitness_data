
use chrono::{DateTime, Local, NaiveDate, Utc, Duration};
use dotenv::dotenv;
use sqlx::{Row, FromRow};
use sqlx::mysql::{ MySqlPool, MySqlRow };
use std::error::Error;

// pub fn parse_to_db(source_type: args::Source, file: String) {

// }

pub async fn create_pool() -> Result<MySqlPool, Box<dyn Error>>{
    dotenv().ok();
    let url: &str = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool: MySqlPool = MySqlPool::connect(url)
        .await?;
    return Ok(pool);
}

// checks if file already exists, if not, insert it, then check if last_modified value got changed
pub async fn is_db_up_to_date(name: &str, last_modified: DateTime<Local>) -> Result<bool, Box<dyn Error>>{
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    let res: Option<MySqlRow> = sqlx::query(&format!("select last_modified from known_files where name = '{}'", name))
        .fetch_optional(&pool)
        .await?;
    match res {
        Some(row) => {
            let db_lm: DateTime<Utc> = row.get("last_modified");
            // add a second because of floating point in last_modified
            if db_lm + Duration::seconds(1) > last_modified {
                // db is up to date
                return Ok(false);
            }else {
                // db is NOT up to date
                return Ok(true);
            }
        },
        None => { 
            _ = add_known_file(pool, name, last_modified).await;
            // db is NOT up to date
            Ok(true)
        }
    }
}

async fn add_known_file(pool: MySqlPool, name: &str, last_modified: DateTime<Local>) -> Result<(), Box<dyn Error>> {
    let sql = "insert into known_files (name, last_modified) values (?, ?)";
    let _ = sqlx::query(sql)
        .bind(&name)
        .bind(&last_modified.format("%Y-%m-%d %H:%M:%S").to_string())
        .execute(&pool).await;
    Ok(())
}

pub async fn reset_known_files() {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    _ = sqlx::query("delete from known_files where id > 0")
        .execute(&pool)
        .await;
}

pub async fn add_concept2_entry(values: Vec<&str>) {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    let sql = "insert into concept2 (log_id, work_date, name, duration_sec, distance, stroke_rate, stroke_count, pace, watts) values (?, ?, ?, ?, ?, ?, ?, ?, ?)";
    let aa = sqlx::query(sql)
        .bind(values[0])
        .bind(values[1].replace("\"", ""))
        .bind(values[2])
        .bind(values[4].split(".").next())
        .bind(values[7])
        .bind(values[9])
        .bind(values[10])
        .bind(values[11].split(".").next())
        .bind(values[12])
        .execute(&pool).await;

    println!("{:?}", aa);
}