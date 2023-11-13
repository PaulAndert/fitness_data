use chrono::NaiveDate;

pub fn get_low_high(data: Vec<(NaiveDate, f32)>) -> (NaiveDate, NaiveDate) {
    // returns a tuple of the first and last Date from the given Vec of (Date,f32) tuples
    let mut x_lowest: NaiveDate = data[0].0;
    let mut x_highest: NaiveDate = data[0].0;
    for item in data {
        if item.0 < x_lowest {
            x_lowest = item.0;
        }else if item.0 > x_highest {
            x_highest = item.0;
        }
    }
    // this is so that the graph has some padding to the sides
    x_lowest = x_lowest.checked_sub_days(chrono::Days::new(3)).unwrap();
    x_highest = x_highest.checked_add_days(chrono::Days::new(3)).unwrap();

    return (x_lowest, x_highest);
}