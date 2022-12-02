pub fn get_move_value(m: &str) -> i64 {
    match m {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Invalid"),
    }
}

pub fn get_result(opponent_move: &str, your_move: &str) -> i64 {
    match (opponent_move, your_move) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,

        (_, _) => panic!("Invalid input {} {}", opponent_move, your_move),
    }
}

pub fn run(input: &[String]) {
    println!("Here be day 2");
    part_2(input);
}

pub fn part_1(input: &[String]) {
    println!("Here be day 2");

    let mut result = 0;
    for line in input {
        let tokens: Vec<&str> = line.split(' ').collect();
        let opponent_move = &tokens[0];
        let your_move = &tokens[1];
        let move_score = get_move_value(your_move);
        let result_score = get_result(opponent_move, your_move);
        println!(
            "OP: {} YOU: {} MOVE {} RESULT {}, TOTAL {}",
            opponent_move,
            your_move,
            move_score,
            result_score,
            move_score + result_score
        );
        result += move_score;
        result += result_score;
    }

    println!("Total: {}", result);
}

pub fn get_move(opponent: &str, strategy: &str) -> String {
    match (opponent, strategy) {
        ("A", "X") => "Z".to_string(),
        ("A", "Y") => "X".to_string(),
        ("A", "Z") => "Y".to_string(),
        ("B", "X") => "X".to_string(),
        ("B", "Y") => "Y".to_string(),
        ("B", "Z") => "Z".to_string(),
        ("C", "X") => "Y".to_string(),
        ("C", "Y") => "Z".to_string(),
        ("C", "Z") => "X".to_string(),

        (_, _) => panic!("Invalid input {} {}", opponent, strategy),
    }
}

pub fn get_res_value(res: &str) -> i64 {
    match res {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Invalid"),
    }
}

pub fn part_2(input: &[String]) {
    println!("Here be day 2");

    let mut result = 0;
    for line in input {
        let tokens: Vec<&str> = line.split(' ').collect();
        let opponent_move = &tokens[0];
        let strategy = &tokens[1];
        let your_move = get_move(opponent_move, strategy);
        let sum = get_res_value(strategy) + get_move_value(&your_move);
        result += sum;
        println!(
            "OP: {} ST: {} YR: {} RES: {} MOV: {}",
            opponent_move,
            strategy,
            your_move,
            get_res_value(strategy),
            get_move_value(&your_move)
        );
    }

    println!("Total: {}", result);
}
