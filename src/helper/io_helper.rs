use std::io;

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