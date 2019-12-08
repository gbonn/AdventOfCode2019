// For file IO to the input data file
use std::fs;

// Get the Day 2 Part 1 solution. Public entry point for main.rs.
pub fn get_day_2_part_1_solution() -> u32 {

    let mut input_program = get_day_2_inputs();
    process_intcode_prog(input_program.as_mut_slice());

    return input_program[0];
}

// Get the Day 2 Part 2 solution. Public entry point for main.rs.
pub fn get_day_2_part_2_solution() -> u32 {
    return brute_force_gravity_assist();
}

// This function walks through the intcode program
fn process_intcode_prog (input_program: &mut [u32]) {

    // Jump by 4 positions per instruction
    for index in (0..input_program.len()).step_by(4) {
        match input_program[index] {
            1 => process_add_opcode(index, input_program),
            2 => process_mult_opcode(index, input_program),
            99 => break,
            _ => println!("ERROR!")
        }
    }
}

// This function processes the addition opcode
fn process_add_opcode (opcode_index: usize, input_program: &mut [u32]) {
    input_program[input_program[opcode_index + 3] as usize] = 
        input_program[input_program[opcode_index + 1] as usize] + 
        input_program[input_program[opcode_index + 2] as usize];
}

// This function processes the multiplication opcode
fn process_mult_opcode (opcode_index: usize, input_program: &mut [u32]) {
    input_program[input_program[opcode_index + 3] as usize] = 
        input_program[input_program[opcode_index + 1] as usize] * 
        input_program[input_program[opcode_index + 2] as usize];
}

// We want a specific result. We could build up a breadcrumb list and walk back
// through the execution stack, but it's so much easier to brute force. This
// would fall apart if they opened up the input space.
fn brute_force_gravity_assist () -> u32 {
    let mut formatted_result = 0;

    for noun in 0..99 {
        for verb in 0..99 {
            // Loooots of IO. This would benefit from caching...
            let mut input_program = get_day_2_inputs();

            // Try out the new noun/verb pair
            input_program[1] = noun;
            input_program[2] = verb;
            process_intcode_prog(input_program.as_mut_slice());

            // Check for the magic gravity assist number
            if input_program[0] == 19690720 {
                // Log our special numbers
                formatted_result = 100 * noun + verb;
                break;
            }
        }
    }

    return formatted_result;
}

// This function returns a vector of the Day 2 inputs.
fn get_day_2_inputs() -> Vec<u32> {
    let input_text = fs::read_to_string("src/Day_2/day_2_input.txt").expect("Failed to open input file.");
    let input_vec: Vec<u32> = input_text.split(",").map(|input_value| input_value.parse::<u32>().expect("Failed to parse string to u32")).collect();

    return input_vec;
}

#[test]
fn process_intcode_prog_test() {
    let mut test_program = vec![1, 0, 0, 0, 99];
    process_intcode_prog(test_program.as_mut_slice());
    assert_eq!(test_program, vec![2, 0, 0, 0, 99]);

    test_program = vec![2, 3, 0, 3, 99];
    process_intcode_prog(test_program.as_mut_slice());
    assert_eq!(test_program, vec![2, 3, 0, 6, 99]);

    test_program = vec![2, 4, 4, 5, 99, 0];
    process_intcode_prog(test_program.as_mut_slice());
    assert_eq!(test_program, vec![2, 4, 4, 5, 99, 9801]);

    test_program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    process_intcode_prog(test_program.as_mut_slice());
    assert_eq!(test_program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}