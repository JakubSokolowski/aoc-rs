use itertools::Itertools;

pub fn run(input: &[String]) {
    let max = top_calories(input);
    println!("Top: {}", max);

    let top_3 = top_3_sum(input);
    println!("Top 3 sum: {}", top_3);
}

pub fn get_calories(input: &[String]) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    let mut curr = 0;
    for line in input {
        if line.is_empty() {
            res.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<i64>().unwrap();
        }
    }
    res
}

pub fn top_calories(input: &[String]) -> i64 {
    *get_calories(input).iter().max().unwrap()
}

pub fn top_3_sum(input: &[String]) -> i64 {
    get_calories(input)
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum()
}
