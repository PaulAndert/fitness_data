use std::time::Instant;
use std::env;
use dotenv::dotenv;

mod adapters; 
mod database;
mod models;
mod helper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// async fn main() -> Result<()> {
    let now = Instant::now();
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let load_data = if args.len() > 1 && args[1] == "load" {
        true
    } else {
        false
    };

    let options: Vec<&str> = vec!["Fddb", "Concept 2", "Apple"];
    let answer: usize = helper::io_helper::ask_choice_question("What source would you like to graph? (1, 2, 3, ...)", options);

    match (answer, load_data) {
        (1, false) => adapters::fddb::main().await,
        (1, true) => adapters::fddb::load_data().await,
        (2, false) => adapters::concept2::main().await,
        (2, true) => adapters::concept2::load_data().await,
        (3, false) => adapters::apple::main().await,
        (3, true) => adapters::apple::load_data().await,
        _ => {
            println!("Error: Unknown Source");
        }
    };
    
    println!("Elapsed: {:.2?}", now.elapsed());
    Ok(())
}
