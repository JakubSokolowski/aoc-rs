use std::collections::HashMap;
use itertools::Itertools;

pub struct Vertex {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Happiness {
    from: String,
    to: String,
    delta: i32,
}


pub struct AsymmetricHappinessMatrix {
    name_lookup: HashMap<String, usize>,
    matrix: Vec<Vec<i32>>,
    distances: Vec<Happiness>,
    cities: Vec<String>,
}

impl AsymmetricHappinessMatrix {
    fn new(distances: Vec<Happiness>) -> AsymmetricHappinessMatrix {
        let guests = AsymmetricHappinessMatrix::uniq_guests(&distances);

        let name_lookup: HashMap<_, _> = guests
            .iter()
            .enumerate()
            .map(|(i, c)| (c.to_string(), i))
            .collect();

        let matrix_size = guests.len();
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; matrix_size]; matrix_size];

        for distance in &distances {
            let from_idx = *name_lookup.get(&distance.from).unwrap();
            let to_idx = *name_lookup.get(&distance.to).unwrap();
            matrix[from_idx][to_idx] = distance.delta;
        }

        AsymmetricHappinessMatrix { name_lookup, matrix, distances, cities: guests }
    }

    fn uniq_guests(distances: &Vec<Happiness>) -> Vec<String> {
        let all_froms: Vec<String> = distances
            .iter()
            .map(|d| d.from.to_string())
            .collect();

        let all_tos: Vec<String> = distances
            .iter()
            .map(|d| d.to.to_string())
            .collect();

        all_froms
            .into_iter()
            .chain(all_tos.into_iter())
            .unique()
            .collect()
    }

    fn happiness(&self, from: usize, to: usize) -> i32 {
        self.matrix[from][to]
    }

    fn happiness_guests(&self, from: &str, to: &str) -> i32 {
        self.happiness(
            self.name_lookup.get(from).unwrap().clone(),
            self.name_lookup.get(to).unwrap().clone(),
        )
    }

    fn get_seating_delta(&self, cities: Vec<&str>) -> i32 {
        let in_line: i32 = cities
            .windows(2)
            .map(|pair| {
                self.happiness_guests(&pair[0], &pair[1]) +  self.happiness_guests(&pair[1], &pair[0])
            })
            .sum();

        let close: i32 = self.happiness_guests(cities.last().unwrap(), cities.first().unwrap())
            + self.happiness_guests(cities.first().unwrap(), cities.last().unwrap());

        in_line + close
    }

    fn solve_max(&self) {
        let mut best = 0;
        for perm in self.cities.iter().permutations(self.cities.len()).unique() {
            let v2 = perm.iter().map(|s| s.as_str()).collect();
            let tour = self.get_seating_delta(v2);
            if tour > best {
                best = tour;
            }
        }
        println!("Longest route: {}", best)
    }

    fn solve_min(&self) {
        let mut best = i32::MAX;
        for perm in self.cities.iter().permutations(self.cities.len()).unique() {
            let v2 = perm.iter().map(|s| s.as_str()).collect();
            let tour = self.get_seating_delta(v2);
            if tour < best {
                best = tour;
            }
        }
        println!("Shortest route: {}", best)
    }
}

pub fn run(input: &Vec<String>) {
    let distances = parse_input(input);
    println!("{:?}", distances);
    let matrix = AsymmetricHappinessMatrix::new(distances);
    println!("Alice -> Bob: {}", matrix.happiness_guests("Alice", "Bob"));
    // matrix.solve_min();
    matrix.solve_max();
}


fn parse_input(input: &Vec<String>) -> Vec<Happiness> {
    input.iter()
        .map(|line| {
            let tokens: Vec<&str> = line.split(" ").collect();
            let from = tokens[0].to_string();
            let to = tokens[10].to_string().replace(".", "");

            let multiplier: i32 = match tokens[2] {
                "gain" => 1,
                "lose" => -1,
                _ => panic!("Invalid delta")
            };
            let delta = tokens[3].parse::<i32>().unwrap() * multiplier;
            Happiness { from, to, delta }
        })
        .collect()
}