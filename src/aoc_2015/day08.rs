use std::str;
use onig::Regex;


pub fn run(input: &String) {
    println!("Number of extra characters: {}", extra_chars_unescaped(input));
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn to_escape() -> Vec<&'static str> {
    let base = vec!["\\\"", "\\\\"];
    return base
}

pub fn raw_and_unescaped_len(s: &str) -> (usize, usize) {
    if s.chars().nth(0) != Some('"') || s.chars().last() != Some('"') {
        panic!("invalid format (not quoted)");
    }
    let raw_len = s.len();
    let re = Regex::new(r#"\\(\\|"|x[0-9a-f]{2})"#).unwrap();
    let ss = &s[1..s.len()-1];
    let (esc_count, esc_size) = re.find_iter(ss).fold((0, 0), |(esc_count, esc_size), (start_pos, end_pos)| {
        (esc_count + 1, esc_size + (end_pos - start_pos))
    });
    (raw_len, raw_len - 2 - esc_size + esc_count)
}

fn strip_escape(value: &str) -> String {
    let removed = rem_first_and_last(value);
    let plain_bytes = strip_ansi_escapes::strip(removed.as_bytes()).unwrap();
    let res = str::from_utf8(&plain_bytes).unwrap();
    println!("Stripped: {}", res);
    return res.to_string();
}

fn len_diff_2(value: &str) -> usize {
    let char_len = value.len();
    let stripped_len = strip_escape(value).len();
    return char_len - stripped_len;
}

fn len_diff(value: &str) -> usize {
    let char_len = value.len();
    let stripped = rem_first_and_last(value);
    let mut escaped: String = stripped.to_string();
    let literals: Vec<String> = ( 0x20..=0x7e).map(|c| format!("\\x{:x}", c)).collect();

    for token in literals {
        escaped = escaped.replace(&token, "?")
    }

    for token in to_escape() {
        escaped = escaped.replace(token, "?")
    }

    return char_len - escaped.len()
}

pub fn extra_chars_unescaped(text: &str) -> usize {
    text.lines().fold(0, |extra_chars, line| {
        let (raw_len, unescaped_len) = raw_and_unescaped_len(line);
        extra_chars + (raw_len - unescaped_len)
    })
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_with_escaped_ascii() {
        // given
        let raw = r#""\x27""#;

        // when
        let res = len_diff(raw);

        // then
        let expected = 1;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_len_with_escaped_quotes() {
        // given
        let raw =r#""aaa\"aaa""#;

        // when
        let res = len_diff(raw);

        // then
        let expected = 7;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_len_with_escaped_slash() {
        // given
        let raw =r#""kwd\\""#;

        // when
        let res = len_diff(raw);

        // then
        let expected = 4;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_len_nothing_to_escape() {
        // given
        let raw ="\"nq\"";

        // when
        let res = len_diff(raw);

        // then
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_multiple_slashes() {
        // given
        let raw =r#""\"\"r""#;
        // when
        let res = len_diff(raw);

        // then
        let expected = 2;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_hex_slashes() {
        // given
        let raw =r#""\x22v\\""#;
        // when
        let res = len_diff(raw);

        // then
        let expected = 2;
        assert_eq!(res, expected);
    }

}
