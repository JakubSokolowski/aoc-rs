pub fn run(input: &str) {
    let floor = get_floor(input);
    let basement_position = get_basement_position(input);

    println!("Floor: {}, Position: {}", floor, basement_position);
}

fn get_floor(input: &str) -> i64 {
    let up_token = '(';
    let down_token = ')';

    let down_count: i64 = input.matches(down_token).count() as i64;
    let up_count: i64 = input.matches(up_token).count() as i64;
    let floor: i64 = up_count - down_count;

    floor
}

fn get_basement_position(input: &str) -> i64 {
    let mut current_floor = 0;
    let basement = -1;
    let mut position = 1;

    for c in input.chars() {
        let change = match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        current_floor += change;
        position += 1;

        if current_floor == basement {
            println!("Found basement at {}", position);
            break;
        }
    }

    position
}
