use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub fn run(input: &str) {
    let (bingo_line, matrices) = parse(input);

    play_bingo(&bingo_line, matrices.clone());
    play_bingo_last(&bingo_line, matrices);
}

fn play_bingo(bingo_line: &[usize], mut matrices: Vec<BingoMatrix>) {
    for num in bingo_line {
        for m in matrices.iter_mut() {
            m.mark(*num);
            if m.has_bingo() {
                println!("Found bingo: {}", m.sum_unmarked() * num);
                return;
            }
        }
    }
}

fn play_bingo_last(bingo_line: &[usize], mut matrices: Vec<BingoMatrix>) {
    let mut num_winners = 0;
    let num_players = matrices.len();
    let mut solved: HashSet<usize> = HashSet::new();

    for num in bingo_line {
        for m in matrices.iter_mut().enumerate() {
            m.1.mark(*num);
            if m.1.has_bingo() && !solved.contains(&m.0) {
                num_winners += 1;
                solved.insert(m.0);
                println!("Found bingo: {}", m.1.sum_unmarked() * num);
                if num_winners == num_players {
                    println!("Last winner: {}", m.1.sum_unmarked() * num);
                    return;
                }
            }
        }
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<BingoMatrix>) {
    let bingo_line: Vec<usize> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let matrices: Vec<BingoMatrix> = input.split("\n\n").skip(1).map(parse_matrix).collect();

    (bingo_line, matrices)
}

fn parse_matrix(input: &str) -> BingoMatrix {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let values: Vec<usize> = RE
        .find_iter(input)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();

    BingoMatrix::new(values)
}

#[derive(Debug, Clone)]
struct BingoMatrix {
    width: usize,
    height: usize,
    values: Vec<BingoNum>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct BingoNum {
    marked: bool,
    value: usize,
}

impl BingoMatrix {
    pub fn new(values: Vec<usize>) -> BingoMatrix {
        let width = (values.len() as f64).sqrt() as usize;
        let height = width;

        BingoMatrix {
            width,
            height,
            values: values
                .into_iter()
                .map(|v| BingoNum {
                    value: v,
                    marked: false,
                })
                .collect(),
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_value(&self, row: usize, column: usize) -> BingoNum {
        match self.values.get(self.get_index(row, column)) {
            Some(v) => *v,
            None => BingoNum {
                value: 0,
                marked: false,
            },
        }
    }

    fn mark(&mut self, number: usize) {
        match self.values.iter().position(|n| n.value == number) {
            None => {}
            Some(index) => self.values.get_mut(index).unwrap().marked = true,
        };
    }

    fn any_column_has_bingo(&self) -> bool {
        (0..self.width)
            .map(|column| self.column_has_bingo(column))
            .any(|bingo| bingo)
    }

    fn column_has_bingo(&self, column: usize) -> bool {
        (0..self.height)
            .map(|row| self.get_value(row, column))
            .all(|n| n.marked)
    }

    fn any_row_has_bingo(&self) -> bool {
        (0..self.height)
            .map(|column| self.row_has_bingo(column))
            .any(|bingo| bingo)
    }

    fn row_has_bingo(&self, row: usize) -> bool {
        (0..self.width)
            .map(|column| self.get_value(row, column))
            .all(|n| n.marked)
    }

    fn has_bingo(&self) -> bool {
        self.any_row_has_bingo() || self.any_column_has_bingo()
    }

    fn sum_unmarked(&self) -> usize {
        self.values
            .iter()
            .filter_map(|&n| if !n.marked { Some(n.value) } else { None })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_marks_value_as_bingoed() {
        // given
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut bingo_matrix = BingoMatrix::new(values);

        // when
        bingo_matrix.mark(3);
        let result = bingo_matrix.get_value(0, 2);

        // then
        let expected = BingoNum {
            value: 3,
            marked: true,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn any_row_has_bingo_returns_returns_true_if_all_nums_in_some_row_are_marked() {
        // given
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut bingo_matrix = BingoMatrix::new(values);

        // when
        bingo_matrix.mark(4);
        bingo_matrix.mark(5);
        bingo_matrix.mark(6);
        let row_bingo = bingo_matrix.any_row_has_bingo();
        let column_bingo = bingo_matrix.any_column_has_bingo();

        // then
        let expected_row = true;
        let expected_column = false;
        assert_eq!(row_bingo, expected_row);
        assert_eq!(column_bingo, expected_column);
    }

    #[test]
    fn any_row_has_bingo_returns_returns_true_if_all_nums_in_some_column_are_marked() {
        // given
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut bingo_matrix = BingoMatrix::new(values);

        // when
        bingo_matrix.mark(3);
        bingo_matrix.mark(6);
        bingo_matrix.mark(9);
        let row_bingo = bingo_matrix.any_row_has_bingo();
        let column_bingo = bingo_matrix.any_column_has_bingo();

        // then
        let expected_row = false;
        let expected_column = true;
        assert_eq!(row_bingo, expected_row);
        assert_eq!(column_bingo, expected_column);
    }
}
