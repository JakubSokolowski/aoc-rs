use std::cmp::{max, min};

pub fn run(input: &[String]) {
    let cmds: Vec<Command> = input.iter().map(|line| parse_command(line)).collect();

    let mut first_grid = BinaryGrid::new(1000, 1000);
    first_grid.apply_commands(&cmds);
    let count = first_grid.count(Light::On);
    println!("Got {} lights on", count);

    let mut second_grid = ValueGrid::new(1000, 1000);
    second_grid.apply_commands(&cmds);
    let brightness = second_grid.birghtness();
    println!("Got {} brightness", brightness);
}

#[derive(PartialEq)]
enum Light {
    Off = 0,
    On = 1,
}

struct BinaryGrid {
    width: usize,
    lights: Vec<Light>,
}

struct ValueGrid {
    width: usize,
    lights: Vec<usize>,
}

#[derive(PartialEq, Debug)]
enum CommandType {
    Off = 0,
    On = 1,
    Toggle = 2,
}

#[derive(Debug)]
pub struct Command {
    cmd_type: CommandType,
    from: (usize, usize),
    to: (usize, usize),
}

impl BinaryGrid {
    pub fn new(w: usize, h: usize) -> BinaryGrid {
        let width = w;
        let height = h;

        let lights = (0..width * height).map(|_i| Light::Off).collect();

        BinaryGrid { width, lights }
    }

    pub fn apply_commands(&mut self, cmds: &[Command]) {
        for cmd in cmds {
            self.apply_command(cmd)
        }
    }

    pub fn apply_command(&mut self, cmd: &Command) {
        let start_row = min(cmd.from.0, cmd.to.0);
        let end_row = max(cmd.from.0, cmd.to.0);

        let start_column = min(cmd.from.1, cmd.to.1);
        let end_column = max(cmd.from.1, cmd.to.1);

        for x in start_row..=end_row {
            for y in start_column..=end_column {
                self.apply_to_coord(x, y, cmd);
            }
        }
    }

    pub fn apply_to_coord(&mut self, row: usize, column: usize, cmd: &Command) {
        match cmd.cmd_type {
            CommandType::Off => self.off(row, column),
            CommandType::On => self.on(row, column),
            CommandType::Toggle => self.toggle(row, column),
        }
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn toggle(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        let light = &self.lights[idx];

        let new_light = match light {
            Light::On => Light::Off,
            Light::Off => Light::On,
        };

        self.lights[idx] = new_light;
    }

    pub fn on(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        let new_light = Light::On;
        self.lights[idx] = new_light
    }

    pub fn off(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        let new_light = Light::Off;
        self.lights[idx] = new_light
    }

    pub fn count(&self, light: Light) -> usize {
        self.lights.iter().filter(|l| **l == light).count()
    }
}

impl ValueGrid {
    pub fn new(w: usize, h: usize) -> ValueGrid {
        let width = w;
        let height = h;

        let lights: Vec<usize> = (0..width * height).map(|_i| 0).collect();

        ValueGrid { width, lights }
    }

    pub fn apply_commands(&mut self, cmds: &[Command]) {
        for cmd in cmds {
            self.apply_command(cmd)
        }
    }

    pub fn apply_command(&mut self, cmd: &Command) {
        let start_row = min(cmd.from.0, cmd.to.0);
        let end_row = max(cmd.from.0, cmd.to.0);

        let start_column = min(cmd.from.1, cmd.to.1);
        let end_column = max(cmd.from.1, cmd.to.1);

        for x in start_row..=end_row {
            for y in start_column..=end_column {
                self.apply_to_coord(x, y, cmd);
            }
        }
    }

    pub fn apply_to_coord(&mut self, row: usize, column: usize, cmd: &Command) {
        match cmd.cmd_type {
            CommandType::Off => self.off(row, column),
            CommandType::On => self.on(row, column),
            CommandType::Toggle => self.toggle(row, column),
        }
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn toggle(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        let light = &self.lights[idx];
        self.lights[idx] = light + 2;
    }

    pub fn on(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        let light = &self.lights[idx];
        self.lights[idx] = light + 1
    }

    pub fn off(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        let light = &self.lights[idx];
        if light == &0 {
            return;
        }
        let new_light = light - 1;
        self.lights[idx] = new_light;
    }

    pub fn birghtness(&self) -> usize {
        self.lights.iter().sum()
    }
}

pub fn parse_command(cmd: &str) -> Command {
    let start_token = &cmd[..7];

    match start_token {
        "turn on" => parse_turn_on(cmd),
        "turn of" => parse_turn_off(cmd),
        "toggle " => parse_toggle(cmd),
        _ => panic!("Could not match command {}", cmd),
    }
}

fn parse_turn_on(cmd: &str) -> Command {
    let tokens: Vec<&str> = cmd.split(' ').collect();
    let from = &tokens[2];
    let to = &tokens[4];

    Command {
        cmd_type: CommandType::On,
        from: parse_coords(from),
        to: parse_coords(to),
    }
}

fn parse_coords(coords: &str) -> (usize, usize) {
    let split: Vec<&str> = coords.split(',').collect();
    let x = split[0].parse::<usize>().unwrap();
    let y = split[1].parse::<usize>().unwrap();

    (x, y)
}

fn parse_turn_off(cmd: &str) -> Command {
    let tokens: Vec<&str> = cmd.split(' ').collect();
    let from = &tokens[2];
    let to = &tokens[4];

    Command {
        cmd_type: CommandType::Off,
        from: parse_coords(from),
        to: parse_coords(to),
    }
}

fn parse_toggle(cmd: &str) -> Command {
    let tokens: Vec<&str> = cmd.split(' ').collect();
    let from = tokens[1];
    let to = tokens[3];

    Command {
        cmd_type: CommandType::Toggle,
        from: parse_coords(from),
        to: parse_coords(to),
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_2015::day06::{BinaryGrid, Command, CommandType, Light};

    #[test]
    fn test_all_on_cmd() {
        // given
        let w = 1000;
        let h = 1000;
        let mut grid = BinaryGrid::new(w, h);
        let cmd = Command {
            cmd_type: CommandType::On,
            from: (0, 0),
            to: (999, 999),
        };

        // when
        grid.apply_command(&cmd);
        let count = grid.count(Light::On);
        // then
        let expected = w * h;
        assert_eq!(count, expected);
    }
}
