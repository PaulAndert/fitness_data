use chrono::NaiveDate;
use sqlx::{MySqlPool, Row};

use crate::dto::range::Range;
use crate::store;
use crate::dto::apple_record_dto::AppleRecordDto;

pub async fn get_data_daily_sumvalue(record_type: &str, range: Range, source_filter: Option<String>) -> Vec<(NaiveDate, f32)> {

    let mut all_data: Vec<(NaiveDate, f32)> = Vec::new();
    let pool: MySqlPool = match store::common_store::create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };

    let mut query: String = format!("SELECT DATE(datetime_start) AS day, SUM(value) AS total_value FROM apple_records WHERE record_type = '{}'", record_type);
    match range.start {
        Some(date) => query = format!("{} AND datetime_start >= '{} 00:00:00'", query, date.format("%Y-%m-%d").to_string()),
        None => {}
    };
    match range.end {
        Some(date) => query = format!("{} AND datetime_start <= '{} 00:00:00'", query, date.format("%Y-%m-%d").to_string()),
        None => {}
    };
    match source_filter {
        Some(source) => query = format!("{} AND source_name <= '{}'", query, source),
        None => {}
    }
    query += " GROUP BY DATE(datetime_start) ORDER BY day ASC;";

    let rows_opt = sqlx::query(&query)
        .fetch_all(&pool)
        .await;

    match rows_opt {
        Ok(rows) => {
            for row in rows {
                let date: NaiveDate = row.get("day");
                let value: f32 = row.get("total_value");
                all_data.push((date, value));
            }
        }
        Err(e) => { panic!("Error: {}", e); }
    }

    return all_data;
}

pub async fn get_sources_by_type(record_type: &str) -> Vec<String> {
    let mut all_data: Vec<String> = Vec::new();
    let pool: MySqlPool = match store::common_store::create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };

    let query: String = format!("SELECT DISTINCT source_name FROM apple_records WHERE record_type = '{}';", record_type);
    let rows_opt = sqlx::query(&query)
        .fetch_all(&pool)
        .await;
    match rows_opt {
        Ok(rows) => {
            for row in rows {
                let source_name: String = row.get("source_name");
                all_data.push(source_name);
            }
        }
        Err(e) => { panic!("Error: {}", e); }
    }
    return all_data;
}


pub async fn add_apple_record_entries(records: Vec<AppleRecordDto>) {
    let pool: MySqlPool = match store::common_store::create_pool().await {
        Ok(pool) => { pool },
        Err(e) => { panic!("{}", e); },
    };
    if records.len() == 0 {
        return;
    }

    let query: String = String::from("INSERT IGNORE INTO apple_records (log_id, record_type, source_name, datetime_start, datetime_end, value, unit) VALUES ");

    let mut transaction_cnt = 0;
    let mut transaction_query: String = query.clone();
    for record in records {
        let mut insert_line: String = String::new();
        match record.log_id {
            Some(element) => insert_line += format!("{}, ", element).as_str(),
            None => insert_line += "NULL, "
        }
        match record.record_type {
            Some(element) => insert_line += format!("'{}', ", element).as_str(),
            None => insert_line += "NULL, "
        }
        match record.source_name {
            Some(element) => insert_line += format!("'{}', ", element).as_str(),
            None => insert_line += "NULL, "
        }
        match record.datetime_start {
            Some(element) => insert_line += format!("'{}', ", element).as_str(),
            None => insert_line += "NULL, "
        }
        match record.datetime_end {
            Some(element) => insert_line += format!("'{}', ", element).as_str(),
            None => insert_line += "NULL, "
        }
        match record.value {
            Some(element) => insert_line += format!("'{}', ", element).as_str(),
            None => insert_line += "NULL, "
        }
        match record.unit {
            Some(element) => insert_line += format!("'{}', ", element).as_str(),
            None => insert_line += "NULL, "
        }
        insert_line.truncate(insert_line.len() - 2);
        transaction_query += format!("({}), ", insert_line).as_str();
        transaction_cnt += 1;

        if transaction_cnt == 1000 {
            transaction_query.truncate(transaction_query.len() - 2);
            transaction_query += ";";
            let a = sqlx::query(&transaction_query)
                .execute(&pool)
                .await;
            println!("{:?}", a);
            transaction_query = query.clone();
            transaction_cnt = 0;
        }
    }
    
    if transaction_cnt != 0 {
        transaction_query.truncate(transaction_query.len() - 2);
        transaction_query += ";";
        let a = sqlx::query(&transaction_query)
            .execute(&pool)
            .await;
        println!("{:?}", a);
    }
}
