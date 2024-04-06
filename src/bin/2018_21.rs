use rustventofcode::device18::{parse_input, Instr};
use std::fs;

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
    0
}

fn part2((ip, instructions): &(usize, Vec<Instr>)) -> usize {
    0
}
