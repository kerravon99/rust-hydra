use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).expect("File not found"))
        .lines()
        .filter_map(Result::ok)
        .collect()
}
