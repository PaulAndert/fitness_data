
CREATE DATABASE IF NOT EXISTS fitness_data;

use fitness_data;

CREATE TABLE files (
    id int NOT NULL AUTO_INCREMENT,
    name varchar(255),
    last_modified timestamp,
    PRIMARY KEY (id)
); 

CREATE TABLE concept2 (
    log_id int NOT NULL,
    work_date timestamp,
    name varchar(255),
    duration_sec float,
    distance int,
    stroke_rate int,
    stroke_count int,
    pace_sec float,
    watts int,
    PRIMARY KEY (log_id)
);

CREATE TABLE fddb (
    work_date date,
    weight float,
    PRIMARY KEY (work_date)
);

CREATE TABLE apple_records (
    log_id int NOT NULL AUTO_INCREMENT,
    record_type varchar(255),
    source_name varchar(255),
    datetime_start timestamp,
    datetime_end timestamp,
    value varchar(255),
    unit varchar(255),
    PRIMARY KEY (log_id)
); 

ALTER TABLE apple_records ADD CONSTRAINT uq_record UNIQUE (record_type, source_name, datetime_start, datetime_end);