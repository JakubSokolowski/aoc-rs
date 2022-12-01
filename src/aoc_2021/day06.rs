use crate::common::parse::parse_numbers;

const DEFAULT_DAYS_TO_SPAWN: i64 = 6;

struct LanternFish {
    days_to_spawn: i64,
    days_left: i64,
}

impl LanternFish {
    fn will_spawn(&self) -> bool {
        matches!(self.days_left, 0)
    }

    fn pass_day(&mut self) {
        self.days_left -= 1;
    }

    fn spawn(&mut self) -> LanternFish {
        self.days_left = self.days_to_spawn;

        LanternFish {
            days_to_spawn: self.days_to_spawn,
            days_left: self.days_to_spawn + 2,
        }
    }
}

struct LanternFishSchool {
    fish: Vec<LanternFish>,
}

impl LanternFishSchool {
    fn pass_days(&mut self, num_days: i64) {
        for _ in 0..num_days {
            self.pass_day();
        }
    }

    fn pass_day(&mut self) {
        let mut new_fish: Vec<LanternFish> = vec![];

        for fish in &mut self.fish {
            match fish.will_spawn() {
                true => {
                    new_fish.push(fish.spawn());
                }
                false => fish.pass_day(),
            }
        }

        self.fish.extend(new_fish);
    }

    fn population_size(&self) -> i64 {
        self.fish.len() as i64
    }
}

fn parse_fish(input: &str) -> Vec<LanternFish> {
    parse_numbers(input)
        .iter()
        .map(|n| LanternFish {
            days_left: *n,
            days_to_spawn: DEFAULT_DAYS_TO_SPAWN,
        })
        .collect()
}

fn population_size(input: &str, num_days: i64) -> i64 {
    let mut school = LanternFishSchool {
        fish: parse_fish(input),
    };
    school.pass_days(num_days);
    school.population_size()
}

fn population_size_v2(input: &str, num_days: i64) -> i64 {
    let fish = parse_numbers(input);
    let mut groups: Vec<i64> = vec![0; 9];

    for fish in fish {
        groups[fish as usize] += 1;
    }

    for _ in 0..num_days {
        groups[7] += groups[0];
        groups.rotate_left(1);
    }

    groups.iter().sum()
}

pub fn run(input: &str) {
    let days = 80;
    let size = population_size(input, days);
    println!("Population after {} days: {}", days, size);
    let days_2 = 256;
    let size_2 = population_size_v2(input, 256);
    println!("Population after {} days: {}", days_2, size_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_size_example_1() {
        // given
        let input = "3,4,3,1,2";
        let num_days = 18;

        // when
        let result = population_size(input, num_days);

        // then
        assert_eq!(result, 26)
    }

    #[test]
    fn test_population_size_v2_example_1() {
        // given
        let input = "3,4,3,1,2";

        // then
        assert_eq!(population_size_v2(input, 1), 5);
        assert_eq!(population_size_v2(input, 2), 6);
        assert_eq!(population_size_v2(input, 3), 7);
        assert_eq!(population_size_v2(input, 4), 9);
        assert_eq!(population_size_v2(input, 5), 10);
        assert_eq!(population_size_v2(input, 6), 10);
        assert_eq!(population_size_v2(input, 7), 10);
        assert_eq!(population_size_v2(input, 8), 10);
        assert_eq!(population_size_v2(input, 9), 11);
        assert_eq!(population_size_v2(input, 18), 26);
    }

    #[test]
    fn test_population_size_example_2() {
        // given
        let input = "3,4,3,1,2";
        let num_days = 80;

        // when
        let result = population_size(input, num_days);

        // then
        assert_eq!(result, 5934)
    }
}
