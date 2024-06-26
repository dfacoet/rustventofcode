use rustventofcode::device18::{parse_input, Instr};
use std::{collections::HashMap, fs};

const YEAR: u16 = 2018;
const DAY: u8 = 21;

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

    // r0 doesn't actually enter the computation, but only
    // the exit condition r5 == r0 implemented by lines 28-29.
    // If we get there, r5 is the answer.

    // lines 0-5 are a test and are executed once
    // lines 6-7 initialise the outer loop

    // lines 17-25 are a loop essentially equivalent to r1 = r3 / 256
    // this is enough optimisation for part1

    loop {
        match registers[*ip] {
            28 => return registers[5],
            17 => {
                registers[1] = registers[3] / 256;
                registers[2] = 26; // pointer
            }
            i if i > 30 => panic!("Program terminated unexpectedly"),
            _ => {
                // execute instruction normally
                instructions[registers[*ip]].apply(&mut registers);
                registers[*ip] += 1;
            }
        }
    }
}

fn part2((ip, instructions): &(usize, Vec<Instr>)) -> usize {
    let mut solutions = HashMap::new();
    let mut registers = [0; 6];
    let mut c = 0;
    loop {
        match registers[*ip] {
            28 => {
                // Store the solutions values of r0 for which the program halts
                // as keys; values are the number of instructions executed the first
                // time a solution is encountered.
                // TO THINK: when do we know we found all solutions?
                // empirically the correct solution is found between 10^9 and 10^10 instructions
                let sol = registers[5];
                c += 2;
                solutions.entry(sol).or_insert(c);
                registers[2] = 6;
            }
            17 => {
                registers[1] = registers[3] / 256;
                registers[2] = 26; // pointer
                c += 5 + 7 * registers[1];
            }
            i if i > 30 => panic!("Program terminated unexpectedly"),
            _ => {
                // execute instruction normally
                instructions[registers[*ip]].apply(&mut registers);
                registers[*ip] += 1;
            }
        }
        c += 1;
        if c > 10usize.pow(10) {
            break;
        }
    }
    *solutions.iter().max_by_key(|(_, &v)| v).unwrap().0
}
