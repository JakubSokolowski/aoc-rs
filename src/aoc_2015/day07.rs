use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn run(input: &[String]) {
    let mut solver = SignalSolver::new();
    solver.solve(input, "a")
}

struct SignalSolver {
    lookup: HashMap<String, u16>,
}

impl SignalSolver {
    pub fn new() -> SignalSolver {
        let lookup: HashMap<String, u16> = HashMap::new();

        SignalSolver { lookup }
    }

    pub fn solve(&mut self, input: &[String], unknown: &str) {
        let mut unsolved: HashSet<&String> = HashSet::from_iter(input.iter());

        println!("{:?}", unsolved);
        let mut iter_num = 1;
        loop {
            println!("Iteration: {}", iter_num);
            iter_num += 1;
            let mut to_remove: Vec<&String> = vec![];

            for line in &unsolved {
                let separator = "->";
                let tokens: Vec<&str> = line.trim().split(separator).collect();

                let input = tokens[0];
                let output_signal = tokens[1].trim();

                let result = self.eval(input);

                if let Some(r) = result {
                    println!("Solved {}={}", output_signal, r);
                    self.lookup.insert(output_signal.to_string(), r);
                    to_remove.push(line);
                }
            }

            for el in to_remove {
                unsolved.remove(el);
            }

            if let Some(s) = self.lookup.get(unknown) {
                println!("Found solution! {} = {}", unknown, s);
                break;
            }
        }
    }

    pub fn eval(&self, input: &str) -> Option<u16> {
        let tokens: Vec<&str> = input.trim().split(' ').collect();

        return match tokens.len() {
            3 => self.eval_double(tokens[0].trim(), tokens[1].trim(), tokens[2].trim()),
            2 => self.eval_single(tokens[1].trim(), tokens[0].trim()),
            1 => self.eval_direct(tokens[0].trim()),
            _ => panic!("Cannot eval input: {}", input),
        };
    }

    fn eval_double(&self, x: &str, op: &str, y: &str) -> Option<u16> {
        let x_val = self.eval_direct(x);
        let y_val = self.eval_direct(y);

        if let (Some(x), Some(y)) = (x_val, y_val) {
            return Some(double_op_bitwise(op, x, y));
        }
        None
    }

    fn eval_single(&self, x: &str, op: &str) -> Option<u16> {
        let x_val = self.eval_direct(x);
        x_val.map(|x| single_op_bitwise(op, x))
    }

    fn eval_direct(&self, x: &str) -> Option<u16> {
        return match x.parse::<u16>() {
            Ok(n) => Some(n),
            Err(_e) => {
                return self.lookup.get(x).cloned();
            }
        };
    }
}

pub fn double_op_bitwise(op: &str, op1: u16, op2: u16) -> u16 {
    match op {
        "AND" => op1 & op2,
        "OR" => op1 | op2,
        "RSHIFT" => op1 >> op2,
        "LSHIFT" => op1 << op2,
        _ => panic!("Invalid operation: {}", op),
    }
}

pub fn single_op_bitwise(op: &str, op1: u16) -> u16 {
    match op {
        "NOT" => !op1,
        _ => panic!("Invalid operation: {}", op),
    }
}
