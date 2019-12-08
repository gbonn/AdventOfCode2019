// Bring the individual solvers in
mod day_1;
mod day_2;

fn main() {
    println!("Day 1 part 1 solution: {}", day_1::day_1_solver::get_day_1_part_1_solution());
    println!("Day 1 part 2 solution: {}", day_1::day_1_solver::get_day_1_part_2_solution());
    println!("Day 2 part 1 solution: {}", day_2::day_2_solver::get_day_2_part_1_solution());
    println!("Day 2 part 2 solution: {}", day_2::day_2_solver::get_day_2_part_2_solution());
}
