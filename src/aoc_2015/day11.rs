use itertools::Itertools;

pub fn run(input: &str) {
    println!("input: {}", input);

    let mut current = input.to_string();
    let mut iter = 1;

    loop {
        let new_pwd = increment_password(current.to_string());
        current = new_pwd.clone();
        println!("Iter: {}. Checking: {}", iter, current);
        iter += 1;
        if let true = valid_password(current.as_str()) {
            break;
        }
    }
}

fn next_in_alphabet(c: char) -> char {
    match c {
        'z' => 'a',
        _ => std::char::from_u32(c as u32 + 1).unwrap_or(c),
    }
}

fn increment_password(input: String) -> String {
    let mut res: String = String::from("");

    let mut keep_adding = true;
    for c in input.chars().rev() {
        if keep_adding {
            let next = next_in_alphabet(c);
            res.push(next);
            if next != 'a' {
                keep_adding = false
            }
        } else {
            res.push(c)
        }
    }

    res.chars().rev().collect::<String>()
}

fn valid_password(input: &str) -> bool {
    no_forbidden_letters(input) && has_incrementing_triplet(input) && has_different_pairs(input)
}

fn has_incrementing_triplet(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|w| is_triplet(w.iter().collect::<String>()))
}

fn has_different_pairs(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter(|c| c[0] == c[1])
        .unique()
        .count()
        >= 2
}

fn no_forbidden_letters(input: &str) -> bool {
    !["i", "o", "l"]
        .iter()
        .any(|c| input.contains(&c.to_string()))
}

fn is_triplet(input: String) -> bool {
    input.chars().next().unwrap() as u32 + 1 == input.chars().nth(1).unwrap() as u32
        && input.chars().nth(1).unwrap() as u32 + 1 == input.chars().nth(2).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use crate::aoc_2015::day11::{
        has_different_pairs, has_incrementing_triplet, increment_password, is_triplet,
        next_in_alphabet, no_forbidden_letters,
    };

    #[test]
    fn test_next_in_alphabet_returns_proper_next_letter() {
        // given
        let letter = 'a';

        // when
        let result = next_in_alphabet(letter);

        // then
        let expected = 'b';
        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_in_alphabet_returns_y_for_x() {
        // given
        let letter = 'x';

        // when
        let result = next_in_alphabet(letter);

        // then
        let expected = 'y';
        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_in_alphabet_wraps_around_to_a_from_z() {
        // given
        let letter = 'z';

        // when
        let result = next_in_alphabet(letter);

        // then
        let expected = 'a';
        assert_eq!(result, expected);
    }

    #[test]
    fn test_increment_str_returns_proper_str() {
        // given
        let input = "xx".to_string();

        // when
        let result = increment_password(input);

        // then
        let expected = "xy";
        assert_eq!(result.as_str(), expected);
    }

    #[test]
    fn test_increment_str_returns_ya_for_xz() {
        // given
        let input = "xz".to_string();

        // when
        let result = increment_password(input);

        // then
        let expected = "ya";
        assert_eq!(result.as_str(), expected);
    }

    #[test]
    fn test_is_triplet_returns_true_for_aaa() {
        // given
        let input = "aaa".to_string();

        // when
        let result = is_triplet(input);

        // then
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_triplet_returns_true_for_abc() {
        // given
        let input = "abc".to_string();

        // when
        let result = is_triplet(input);

        // then
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_triplet() {
        // given
        let input = "abc";

        // when
        let result = has_incrementing_triplet(input);

        // then
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_triplet_full_pwd() {
        // given
        let input = "dffaaabc";

        // when
        let result = has_incrementing_triplet(input);

        // then
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_triplet_full_no_triplet() {
        // given
        let input = "dffaaabd";

        // when
        let result = has_incrementing_triplet(input);

        // then
        assert_eq!(result, false);
    }

    #[test]
    fn test_has_different_pairs_some_pairs() {
        // given
        let input = "abbceffg";

        // when
        let result = has_different_pairs(input);

        // then
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_different_pairs_no_pairs() {
        // given
        let input = "abbbefeg";

        // when
        let result = has_different_pairs(input);

        // then
        assert_eq!(result, false);
    }

    #[test]
    fn test_no_forbidden_has_some_forbidden() {
        // given
        let input = "ibbbefeg";

        // when
        let result = no_forbidden_letters(input);

        // then
        assert_eq!(result, false);
    }

    #[test]
    fn test_no_forbidden_has_only_allowed() {
        // given
        let input = "abbbefeg";

        // when
        let result = no_forbidden_letters(input);

        // then
        assert_eq!(result, true);
    }
}
