use onig::Regex;
use std::str;

pub fn run(input: &str) {
    println!(
        "Number of extra characters: {}",
        extra_chars_unescaped(input)
    );
}

pub fn raw_and_unescaped_len(s: &str) -> (usize, usize) {
    if !s.starts_with('"') || !s.ends_with('"') {
        panic!("invalid format (not quoted)");
    }
    let raw_len = s.len();
    let re = Regex::new(r#"\\(\\|"|x[0-9a-f]{2})"#).unwrap();
    let ss = &s[1..s.len() - 1];
    let (esc_count, esc_size) =
        re.find_iter(ss)
            .fold((0, 0), |(esc_count, esc_size), (start_pos, end_pos)| {
                (esc_count + 1, esc_size + (end_pos - start_pos))
            });
    (raw_len, raw_len - 2 - esc_size + esc_count)
}

pub fn extra_chars_unescaped(text: &str) -> usize {
    text.lines().fold(0, |extra_chars, line| {
        let (raw_len, unescaped_len) = raw_and_unescaped_len(line);
        extra_chars + (raw_len - unescaped_len)
    })
}
