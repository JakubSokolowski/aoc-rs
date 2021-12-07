use crate::common::parse::parse_numbers;

pub fn run(input: &str) {
    let min_1 = part_1(input);
    println!("Part 1 total fuel: {}", min_1);
    let min_2 = part_2(input);
    println!("Part 2 total fuel: {}", min_2);
}

pub fn part_1(input: &str) -> i64 {
    let crabs = parse_numbers(input);
    let max_pos = crabs.iter().max().unwrap();
    let mut min_fuel = i64::MAX;

    for pos in 0..=*max_pos {
        let fuel = total_fuel(pos, &crabs);
        if fuel < min_fuel {
            min_fuel = fuel
        }
    }

    min_fuel
}

pub fn part_2(input: &str) -> i64 {
    let crabs = parse_numbers(input);
    let max_pos = crabs.iter().max().unwrap();

    let mut min_fuel = i64::MAX;

    for pos in 0..=*max_pos {
        let fuel = total_fuel_2(pos, &crabs);
        if fuel < min_fuel {
            min_fuel = fuel
        }
    }

    min_fuel
}

pub fn total_fuel(position: i64, crabs: &[i64]) -> i64 {
    crabs.iter().map(|c| (position - *c).abs()).sum()
}

pub fn total_fuel_2(position: i64, crabs: &[i64]) -> i64 {
    crabs.iter().map(|c| gaussian((position - *c).abs())).sum()
}

fn gaussian(n: i64) -> i64 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // given
        let input = "16,1,2,0,4,2,7,1,2,14";

        // when
        let result = part_1(input);

        // then
        let expected = 37_i64;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        // given
        let input = "16,1,2,0,4,2,7,1,2,14";

        // when
        let result = part_2(input);

        // then
        let expected = 168_i64;
        assert_eq!(result, expected);
    }
}
