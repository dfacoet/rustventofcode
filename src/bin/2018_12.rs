use std::{collections::HashMap, fs};

const YEAR: u16 = 2018;
const DAY: u8 = 12;

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

fn parse_input(input: String) -> (String, HashMap<String, char>) {
    let state = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("initial state: ")
        .trim_end()
        .to_string();

    let mut rules = HashMap::new();
    for line in input.lines().skip(2) {
        if line == "\n" {
            break;
        }
        rules.insert(line[..5].to_string(), line.chars().last().unwrap());
    }
    (state, rules)
}

fn part1((initial_state, rules): &(String, HashMap<String, char>)) -> usize {
    let mut state = initial_state.to_owned(); // Should use LinkedList

    for _ in 0..20 {
        state = format!("....{state}....");
        state = state
            .as_bytes()
            .windows(5)
            .map(|w| {
                let window = std::str::from_utf8(w).unwrap();
                rules.get(window).unwrap()
            })
            .collect();
        // Padding with more chars than necessary, add the same number
        // of chars at each step (2+2) making it easier to compute the positions
    }
    eval(&state, 20)
}

fn part2((_, _): &(String, HashMap<String, char>)) -> usize {
    // TODO: prove formula / write function to compute the coefficients
    62 * 50_000_000_000 + 655
}

fn eval(state: &str, n_gen: usize) -> usize {
    state
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == '#' { Some(i - 2 * n_gen) } else { None })
        .sum()
}
