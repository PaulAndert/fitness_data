use std::time::Instant;
use dotenv::dotenv;

mod adapters; 
mod database;
mod models;
mod helper;

use crate::helper::io_helper::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// async fn main() -> Result<()> {
    let now = Instant::now();

    dotenv().ok();

    let options: Vec<&str> = vec!["Fddb", "Concept 2", "Apple"];
    let answer: usize = ask_choice_question("What source would you like to graph? (1, 2, 3, ...)", options);

    match answer {
        1 => {
            adapters::fddb::main().await;
        },
        2 => {
            adapters::concept2::main().await;
        },
        3 => {
            adapters::apple::main().await;
        },
        _ => {
            println!("Error: Unknown Source");
        }
    };
    
    println!("Elapsed: {:.2?}", now.elapsed());
    Ok(())
}
