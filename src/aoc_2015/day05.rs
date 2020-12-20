use fancy_regex::Regex;
use std::collections::HashMap;

pub fn run(input: &Vec<String>) {
    let count = count_nice(input);
    println!("Num of nice strs: {}", count);

    let count_v2 = count_nice_v2(input);
    println!("Num of v2 nice strs {}", count_v2);
}

pub fn count_nice(lines: &Vec<String>) -> usize {
    let blacklist = Regex::new(r"ab|cd|pq|xy").unwrap();

    let count = lines.iter()
        .filter(|line| !blacklist.is_match(line).unwrap())
        .filter(|line| has_vowels(line))
        .filter(|line| repeated_letter(line))
        .count();

    return count;
}

pub fn count_nice_v2(lines: &Vec<String>) -> usize {
    let count = lines.iter()
        .filter(|line| has_separated_pair(line))
        .filter(|line| has_non_overlapping_pairs(line))
        .count();

    return count
}

fn repeated_letter(line: &String) -> bool {
    let mut prev = None;
    for ch in line.chars() {
        if prev.map_or(false, |prev| prev == ch) {
            return true;
        }
        prev = Some(ch);
    }
    false
}

fn has_vowels(line: &String) -> bool {
    let vowels = "aeiou";

    let has_all = line
        .chars()
        .into_iter()
        .filter(|c| vowels.contains(&c.to_string()))
        .count() >= 3;

    has_all
}

fn has_separated_pair(line: &String) -> bool {
    for triplet in line.chars().collect::<Vec<char>>().windows(3) {
        let first = triplet[0];
        let last = triplet[2];

        if first == last {
            return true
        }
    }
    false
}

fn has_non_overlapping_pairs(line: &String) -> bool {
    let mut positions = HashMap::new();
    for (position, pair) in line.chars().collect::<Vec<char>>().windows(2).enumerate() {
        let pair_str: String = pair.iter().collect();

        if positions.contains_key(&pair_str) {
            let dup_position = match positions.get(&pair_str) {
                Some(position) => *position,
                None => 100
            };

            let position_difference = position - dup_position;

            if position_difference >=2 {
                return true
            }
        }

        positions.insert(pair_str, position);
    }
    false
}
