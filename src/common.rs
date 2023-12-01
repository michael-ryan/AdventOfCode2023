use std::{fs::File, io::{BufReader, BufRead}};

pub fn read_file(path: &str) -> Vec<String> {
    read_file_iter(path).collect()
}

pub fn read_file_iter(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
}