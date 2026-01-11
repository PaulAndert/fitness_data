use std::time::Instant;
use std::env;
use dotenv::dotenv;

mod service; 
mod store;
mod dto;
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
        (1, false) => service::fddb_service::main().await,
        (1, true) => service::fddb_service::load_data().await,
        (2, false) => service::concept2_service::main().await,
        (2, true) => service::concept2_service::load_data().await,
        (3, false) => service::apple_service::main().await,
        (3, true) => service::apple_service::load_data().await,
        _ => {
            println!("Error: Unknown Source");
        }
    };
    
    println!("Elapsed: {:.2?}", now.elapsed());
    Ok(())
}
