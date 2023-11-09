
CREATE DATABASE IF NOT EXISTS fitness_data;

use fitness_data;

CREATE TABLE concept2 (
    log_id int NOT NULL,
    work_date timestamp,
    name varchar(255),
    duration_sec int,
    distance int,
    stroke_rate int,
    stroke_count int,
    pace time,
    watts int
); 
--Log ID
--Date
--Description
--Work Time (Seconds) //cut of miliseconds
--"Work Distance"
--"Stroke Rate/Cadence"
--"Stroke Count"
--Pace //cut of miliseconds
--"Avg Watts"

CREATE TABLE known_files (
    id int NOT NULL,
    name varchar(255),
    last_modified timestamp
); 


CREATE TABLE apple_walk (
    ...
); 