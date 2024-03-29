#[derive(PartialEq, Eq, Debug)]
pub struct SubmarineCommand {
    command_type: SubmarineCommandType,
    value: i64,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SubmarineCommandType {
    Forward = 0,
    Up = 1,
    Down = 2,
}

fn total_area(commands: &[SubmarineCommand]) -> i64 {
    total_x(commands) * total_y(commands)
}

fn total_x(commands: &[SubmarineCommand]) -> i64 {
    commands
        .iter()
        .filter(|c| c.command_type == SubmarineCommandType::Forward)
        .map(|c| c.value)
        .sum()
}

fn total_y(commands: &[SubmarineCommand]) -> i64 {
    commands
        .iter()
        .filter(|c| c.command_type != SubmarineCommandType::Forward)
        .map(|c| {
            if c.command_type == SubmarineCommandType::Up {
                -c.value
            } else {
                c.value
            }
        })
        .sum()
}

fn total_aim_area(commands: &[SubmarineCommand]) -> i64 {
    let mut aim = 0_i64;
    let mut total_x = 0_i64;
    let mut total_y = 0_i64;

    for cmd in commands {
        match cmd.command_type {
            SubmarineCommandType::Forward => {
                total_x += cmd.value;
                total_y += cmd.value * aim;
            }
            SubmarineCommandType::Up => {
                aim -= cmd.value;
            }
            SubmarineCommandType::Down => {
                aim += cmd.value;
            }
        }
    }

    total_x * total_y
}

pub fn parse_command(cmd: &str) -> SubmarineCommand {
    let tokens: Vec<&str> = cmd.split(' ').collect();
    let command_str = tokens[0];
    let value_str = tokens[1];
    let value = value_str.parse::<i64>().unwrap();

    let command_type = match command_str {
        "forward" => SubmarineCommandType::Forward,
        "up" => SubmarineCommandType::Up,
        "down" => SubmarineCommandType::Down,
        _ => panic!("Could not match command {}", cmd),
    };

    SubmarineCommand {
        command_type,
        value,
    }
}

pub fn run(input: &[String]) {
    let commands: Vec<SubmarineCommand> = input.iter().map(|line| parse_command(line)).collect();
    println!("Commands: {:?}", commands);
    println!("Total area: {:?}", total_area(&commands));
    println!("Total aim area: {:?}", total_aim_area(&commands));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_commands() -> Vec<SubmarineCommand> {
        vec![
            SubmarineCommand {
                command_type: SubmarineCommandType::Forward,
                value: 5,
            },
            SubmarineCommand {
                command_type: SubmarineCommandType::Down,
                value: 5,
            },
            SubmarineCommand {
                command_type: SubmarineCommandType::Forward,
                value: 8,
            },
            SubmarineCommand {
                command_type: SubmarineCommandType::Up,
                value: 3,
            },
            SubmarineCommand {
                command_type: SubmarineCommandType::Down,
                value: 8,
            },
            SubmarineCommand {
                command_type: SubmarineCommandType::Forward,
                value: 2,
            },
        ]
    }

    #[test]
    fn test_total_area_returns_product_of_total_x_and_y() {
        // given
        let commands = mock_commands();

        // when
        let result = total_area(&commands);

        // then
        let expected = 150;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_total_aim_area_returns_product_of_total_x_and_y() {
        // given
        let commands = mock_commands();

        // when
        let result = total_aim_area(&commands);

        // then
        let expected = 900;
        assert_eq!(result, expected);
    }
}
