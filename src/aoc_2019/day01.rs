pub fn run(input: &[String]) {
    let modules: Vec<i32> = input.iter().map(|m| m.parse::<i32>().unwrap()).collect();

    let requirements = get_modules_requirements(&modules);
    println!("Module fuel requirements: {}", requirements);
    let total_requirements = get_total_fuel_requirements(&modules);
    println!("Total fuel requirements: {}", total_requirements);
}

fn get_modules_requirements(modules: &[i32]) -> i32 {
    modules.iter().map(|&m| get_module_fuel(m)).sum()
}

fn get_module_fuel(module: i32) -> i32 {
    (module as f64 / 3_f64).floor() as i32 - 2
}

fn get_module_total_fuel(module: i32) -> i32 {
    let module_fuel = get_module_fuel(module);
    module_fuel + get_fuel_for_fuel(0, module_fuel)
}

fn get_total_fuel_requirements(modules: &[i32]) -> i32 {
    modules.iter().map(|&m| get_module_total_fuel(m)).sum()
}

fn get_fuel_for_fuel(total: i32, curr: i32) -> i32 {
    let new_fuel = get_module_fuel(curr);
    if new_fuel <= 0 {
        total
    } else {
        get_fuel_for_fuel(total + new_fuel, new_fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_module_fuel_returns_2_for_12() {
        // given
        let module = 12;

        // when
        let result = get_module_fuel(module);

        // then
        let expected = 2;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_module_fuel_returns_33583_for_100756() {
        // given
        let module = 100756;

        // when
        let result = get_module_fuel(module);

        // then
        let expected = 33583;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_fuel_for_fuel_returns_2_for_14() {
        // given
        let fuel = 14;

        // when
        let result = get_fuel_for_fuel(0, fuel);

        // then
        let expected = 2;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_fuel_for_fuel_returns_50346_for_100756() {
        // given
        let fuel = 100756;

        // when
        let result = get_fuel_for_fuel(0, fuel);

        // then
        let expected = 50346;
        assert_eq!(expected, result);
    }
}
