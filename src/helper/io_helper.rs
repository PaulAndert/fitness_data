use std::io;
use chrono::{Days, Local, Months, NaiveDate};
use crate::models::range::*;


pub fn ask_choice_question(question: &str, options: Vec<&str>) -> usize {

    let top = "-".repeat(question.len());
    println!("{}\n{}", top, question);
    for (index, option) in options.iter().enumerate() {
        println!("{}) {}", index + 1, option);
    }

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("error: unable to read user input");
    
    return match user_input.trim().parse::<usize>() {
        Ok(number) => number,
        Err(e) => panic!("{}", e)
    };
}


pub fn ask_input_question(question: &str) -> String {

    let top = "-".repeat(question.len());
    println!("{}\n{}", top, question);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("error: unable to read user input");
    
    return String::from(user_input.trim());
}

pub fn ask_range() -> Range {

    let options = vec!["No", "Yes, by date range", "Yes, by range description (last x weeks, ...)"];
    let answer: usize = ask_choice_question("Do you want to filter the data?", options);

    match answer {
        1 => return Range::create(None, None),
        2 => {
            let answer_start: String = ask_input_question("Plese give the date from when the data should be used. (Plese use the format yyyy-mm-dd)");
            let range_start: NaiveDate = match NaiveDate::parse_from_str(&answer_start, "%Y-%m-%d") {
                Ok(date) => date,
                Err(e) => panic!("The Date specified is either invalid or in the wrong format: {}\n{}", answer_start, e)
            };
            let answer_end: String = ask_input_question("Plese give the date until when the data should be used. (Plese use the format yyyy-mm-dd)");
            let range_end: NaiveDate = match NaiveDate::parse_from_str(&answer_end, "%Y-%m-%d") {
                Ok(date) => date,
                Err(e) => panic!("The Date specified is either invalid or in the wrong format: {}\n{}", answer_end, e)
            };
            return Range::create(Some(range_start), Some(range_end));
        },
        3 => {
            let options = vec!["Days", "Weeks", "Months", "Quarters", "Years"];
            let answer_range_specifier = ask_choice_question("What range do you want to specify?", options.clone());

            if answer_range_specifier <= 0 && answer_range_specifier > options.len() {
                panic!("The Option specified is not valid: {}", answer_range_specifier);
            }

            let answer_range_size_string: String = ask_input_question(format!("How many {} of data should be used?", options[answer_range_specifier - 1]).as_str());
            let answer_range_size_number: u32 = match answer_range_size_string.parse::<u32>() {
                Ok(number) => number,
                Err(e) => panic!("The Number specified is either invalid or in the wrong format: {}\n{}", answer_range_size_string, e)
            };

            let today: NaiveDate = Local::now().date_naive();
            let range_start: NaiveDate = match answer_range_specifier {
                1 => match today.checked_sub_days(Days::new(answer_range_size_number as u64)) {
                    Some(date) => date,
                    None => panic!("Number of Days could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                2 => match today.checked_sub_days(Days::new((answer_range_size_number as u64) * 7)) {
                    Some(date) => date,
                    None => panic!("Number of Weeks could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                3 => match today.checked_sub_months(Months::new(answer_range_size_number)) {
                    Some(date) => date,
                    None => panic!("Number of Months could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                4 => match today.checked_sub_months(Months::new(answer_range_size_number * 3)) {
                    Some(date) => date,
                    None => panic!("Number of Quarters could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                5 => match today.checked_sub_months(Months::new(answer_range_size_number * 12)) {
                    Some(date) => date,
                    None => panic!("Number of Years could not be subtracted {} from Date {}", answer_range_size_number, today),
                },
                _ => panic!("The Option specified is not valid: {}", answer_range_specifier)
            };

            return Range::create(Some(range_start), Some(today));
        },
        _ => panic!("The option specified is not valid: {}", answer)
    };
}