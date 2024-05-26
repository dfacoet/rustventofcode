use std::fs;

const YEAR: u16 = 2018;
const DAY: u8 = 1;

fn main() {
    let input_file = format!("inputs/{YEAR}_{DAY:02}.txt");
    let input = fs::read_to_string(input_file).expect("Input file not found");
    let parsed_input = parse_input(input);

    let sol1 = part1(&parsed_input);
    let sol2 = part2(&parsed_input);

    println!("{YEAR} day {DAY}");
    println!("================");
    println!("Part 1: {sol1}");
    println!("Part 2: {sol2}");
}

fn parse_input(input: String) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn part1(changes: &[i32]) -> i32 {
    changes.iter().sum()
}

fn part2(changes: &[i32]) -> i32 {
    let mut past_values = std::collections::HashSet::new();
    let mut current_value = 0;

    // for c in changes.iter().cycle() {}
    let mut i = 0;
    while past_values.insert(current_value) {
        current_value += changes[i];
        i = (i + 1) % changes.len();
    }
    current_value
}
