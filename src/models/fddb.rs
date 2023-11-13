use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Fddb {
    pub work_date: NaiveDate,
    pub weight: f32
}

impl Fddb {
    pub fn create(work_date: NaiveDate, weight: f32) -> Fddb {
        return Fddb { 
            work_date,
            weight
        };
    }
}