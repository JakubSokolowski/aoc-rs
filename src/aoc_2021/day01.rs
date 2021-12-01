pub fn run(input: &[String]) {
    let as_num: Vec<usize> = input.iter().map(|i| i.parse::<usize>().unwrap()).collect();
    let count = count_increased(&as_num);
    println!("Num increased measurements: {}", count);

    let count_sliding = count_sliding_increased(&as_num);
    println!("Num sliding increased measurements: {}", count_sliding);
}

pub fn count_increased(input: &[usize]) -> usize {
    input
        .windows(2)
        .map(|w| if w[1] > w[0] { 1 } else { 0 })
        .sum()
}

pub fn count_sliding_increased(input: &[usize]) -> usize {
    input
        .windows(4)
        .map(|w| {
            let prev = w[0] + w[1] + w[2];
            let curr = w[1] + w[2] + w[3];
            if curr > prev {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_increasing_measurements() {
        // given
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // when
        let result = count_increased(&measurements);

        // then
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_counts_sliding_window_increasing_measurements() {
        // given
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // when
        let result = count_sliding_increased(&measurements);

        // then
        let expected = 5;
        assert_eq!(result, expected);
    }
}
