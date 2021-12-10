use std::env;

mod aoc_2015;
mod aoc_2019;
mod aoc_2021;
mod common;

use crate::common::data::{read_to_string, read_to_vec};

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 3 {
        panic!("Usage: cargo run <year> <day> <bigboy>");
    }

    let year: u32 = argv[1].parse().expect("Year must be a number");
    let day: u8 = argv[2].parse().expect("Day must be a number");
    let bigboy = argv.get(3).is_some();
    println!("Running year {}, day {}", year, day);

    match (year, day) {
        // 2015
        (2015, 1) => aoc_2015::day01::run(&common::data::read_to_string(year, day, bigboy)),
        (2015, 2) => aoc_2015::day02::run(&common::data::read_to_vec(year, day, bigboy)),
        (2015, 3) => aoc_2015::day03::run(&common::data::read_to_string(year, day, bigboy)),
        (2015, 4) => aoc_2015::day04::run("iwrupvqb"),
        (2015, 5) => aoc_2015::day05::run(&read_to_vec(year, day, bigboy)),
        (2015, 6) => aoc_2015::day06::run(&read_to_vec(year, day, bigboy)),
        (2015, 7) => aoc_2015::day07::run(&read_to_vec(year, day, bigboy)),
        (2015, 8) => aoc_2015::day08::run(&read_to_string(year, day, bigboy)),
        (2015, 9) => aoc_2015::day09::run(&read_to_vec(year, day, bigboy)),
        (2015, 10) => aoc_2015::day10::run("1113222113"),
        (2015, 11) => aoc_2015::day11::run("hxbxxyzz"),
        (2015, 12) => aoc_2015::day12::run(&read_to_string(year, day, bigboy)),
        (2015, 13) => aoc_2015::day13::run(&read_to_vec(year, day, bigboy)),
        (2015, 14) => aoc_2015::day14::run(&read_to_vec(year, day, bigboy)),

        // 2019
        (2019, 1) => aoc_2019::day01::run(&common::data::read_to_vec(year, day, bigboy)),
        (2019, 2) => aoc_2019::day02::run(&common::data::read_to_string(year, day, bigboy)),

        // 2021
        (2021, 1) => aoc_2021::day01::run(&common::data::read_to_vec(year, day, bigboy)),
        (2021, 2) => aoc_2021::day02::run(&common::data::read_to_vec(year, day, bigboy)),
        (2021, 3) => aoc_2021::day03::run(&common::data::read_to_vec(year, day, bigboy)),
        (2021, 4) => aoc_2021::day04::run(&common::data::read_to_string(year, day, bigboy)),
        (2021, 5) => aoc_2021::day05::run(&common::data::read_to_vec(year, day, bigboy)),
        (2021, 6) => aoc_2021::day06::run(&common::data::read_to_string(year, day, bigboy)),
        (2021, 7) => aoc_2021::day07::run(&common::data::read_to_string(year, day, bigboy)),
        (2021, 8) => aoc_2021::day08::run(&common::data::read_to_vec(year, day, bigboy)),
        (2021, 9) => aoc_2021::day09::run(&common::data::read_to_vec(year, day, bigboy)),
        (2021, 10) => aoc_2021::day10::run(&common::data::read_to_vec(year, day, bigboy)),

        (_, _) => panic!("Not implemented"),
    }
}
