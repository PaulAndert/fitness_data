use chrono::{NaiveDate, Duration};

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

pub fn get_y_duration_low_high(data: Vec<Duration>) -> (Duration, Duration) {
    // returns a tuple of the first and last Date from the given Vec of (Date,f32) tuples
    let mut y_low: Duration = data[0];
    let mut y_high: Duration = data[0];
    for item in data {
        if item < y_low {
            y_low = item;
        }else if item > y_high {
            y_high = item;
        }
    }
    // this is so that the graph has some padding to the sides
    let y_l = Duration::seconds((y_low.num_seconds() as f32 * 0.95) as i64);
    let y_h = Duration::seconds((y_high.num_seconds() as f32 * 1.05) as i64);

    return (y_l, y_h);
}

pub fn get_y_f32_low_high(data: Vec<f32>) -> (f32, f32) {
    // returns a tuple of the first and last Date from the given Vec of (Date,f32) tuples
    let mut y_low: f32 = data[0];
    let mut y_high: f32 = data[0];
    for item in data {
        if item < y_low {
            y_low = item;
        }else if item > y_high {
            y_high = item;
        }
    }
    // this is so that the graph has some padding to the sides
    y_low = y_low * 0.95;
    y_high = y_high * 1.05;

    return (y_low, y_high);
}