pub fn run(input: &Vec<String>) {
    println!("part 2: {}", "xd");
}

#[derive(PartialEq)]
enum Light {
    Off = 0,
    On = 1
}

struct Grid {
    width: u32,
    height: u32,
    lights: Vec<Light>
}

#[derive(PartialEq)]
enum CommandType {
    Off = 0,
    On = 1,
    Toggle = 2
}

struct Command {
    cmd_type: CommandType,
    from: (usize, usize),
    to: (usize, usize)
}

impl Grid {
    pub fn new(w: u32, h: u32) -> Grid {
        let width = w;
        let height = h;

        let lights = (0..width * height)
            .map(|i| Light:: Off)
            .collect();

        Grid {
            width,
            height,
            lights
        }
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn toggle(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let light = &self.lights[idx];

        let new_light = match light {
            (Light::On) => Light::Off,
            (Light::Off) => Light::On
        };

        self.lights[idx] = new_light;
    }

    pub fn on(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let light = &self.lights[idx];

        let new_light = Light::On;
        self.lights[idx] = new_light
    }

    pub fn off(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let light = &self.lights[idx];

        let new_light = Light::Off;
        self.lights[idx] = new_light
    }

    pub fn count(&self, light: Light) -> usize {
        self.lights
            .iter()
            .count()
    }
}
