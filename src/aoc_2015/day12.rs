use lazy_static::lazy_static;
use regex::Regex;

pub fn run(input: &str) {
    let sum = sum_nums(input);
    println!("Sum: {}", sum);
}

fn sum_nums(s :&str) -> i64 {
    let nums = extract_nums(s);
    nums.iter().sum()
}

fn extract_nums(s: &str) -> Vec<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    RE.find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}