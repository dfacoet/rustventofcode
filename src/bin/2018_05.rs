use std::fs;

const YEAR: u16 = 2018;
const DAY: u8 = 5;

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

fn parse_input(input: String) -> String {
    // Input is a single-line string, no need to parse
    input
}

fn part1(polymer: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();

    for c in polymer.trim_end().chars() {
        let last = *stack.last().unwrap_or(&' ');
        if last.to_ascii_lowercase() == c.to_ascii_lowercase() && last != c {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack.len()
}

fn part2(polymer: &str) -> usize {
    let mut min_len = polymer.len();
    for c in 'a'..='z' {
        let improved_polymer = polymer.replace([c, c.to_ascii_uppercase()], "");
        let len = part1(&improved_polymer);
        if len < min_len {
            min_len = len;
        }
    }

    min_len
}
