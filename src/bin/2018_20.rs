use std::{
    collections::{HashMap, HashSet},
    fs,
};

const YEAR: u16 = 2018;
const DAY: u8 = 20;

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

fn parse_input(input: String) -> String {
    input
        .trim_start_matches('^')
        .trim_end()
        .trim_end_matches('$')
        .to_string()
}

fn part1(input: &str) -> usize {
    println!("{}", input);
    let mut map = HashMap::new();

    let mut coord = (0, 0);
    for c in input.chars().take(6) {
        let new_coord = match c {
            'N' => (coord.0, coord.1 + 1),
            'S' => (coord.0, coord.1 - 1),
            'E' => (coord.0 + 1, coord.1),
            'W' => (coord.0 - 1, coord.1),
            _ => panic!(),
        };
        // add links
        map.entry(coord).or_insert(HashSet::new()).insert(new_coord);
        map.entry(new_coord).or_insert(HashSet::new()).insert(coord);
        coord = new_coord;
    }
    println!("{:?}", map);
    0
}

fn part2(input: &str) -> usize {
    0
}
