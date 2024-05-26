use std::{collections::HashSet, fs};

use itertools::Itertools;

const YEAR: u16 = 2018;
const DAY: u8 = 25;

fn main() {
    let input_file = format!("inputs/{YEAR}_{DAY:02}.txt");
    let input = fs::read_to_string(input_file).expect("Input file not found");
    let parsed_input = parse_input(input);

    let sol1 = part1(&parsed_input);
    // let sol2 = part2(&parsed_input);

    println!("{YEAR} day {DAY}");
    println!("================");
    println!("Part 1: {sol1}");
    println!("Part 2: ******");
}

fn parse_input(input: String) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let values: Vec<i32> = line.split(',').map(|num| num.parse().unwrap()).collect();
            [values[0], values[1], values[2], values[3]]
        })
        .collect()
}

fn part1(points: &[Coord]) -> usize {
    let mut constellations = Vec::<Constellation>::new();
    for p in points.iter().sorted() {
        let mut new_constellation = constellations
            .iter()
            .filter(|c| is_in_constellation(p, c))
            .fold(HashSet::new(), |acc, c| acc.union(c).cloned().collect());

        if !new_constellation.is_empty() {
            constellations.retain(|c| !is_in_constellation(p, c));
        }
        new_constellation.insert(*p);
        constellations.push(new_constellation);
    }
    constellations.len()
}

type Coord = [i32; 4];

type Constellation = HashSet<Coord>;

fn is_in_constellation(p1: &Coord, c: &Constellation) -> bool {
    c.iter().any(|p2| manhattan_distance(p1, p2) <= 3)
}

fn manhattan_distance(p1: &Coord, p2: &Coord) -> i32 {
    p1.iter()
        .zip(p2.iter())
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum()
}
