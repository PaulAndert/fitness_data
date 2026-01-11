# Fitness
Rust Project that generates Graphs to track progress.  
Current Data Sources are: 
- Apple Fitness
- Concept2 RowErg Data
- Fddb Weight Data

## Installation
```console
git clone https://github.com/PaulAndert/fitness_data.git
cd fitness_data
cargo build
```

## Setup
modify the .env_example to .env and change to values inside to match your environment

## Usage
```terminal
cargo run
# or
./target/debug/fitness_data
```

## Get Source Data
#### Fddb
- go to [Fddb](https://fddb.info/db/i18n/account)
- login to your account
- under "My Fddb" click "Weight Report"
- click "My Data"
- click "Export (CSV)"
  

#### Concept 2 Data
- go to [Concept2](https://log.concept2.com/login)
- login to your account
- click "History"
- under "Export" click the "CSV" Button for the Season you want to download

#### Apple Data
- go to the "Apple Health" App
- click your Profile in the top right corner
- scroll to the bottom and click "export"
- from which you get a Export.zip, inside which the necessary XML files are
  
---
## TODO
- add descriptions to axis, ex. avg. kcal per day, ...
- illegal answer - loop question?
- enter -> answer 1
- concept2, fddb transaction based machen + print
- mehr -- optionen: init (db load - execute), reset (drop all tables and danach init), ...
- one / many routes visualised on a map (Heatmap, ...)
- Hearth beats over a year in a diagramm overlay?
