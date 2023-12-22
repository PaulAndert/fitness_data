# Fitness
Rust Project that generates Graphs to track progress. Possible Imports are currently: Apple Fitness, Concept2 RowErg Data and Fddb Weight Data.

### Usage
fitness_data [SOURCE] [SPORT] [WORKOUT] [Y-Axis]
```terminal
cargo run -- <Source> <Sport> <Workout> <Y-Axis>
```
source: apple, concept, fddb, ...  
Sport: rowing, walking, ...  
Workout: min10, min15, meter5000, meter2000, ...  
Y-Axis: duration, distance, stroke-rate, stroke-count, pace, watts, ... (what value is on this Axis, X-Axis is always time)

### What to do
- alle routen auf map visualisieren (Heatmap, ...)
- all row in diagramm
- all walk in diagramm
- all lift in diagramm
- Hearth beats over a year in a diagramm overlay

##### ROWING
- ~all 5000 per time~
- ~all 10min per distance~
- ~all 15min per distance~
- multiple datapoints in one graph
- ~dynamicaly choose (CLI) what the y-axis contains~

##### Walking
- one map with one date
- one map with one month
- one map with all datapoints
