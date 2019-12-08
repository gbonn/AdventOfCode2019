use std::fs;

fn main() {
    let day_1_input_masses = get_day1_inputs();
    let mut total_mass = 0;

    for mass in day_1_input_masses {
        total_mass += calc_fuel_needed(mass);
    }

    println!("Total mass: {}", total_mass);
}

fn calc_fuel_needed(mass: u32) -> u32 {
    return (mass / 3) - 2;
}

fn get_day1_inputs() -> Vec<u32> {
    let input_text = fs::read_to_string("src/Day_1/day_1_input.txt").unwrap();
    let input_vec: Vec<u32> = input_text.split("\r\n").map(|input_value| input_value.parse::<u32>().unwrap()).collect();

    return input_vec;
}

#[test]
fn calc_fuel_needed_test() {
    assert_eq!(calc_fuel_needed(12), 2);
    assert_eq!(calc_fuel_needed(14), 2);
    assert_eq!(calc_fuel_needed(1969), 654);
    assert_eq!(calc_fuel_needed(100756), 33583);
}
