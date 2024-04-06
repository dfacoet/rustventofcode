use rustventofcode::device18::{parse_input, Instr};
use std::fs;

const YEAR: u16 = 2018;
const DAY: u8 = 19;

fn main() {
    let input_file = format!("inputs/{YEAR}_{DAY:02}.txt");
    let input = fs::read_to_string(input_file).expect("Input file not found");
    let parsed_input = parse_input(input);

    println!("{YEAR} day {DAY}");
    println!("================");

    let sol1 = part1(&parsed_input);
    println!("Part 1: {sol1}");

    let sol2 = part2(&parsed_input);
    println!("Part 2: {sol2}");
}

fn part1((ip, instructions): &(usize, Vec<Instr>)) -> usize {
    let mut registers = [0; 6];

    while registers[*ip] < instructions.len() {
        instructions[registers[*ip]].apply(&mut registers);
        registers[*ip] += 1;
    }
    registers[0]
}

fn part2((ip, instructions): &(usize, Vec<Instr>)) -> usize {
    let mut registers = [0; 6];
    registers[0] = 1;

    // Solved by inspecting the instructions:
    // lines 17-end only run once, to initialise register[5]. Lines 27-end only run
    // if registers[0] = 1, making the value bigger for part2.
    // Initialisation finishes when instruction 1 is executed.
    while registers[*ip] != 1 {
        instructions[registers[*ip]].apply(&mut registers);
        registers[*ip] += 1;
    }
    // lines 1-16 are equivalent to
    // for x in 1..=registers[5] {
    //     for y in 1..=registers[5] {
    //         if x * y == registers[5] {
    //             registers[0] += x;
    //         }
    //     }
    // }
    // Which just computes the sum of divisors of register[5]
    for x in 1..=registers[5] {
        if registers[5] % x == 0 {
            registers[0] += x;
        }
    }
    registers[0]
}
