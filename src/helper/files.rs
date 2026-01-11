use std::fs::{self, DirEntry};
use chrono::{Local, DateTime};

use crate::store;

pub async fn search_new_files(path: &str, extension: &str) -> Vec<(String, DirEntry)> {
    let mut new_files: Vec<(String, DirEntry)> = Vec::new();
    let mut directories: Vec<String> = vec![String::from(path)];

    loop {
        let current_path: String = match directories.pop() {
            Some(element) => element,
            None => break
        };

        for file_res in fs::read_dir(current_path).unwrap() {
            // is file there
            let file = match file_res {
                Ok(file) => { file },
                Err(e) => { panic!("Error: {}", e); },
            };
            // is metadata there
            let metadata = match file.metadata() {
                Ok(meta) => { meta }
                Err(e)=> { panic!("Error: {}", e); }
            };
            // get last modify date
            let last_modified = match metadata.modified() {
                Ok(systime) => {
                    let last_modified: DateTime<Local> = systime.clone().into();
                    last_modified
                },
                Err(e)=> { panic!("Error: {}", e); }
            };
            // get filename
            let filename = match file.file_name().into_string() {
                Ok(name) => { name },
                Err(e) => { panic!("Error: {:?}", e); },
            };
            // only files
            if metadata.is_dir() {
                directories.push(String::from(file.path().to_str().unwrap()));
                continue;
            }
            // only if the file is the correct extension
            if !filename.contains(extension) || filename == ".DS_Store" {
                continue;
            }
            // check if file is already in the DB
            match store::common_store::is_db_up_to_date(&filename, last_modified).await {
                Ok(boo) => { 
                    match boo {
                        true => {}, // skip
                        false => {  // read new data
                            new_files.push((filename, file));
                        },
                    }    
                },
                Err(e) => { panic!("Error: {:?}", e); },
            };
        }
    }

    return new_files;
}