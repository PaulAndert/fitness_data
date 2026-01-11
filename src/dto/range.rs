use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Range {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

impl Range {
    pub fn create(start: Option<NaiveDate>, end: Option<NaiveDate>) -> Range {
        return Range { 
            start,
            end
        };
    }
}