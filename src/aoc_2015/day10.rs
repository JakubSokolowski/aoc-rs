pub fn run(input: &str) {
    let mut seq = input.to_string();
    for i in 0..50 {
        let new_seq = look_and_say(&seq);
        seq = new_seq;
    }
    println!("seq len: {}", seq.len())
}

#[derive(Debug)]
struct Group {
    value: usize,
    symbol: char,
    count: usize,
}


pub fn look_and_say(input: &str) -> String {
    let mut prev = input.chars().nth(0).unwrap();
    let mut count = 1;
    let mut groups: Vec<Group> = vec![];

    for c in input.chars().skip(1) {
        if c != prev {
            let symbol = prev;
            let value = prev.to_string().parse().unwrap();
            let group = Group { value, symbol, count };
            groups.push(group);
            count = 1;
        } else {
            count += 1;
        }
        prev = c
    }

    let symbol = prev;
    let value = prev.to_string().parse().unwrap();
    let last = Group { count, symbol, value };
    groups.push(last);
    groups.iter().map(|g| transform_group(g)).collect()
}


fn transform_group(group: &Group) -> String {
    format!("{}{}", group.count, group.symbol)
}