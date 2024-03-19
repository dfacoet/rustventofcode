use regex::Regex;
use std::{collections::VecDeque, fs};

const YEAR: u16 = 2018;
const DAY: u8 = 9;

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

fn parse_input(input: String) -> (usize, usize) {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    if let Some(caps) = re.captures(input.lines().next().unwrap()) {
        let [n1, n2] = [&caps[1], &caps[2]].map(|s| s.parse().unwrap());
        return (n1, n2);
    }
    panic!("")
}

fn part1((n_players, n_marbles): &(usize, usize)) -> usize {
    let mut marbles = VecDeque::from([0, 1]);
    let mut scores = vec![0; *n_players];

    for i in 2..=*n_marbles {
        if i % 23 == 0 {
            marbles.rotate_right(7);
            scores[i % n_players] += i + marbles.remove(0).unwrap();
        } else {
            marbles.rotate_left(2);
            marbles.insert(0, i);
        }
    }
    *scores.iter().max().unwrap()
}

fn part2((n_players, n_marbles): &(usize, usize)) -> usize {
    let n_marbles = 100 * n_marbles;
    part1(&(*n_players, n_marbles))
}
