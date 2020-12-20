pub mod data;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_to_vec(year: u32, day: u8) -> Vec<String> {
    let mut r = Vec::with_capacity(1000);
    let path = format!("./data/{}/day{:02}.txt", year, day);
    let fp = File::open(&path).expect(&format!("Can't open {}", path));
    let fp = BufReader::new(fp);
    for line in fp.lines() {
        r.push(line.unwrap());
    }
    return r;
}

pub fn read_to_str(year: u32, day: u8) -> String {
    let path = format!("./data/{}/day{:02}.txt", year, day);
    let mut fp = File::open(&path).expect(&format!("Can't open {}", path));
    let mut buf = String::new();
    fp.read_to_string(&mut buf).unwrap();
    return buf;
}
