pub fn run(input: &[String]) {
    println!("Total surface: {}", get_total_surface(input));
    println!("Total ribbon: {}", get_total_ribbon(input));
}

fn get_total_surface(input: &[String]) -> u32 {
    let mut total_surface = 0;

    for line in input {
        let surface = parse_present(line).surface();
        total_surface += surface
    }

    total_surface
}

fn get_total_ribbon(input: &[String]) -> u32 {
    let mut total_ribbon = 0;

    for line in input {
        let ribbon = parse_present(line).ribbon();
        total_ribbon += ribbon
    }

    total_ribbon
}

fn parse_present(line: &str) -> Present {
    let token = 'x';

    let dimensions: Vec<u32> = line
        .trim()
        .split(token)
        .map(|dim| dim.parse().unwrap())
        .collect();

    Present {
        length: dimensions[0],
        width: dimensions[1],
        height: dimensions[2],
    }
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

pub trait Wrappable {
    fn surface(&self) -> u32;
    fn volume(&self) -> u32;
    fn smallest_perimeter(&self) -> u32;
    fn ribbon(&self) -> u32;
}

impl Wrappable for Present {
    fn surface(&self) -> u32 {
        let first_side = self.length * self.width;
        let second_side = self.width * self.height;
        let third_side = self.height * self.length;

        let slack = match [first_side, second_side, third_side].iter().min() {
            None => 0,
            Some(i) => *i,
        };

        [first_side * 2, second_side * 2, third_side * 2, slack]
            .iter()
            .sum()
    }

    fn volume(&self) -> u32 {
        self.length * self.height * self.width
    }

    fn smallest_perimeter(&self) -> u32 {
        let mut sorted = vec![self.width, self.height, self.length];
        sorted.sort_unstable();
        2 * sorted[0] + 2 * sorted[1]
    }

    fn ribbon(&self) -> u32 {
        let bow = self.smallest_perimeter();
        let wrapper = self.volume();
        bow + wrapper
    }
}
