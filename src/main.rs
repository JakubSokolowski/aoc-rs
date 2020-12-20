use std::env;

mod common;
mod aoc_2015;

use common::data;
use crate::common::data::read_to_vec;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 {
        panic!("Usage: cargo run <year> <day>");
    }


    let year: u32 = argv[1].parse().expect("Year must be a number");
    let day: u8 = argv[2].parse().expect("Day must be a number");
    println!("Running year {}, day {}", year, day);

    match (year, day) {
        // 2015
        (2015, 4) => aoc_2015::day04::run("iwrupvqb"),
        (2015, 5) => aoc_2015::day05::run(&read_to_vec(year, day)),
        (2015, 6) => aoc_2015::day06::run(&read_to_vec(year, day)),

        (_, _) => panic!("Not implemented"),
    }
}
