use std::collections::HashMap;
use itertools::Itertools;

pub struct Vertex {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Distance {
    from: String,
    to: String,
    distance: usize,
}


pub struct AdjacencyMatrix {
    name_lookup: HashMap<String, usize>,
    matrix: Vec<Vec<usize>>,
    distances: Vec<Distance>,
    cities: Vec<String>,
}

impl AdjacencyMatrix {
    fn new(distances: Vec<Distance>) -> AdjacencyMatrix {
        let cities = AdjacencyMatrix::uniq_cities(&distances);

        let name_lookup: HashMap<_, _> = cities
            .iter()
            .enumerate()
            .map(|(i, c)| (c.to_string(), i))
            .collect();

        let matrix_size = cities.len();
        let mut matrix: Vec<Vec<usize>> = vec![vec![usize::MAX; matrix_size]; matrix_size];

        for distance in &distances {
            let from_idx = *name_lookup.get(&distance.from).unwrap();
            let to_idx = *name_lookup.get(&distance.to).unwrap();
            matrix[from_idx][to_idx] = distance.distance;
            matrix[to_idx][from_idx] = distance.distance;
        }

        AdjacencyMatrix { name_lookup, matrix, distances, cities }
    }

    fn uniq_cities(distances: &Vec<Distance>) -> Vec<String> {
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

    fn distance(&self, from: usize, to: usize) -> usize {
        self.matrix[from][to]
    }

    fn distance_cities(&self, from: &str, to: &str) -> usize {
        self.distance(
            self.name_lookup.get(from).unwrap().clone(),
            self.name_lookup.get(to).unwrap().clone(),
        )
    }

    fn get_tour_cost(&self, cities: Vec<&str>) -> usize {
        cities
            .windows(2)
            .map(|pair| self.distance_cities(&pair[0], &pair[1]))
            .sum()
    }

    fn solve_max(&self) {
        let mut best = 0;
        for perm in self.cities.iter().permutations(self.cities.len()).unique() {
            let v2 = perm.iter().map(|s| s.as_str()).collect();
            let tour = self.get_tour_cost(v2);
            if tour > best {
                best = tour;
            }
        }
        println!("Longest route: {}", best)
    }

    fn solve_min(&self) {
        let mut best = usize::MAX;
        for perm in self.cities.iter().permutations(self.cities.len()).unique() {
            let v2 = perm.iter().map(|s| s.as_str()).collect();
            let tour = self.get_tour_cost(v2);
            if tour < best {
                best = tour;
            }
        }
        println!("Shortest route: {}", best)
    }
}

pub fn run(input: &Vec<String>) {
    let distances = parse_input(input);
    let matrix = AdjacencyMatrix::new(distances);
    matrix.solve_min();
    matrix.solve_max();
}


fn parse_input(input: &Vec<String>) -> Vec<Distance> {
    input.iter()
        .map(|line| {
            let tokens: Vec<&str> = line.split(" ").collect();
            let from = tokens[0].to_string();
            let to = tokens[2].to_string();
            let distance: usize = tokens[4].parse().unwrap();
            Distance { from, to, distance }
        })
        .collect()
}