fn main() {
    println!("{}", calc_fuel_needed(12));
}

fn calc_fuel_needed(mass: i32) -> i32 {
    return (mass / 3) - 2;
}

#[test]
fn calc_fuel_needed_test() {
    assert_eq!(calc_fuel_needed(12), 2);
    assert_eq!(calc_fuel_needed(14), 2);
    assert_eq!(calc_fuel_needed(1969), 654);
    assert_eq!(calc_fuel_needed(100756), 33583);
}