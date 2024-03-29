use std::collections::HashMap;

pub fn run(input: &str) {
    println!("Multiple presents {}", count_multiple_presents(input));
    println!(
        "Multiple presents with RoboSanta {}",
        count_multiple_presents_with_robot(input)
    );
}

pub fn count_multiple_presents(input: &str) -> u32 {
    let mut houses: HashMap<House, i64> = HashMap::new();
    let start = House::new(0, 0);
    houses.insert(start, 1);

    let mut current = start;

    for command in input.chars() {
        let next_house = current.next(command);
        *houses.entry(next_house).or_insert(0) += 1;
        current = next_house;
    }

    houses.into_iter().filter(|&(_, v)| v > 0).count() as u32
}

pub fn count_multiple_presents_with_robot(input: &str) -> u32 {
    let mut houses: HashMap<House, i64> = HashMap::new();
    let start = House::new(0, 0);

    houses.insert(start, 2);

    let mut current_santa = start;
    let mut current_robot = start;

    for (idx, command) in input.chars().enumerate() {
        if idx % 2 == 0 {
            let next_house = current_santa.next(command);
            *houses.entry(next_house).or_insert(0) += 1;
            current_santa = next_house;
        } else {
            let next_house = current_robot.next(command);
            *houses.entry(next_house).or_insert(0) += 1;
            current_robot = next_house;
        }
    }

    houses.into_iter().filter(|&(_, v)| v > 0).count() as u32
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct House {
    x: i64,
    y: i64,
}

pub trait VisitNext {
    fn next(&self) -> House;
}

impl House {
    fn new(x: i64, y: i64) -> House {
        House { x, y }
    }

    fn next(&self, direction: char) -> House {
        match direction {
            '^' => House::new(self.x, self.y + 1),
            '>' => House::new(self.x + 1, self.y),
            'v' => House::new(self.x, self.y - 1),
            '<' => House::new(self.x - 1, self.y),
            _ => *self,
        }
    }
}
