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

type Coord = (i32, i32);

fn part1(input: &str) -> usize {
    let graph = build_graph(input);

    let mut d = 0;
    let mut reached = HashSet::new();
    reached.insert((0, 0));
    let mut at_d = HashSet::new();
    at_d.insert((0, 0));
    while reached.len() < graph.len() {
        let mut at_d_plus_one = HashSet::new();
        for coord in at_d.iter() {
            for neighbour in graph.get(coord).unwrap() {
                if reached.insert(*neighbour) {
                    at_d_plus_one.insert(*neighbour);
                }
            }
        }
        d += 1;
        at_d = at_d_plus_one;
    }
    d
}

fn part2(input: &str) -> usize {
    let graph = build_graph(input);

    let mut d = 0;
    let mut count = 0;
    let mut reached = HashSet::new();
    reached.insert((0, 0));
    let mut at_d = HashSet::new();
    at_d.insert((0, 0));
    while reached.len() < graph.len() {
        let mut at_d_plus_one = HashSet::new();
        for coord in at_d.iter() {
            for neighbour in graph.get(coord).unwrap() {
                if reached.insert(*neighbour) {
                    at_d_plus_one.insert(*neighbour);
                }
            }
        }
        d += 1;
        at_d = at_d_plus_one;
        if d >= 1000 {
            count += at_d.len();
        }
    }
    count
}

fn split_directions(s: &str) -> Vec<String> {
    let mut depth = 0;
    let mut options = Vec::new();
    let mut start = 1;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => depth += 1,
            '|' if depth == 1 => {
                options.push(&s[start..i]);
                start = i + 1;
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    options.push(&s[start..i]);
                    return options
                        .iter()
                        .map(|o| format!("{}{}", o, &s[i + 1..]))
                        .collect::<Vec<_>>();
                }
            }
            _ => {}
        }
    }
    panic!("Unmatched parenthesis");
}

fn build_graph(input: &str) -> HashMap<Coord, HashSet<Coord>> {
    let mut graph: HashMap<Coord, HashSet<Coord>> = HashMap::new(); // {(x, y): [(neighbour_x, neighbour_x)]}

    let mut queue = Vec::new(); // [(coord, remaining string)]
    queue.push(((0, 0), input.to_string()));
    let mut seen = HashSet::new();
    while let Some((mut coord, directions)) = queue.pop() {
        seen.insert((coord, directions.clone()));
        for (i, c) in directions.chars().enumerate() {
            let new_coord = match c {
                'N' => (coord.0, coord.1 + 1),
                'S' => (coord.0, coord.1 - 1),
                'E' => (coord.0 + 1, coord.1),
                'W' => (coord.0 - 1, coord.1),
                '(' => {
                    for new_directions in split_directions(&directions[i..]) {
                        if !seen.contains(&(coord, new_directions.clone())) {
                            queue.push((coord, new_directions));
                        }
                    }
                    break;
                }
                _ => panic!(),
            };
            graph.entry(coord).or_default().insert(new_coord);
            graph.entry(new_coord).or_default().insert(coord);
            coord = new_coord;
        }
    }
    graph
}
