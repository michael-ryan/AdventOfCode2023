mod common;

use std::{fs::File, io::{BufReader, BufRead}};

pub fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
