use crate::common::parse::parse_numbers;
use itertools::Itertools;

pub fn run(input: &str) {
    let first_program = parse_numbers(input);
    let second_program = parse_numbers(input);
    let result = run_program(first_program);
    println!("Program result: {}", result);
    let solution = solve_for_value(second_program, 19690720);
    println!(
        "Got solution: {:?}, result value: {}",
        solution,
        100 * solution.0 + solution.1
    );
}

pub fn solve_for_value(program: Vec<i64>, value: i64) -> (i64, i64) {
    // purely on basis that .get() starts to returns none for positions
    // that are greater than program.len(), limit the number for word/verb
    // to program.len()
    let bound = program.len() as i64;
    let inputs = (0..=bound).cartesian_product(0..bound);

    for (noun, verb) in inputs {
        let result = run_parametrized(program.clone(), noun, verb);
        if result == value {
            return (noun, verb);
        }
    }

    (0, 0)
}

pub fn run_parametrized(mut program: Vec<i64>, noun: i64, verb: i64) -> i64 {
    program[1] = noun;
    program[2] = verb;
    run_program(program)
}

pub fn run_program(mut program: Vec<i64>) -> i64 {
    let mut opcode_position = 0;

    loop {
        let opcode = *program.get(opcode_position).unwrap();

        if opcode == 99 {
            // halt
            break;
        }

        let first_pos = *program.get(opcode_position + 1).unwrap();
        let second_pos = *program.get(opcode_position + 2).unwrap();
        let result_pos = *program.get(opcode_position + 3).unwrap();

        let first_op = *program.get(first_pos as usize).unwrap();
        let second_op = *program.get(second_pos as usize).unwrap();

        match opcode {
            1 => {
                // add ops at first_pos and second_pos, store at result_pos
                program[result_pos as usize] = first_op + second_op;
            }
            2 => {
                // multiplies ops at first_pos and second_pos, store at result_pos
                program[result_pos as usize] = first_op * second_op;
            }
            _ => {
                panic!("Got invalid opcode: {}", opcode);
            }
        }

        opcode_position += 4
    }

    *program.get(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program_example_1() {
        // given
        let program: Vec<i64> = vec![1, 0, 0, 0, 99];

        // when
        let result = run_program(program);

        // them
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_example_2() {
        // given
        let program: Vec<i64> = vec![2, 3, 0, 3, 99];

        // when
        let result = run_program(program);

        // them
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_example_3() {
        // given
        let program: Vec<i64> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        // when
        let result = run_program(program);

        // them
        let expected = 30;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_example_4() {
        // given
        let program: Vec<i64> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        // when
        let result = run_program(program);

        // them
        let expected = 3500;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_for_value_example_1() {
        // given
        let program: Vec<i64> = vec![1, 0, 0, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let value = 3500;

        // when
        let result = solve_for_value(program, value);

        // them
        let expected = (9, 10);
        assert_eq!(result, expected);
    }
}
