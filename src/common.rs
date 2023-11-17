use chrono::NaiveDate;

pub fn get_x_low_high(data: Vec<NaiveDate>) -> (NaiveDate, NaiveDate) {
    // returns a tuple of the first and last Date from the given Vec of (Date,f32) tuples
    let mut x_low: NaiveDate = data[0];
    let mut x_high: NaiveDate = data[0];
    for item in data {
        if item < x_low {
            x_low = item;
        }else if item > x_high {
            x_high = item;
        }
    }
    // this is so that the graph has some padding to the sides
    x_low = x_low.checked_sub_days(chrono::Days::new(3)).unwrap();
    x_high = x_high.checked_add_days(chrono::Days::new(3)).unwrap();

    return (x_low, x_high);
}

pub fn get_y_low<T: PartialOrd + Copy>(data: Vec<(NaiveDate, T)>) -> (NaiveDate, T) {
    let mut y_low_date: NaiveDate = data[0].0;
    let mut y_low_val: T = data[0].1;
    for item in data {
        if item.1 < y_low_val {
            y_low_date = item.0;
            y_low_val = item.1;
        }
    }
    return (y_low_date, y_low_val);
}

pub fn get_y_high<T: PartialOrd + Copy>(data: Vec<(NaiveDate, T)>) -> (NaiveDate, T){
    let mut y_high_date: NaiveDate = data[0].0;
    let mut y_high_val: T = data[0].1;
    for item in data {
        if item.1 > y_high_val {
            y_high_date = item.0;
            y_high_val = item.1;
        }
    }
    return (y_high_date, y_high_val);
}

