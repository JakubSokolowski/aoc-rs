use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Happiness {
    from: String,
    to: String,
    delta: i64,
}

pub struct AsymmetricHappinessMatrix {
    name_lookup: HashMap<String, usize>,
    matrix: Vec<Vec<i64>>,
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
        let mut matrix: Vec<Vec<i64>> = vec![vec![0; matrix_size]; matrix_size];

        for distance in &distances {
            let from_idx = *name_lookup.get(&distance.from).unwrap();
            let to_idx = *name_lookup.get(&distance.to).unwrap();
            matrix[from_idx][to_idx] = distance.delta;
        }

        AsymmetricHappinessMatrix {
            name_lookup,
            matrix,
            cities: guests,
        }
    }

    fn uniq_guests(distances: &[Happiness]) -> Vec<String> {
        let all_froms = distances.iter().map(|d| d.from.to_string());

        let all_tos = distances.iter().map(|d| d.to.to_string());

        all_froms
            .into_iter()
            .chain(all_tos.into_iter())
            .unique()
            .collect()
    }

    fn happiness(&self, from: usize, to: usize) -> i64 {
        self.matrix[from][to]
    }

    fn happiness_guests(&self, from: &str, to: &str) -> i64 {
        self.happiness(
            *self.name_lookup.get(from).unwrap(),
            *self.name_lookup.get(to).unwrap(),
        )
    }

    fn get_seating_delta(&self, cities: Vec<&str>) -> i64 {
        let in_line: i64 = cities
            .windows(2)
            .map(|pair| {
                self.happiness_guests(pair[0], pair[1]) + self.happiness_guests(pair[1], pair[0])
            })
            .sum();

        let close: i64 = self.happiness_guests(cities.last().unwrap(), cities.first().unwrap())
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
}

pub fn run(input: &[String]) {
    let distances = parse_input(input);
    println!("{:?}", distances);
    let matrix = AsymmetricHappinessMatrix::new(distances);
    println!("Alice -> Bob: {}", matrix.happiness_guests("Alice", "Bob"));
    // matrix.solve_min();
    matrix.solve_max();
}

fn parse_input(input: &[String]) -> Vec<Happiness> {
    input
        .iter()
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            let from = tokens[0].to_string();
            let to = tokens[10].to_string().replace('.', "");

            let multiplier: i64 = match tokens[2] {
                "gain" => 1,
                "lose" => -1,
                _ => panic!("Invalid delta"),
            };
            let delta = tokens[3].parse::<i64>().unwrap() * multiplier;
            Happiness { from, to, delta }
        })
        .collect()
}
