use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Select Source: "apple", "concept", "fddb", ...
    #[clap(value_enum)]
    pub source: Option<Source>,

    /// Select Sport: "rowing", "walking", ...
    #[clap(value_enum)]
    pub sport: Option<Sport>,

    /// Select Workout Type: "minute", "meter", ...
    #[clap(value_enum)]
    pub workout: Option<Workout>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Source {
    Apple,
    Concept2,
    Fddb
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Sport {
    Rowing,
    Walking
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Workout {
    Min1,
    Min10,
    Min15,
    Meter1000,
    Meter2000,
    Meter5000
}