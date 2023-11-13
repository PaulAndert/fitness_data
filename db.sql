
CREATE DATABASE IF NOT EXISTS fitness_data;

use fitness_data;

CREATE TABLE known_files (
    id int NOT NULL AUTO_INCREMENT,
    name varchar(255),
    last_modified timestamp,
    PRIMARY KEY (id)
); 

CREATE TABLE concept2 (
    log_id int NOT NULL,
    work_date timestamp,
    name varchar(255),
    duration_sec int, ;;TODO: change to float for the miliseconds
    distance int,
    stroke_rate int, ;;TODO: change to float for the miliseconds
    stroke_count int,
    pace time, ;;TODO: change to float for the miliseconds
    watts int,
    PRIMARY KEY (log_id)
);

CREATE TABLE fddb (
    work_date date,
    weight float,
    PRIMARY KEY (work_date)
);

CREATE TABLE apple_walk (
    ...
); 