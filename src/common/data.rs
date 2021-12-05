use std::fs::File;
use std::io::{BufRead, BufReader, Read};

const DEFAULT_CAPACITY: usize = 1000;

pub fn read_to_vec(year: u32, day: u8, bigboy: bool) -> Vec<String> {
    let mut lines = Vec::with_capacity(DEFAULT_CAPACITY);
    let data_path = get_path(year, day, bigboy);
    let file =
        File::open(&data_path).unwrap_or_else(|_| panic!("Cannot open file at {}", data_path));
    let reader = BufReader::new(file);

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

pub fn read_to_string(year: u32, day: u8, bigboy: bool) -> String {
    let data_path = get_path(year, day, bigboy);
    let mut fp =
        File::open(&data_path).unwrap_or_else(|_| panic!("Cannot open file at {}", data_path));
    let mut buf = String::new();
    fp.read_to_string(&mut buf).unwrap();
    buf
}

pub fn get_path(year: u32, day: u8, bigboy: bool) -> String {
    if bigboy {
        format!("./data/{}/.bigboy/day{:02}.txt", year, day)
    } else {
        format!("./data/{}/day{:02}.txt", year, day)
    }
}
