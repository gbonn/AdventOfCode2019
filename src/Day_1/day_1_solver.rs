// For file IO to the input data file
use std::fs;

// Get the Day 1 Part 1 solution. Public entry point for main.rs.
pub fn get_day_1_part_1_solution() -> u32 {
    return get_naive_fuel_for_modules();
}

// Get the Day 1 Part 2 solution. Public entry point for main.rs.
pub fn get_day_1_part_2_solution() -> u32 {
    return get_full_fuel_for_modules();
}

// This function returns the total fuel required, including the fuel needed to
// launch the fuel! 
fn get_full_fuel_for_modules() -> u32 {
    let day_1_input_modules = get_day1_inputs();
    let mut total_fuel = 0;

    // AOC instructions dictate we calculate total fuel needed on a per-module
    // basis. Rounding makes this important.
    for module in day_1_input_modules {
        // Get the fuel required to launch the module
        let module_fuel = get_fuel_required(module);
        // Get the fuel required to launch that fuel
        let fuel_fuel = get_fuel_for_fuel(module_fuel);
        // Combine into a total fuel requirement for this module and add to 
        // running total.
        total_fuel = total_fuel + module_fuel + fuel_fuel;
    }

    return total_fuel;
}

// This function returns the total fuel required to launch the modules. It
// ignores the fuel required to launch the fuel.
fn get_naive_fuel_for_modules() -> u32 {
    let day_1_input_modules = get_day1_inputs();
    let mut total_fuel = 0;

    // AOC instructions dictate we calculate on a per-module basis. Rounding
    // makes this important.
    for module in day_1_input_modules {
        total_fuel += get_fuel_required(module);
    }

    return total_fuel;
}

// This function returns the amount of fuel required to launch a certain amount
// fuel. It recursively addresses the additional amount of fuel down to the
// "wishing really hard" cutoff.
fn get_fuel_for_fuel(fuel: u32) -> u32 {
    let fuel_needed = get_fuel_required(fuel);

    if fuel_needed <= 0 {
        return 0;
    } else {
        return fuel_needed + get_fuel_for_fuel(fuel_needed);
    }
}

// This function returns the amount of fuel needed to launch some mass.
fn get_fuel_required(mass: u32) -> u32 {
    // Less than 6 units of mass doesn't need fuel to launch.
    if mass < 6 {
        return 0;
    } else {
        return (mass / 3) - 2;
    }
}

// This function returns a vector of the Day 1 input module masses.
fn get_day1_inputs() -> Vec<u32> {
    let input_text = fs::read_to_string("src/Day_1/day_1_input.txt").expect("Failed to open input file.");
    let input_vec: Vec<u32> = input_text.split("\r\n").map(|input_value| input_value.parse::<u32>().expect("Failed to parse string to u32")).collect();

    return input_vec;
}

#[test]
fn get_fuel_required_test() {
    assert_eq!(get_fuel_required(9), 1);
    assert_eq!(get_fuel_required(7), 0);
    assert_eq!(get_fuel_required(6), 0);
    assert_eq!(get_fuel_required(5), 0);
    assert_eq!(get_fuel_required(12), 2);
    assert_eq!(get_fuel_required(14), 2);
    assert_eq!(get_fuel_required(1969), 654);
    assert_eq!(get_fuel_required(100756), 33583);
}

#[test]
fn get_fuel_for_fuel_test() {
    assert_eq!(get_fuel_for_fuel(2), 0);
    assert_eq!(get_fuel_for_fuel(654), 312);
    assert_eq!(get_fuel_for_fuel(33583), 16763);
}