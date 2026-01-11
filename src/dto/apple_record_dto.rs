

use chrono::{DateTime, FixedOffset, Local};

use crate::dto::date_time_field_enum::DateTimeField;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppleRecordDto {
    pub log_id: Option<i32>,
    pub record_type: Option<String>,
    pub source_name: Option<String>,
    pub datetime_start: Option<DateTime<Local>>,
    pub datetime_end: Option<DateTime<Local>>,
    pub value: Option<String>,
    pub unit: Option<String>
}

impl AppleRecordDto {
    pub fn create(log_id: Option<i32>, record_type: Option<String>, source_name: Option<String>, datetime_start: Option<DateTime<Local>>,
            datetime_end: Option<DateTime<Local>>, value: Option<String>, unit: Option<String>) -> AppleRecordDto {
        return AppleRecordDto { 
            log_id,
            record_type,
            source_name,
            datetime_start,
            datetime_end,
            value,
            unit
        };
    }

    pub fn new() -> AppleRecordDto {
        return AppleRecordDto::create(None, None, None, None, None, None, None);
    }

    pub fn set_work_date_from_str(
        &mut self,
        date_str: &str,
        field: DateTimeField
    ) -> Result<(), chrono::ParseError> {
        let dt_fixed: DateTime<FixedOffset> = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z")?;
        let dt_local = dt_fixed.with_timezone(&Local);
        match field {
            DateTimeField::Start => self.datetime_start = Some(dt_local),
            DateTimeField::End => self.datetime_end = Some(dt_local),
        }

        Ok(())
    }
}