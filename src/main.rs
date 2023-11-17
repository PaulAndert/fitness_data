use std::time::Instant;
use clap::Parser;

mod adapters; 
mod database;
mod models;

mod args;
mod common;
mod graph;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let args = args::Args::parse();

    match args.source {
        Some(args::Source::Apple) => {
            adapters::apple::main(args).await;
        },
        Some(args::Source::Concept2) => {
            adapters::concept2::main(args).await;
        },
        Some(args::Source::Fddb) => {
            adapters::fddb::main().await;
        },
        None => {
            panic!("Error: Unknown Source");
        }
    };
    
    println!("Elapsed: {:.2?}", now.elapsed());
    Ok(())
}
