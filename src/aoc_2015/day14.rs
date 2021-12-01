pub fn run(input: &[String]) {
    let mut reindeers = parse_input(input);

    let max = reindeers
        .iter_mut()
        .map(|r| r.run(2503))
        .inspect(|d| println!("Distance: {}", d))
        .max()
        .unwrap();

    println!("Max distance: {}", max)
}

#[derive(Debug)]
enum State {
    Running,
    Resting,
}

impl Default for State {
    fn default() -> Self {
        State::Running
    }
}

#[derive(Debug, Default)]
struct Reindeer {
    speed: usize,
    run_time: usize,
    rest_time: usize,
    rest_budget: usize,
    run_budget: usize,
    state: State,
}

impl Reindeer {
    pub fn run(&mut self, seconds: usize) -> usize {
        let res = (0..=seconds)
            .collect::<Vec<usize>>()
            .iter()
            .map(|_| self.tick())
            .sum();

        self.reset();

        res
    }

    pub fn reset(&mut self) {
        self.run_budget = 0;
        self.rest_budget = 0;
        self.state = State::Running;
    }

    pub fn tick(&mut self) -> usize {
        match self.state {
            State::Running => {
                self.run_budget += 1;

                if self.run_budget == self.run_time {
                    self.run_budget = 0;
                    self.state = State::Resting;
                }

                self.speed
            }
            State::Resting => {
                self.rest_budget += 1;

                if self.rest_budget == self.rest_time {
                    self.rest_budget = 0;
                    self.state = State::Running
                }

                0
            }
        }
    }
}

fn parse_input(input: &[String]) -> Vec<Reindeer> {
    input.iter().map(|l| to_reindeer(l)).collect()
}

fn to_reindeer(input: &str) -> Reindeer {
    let tokens: Vec<&str> = input.split(' ').collect();
    let speed: usize = tokens[3].parse().unwrap();
    let run_time: usize = tokens[6].parse().unwrap();
    let rest_time: usize = tokens[13].parse().unwrap();

    Reindeer {
        speed,
        run_time,
        rest_time,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_2015::day14::Reindeer;

    #[test]
    fn test_travels_full_amount() {
        // given
        let mut r = Reindeer {
            speed: 10,
            run_time: 1,
            rest_time: 1,
            ..Default::default()
        };

        // then
        assert_eq!(r.run(1), 10);
    }

    #[test]
    fn test_travels_full_distance_rests_and_travels_again() {
        // given
        let mut r = Reindeer {
            speed: 10,
            run_time: 1,
            rest_time: 1,
            ..Default::default()
        };

        // then
        assert_eq!(r.run(3), 20);
    }

    #[test]
    fn test_travels_full_distance_rests_and_travels_again_twice() {
        // given
        let mut r = Reindeer {
            speed: 10,
            run_time: 1,
            rest_time: 1,
            ..Default::default()
        };

        // then
        assert_eq!(r.run(5), 30);
    }
}
