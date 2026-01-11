use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct FddbDto {
    pub work_date: NaiveDate,
    pub weight: f32
}

impl FddbDto {
    pub fn create(work_date: NaiveDate, weight: f32) -> FddbDto {
        return FddbDto { 
            work_date,
            weight
        };
    }
}