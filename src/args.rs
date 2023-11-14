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

    /// Select what is displayed on the y-axis: "duration", "distance", "stroke-rate", "stroke-count", "pace", "watts", ...
    #[clap(value_enum)]
    pub y_axis: Option<YAxis>,
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
    Meter1k,
    Meter2k,
    Meter5k
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum YAxis {
    Duration,
    Distance,
    StrokeRate,
    StrokeCount,
    Pace,
    Watts
}