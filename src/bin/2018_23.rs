use std::{fs, str::FromStr};

use regex::Regex;

const YEAR: u16 = 2018;
const DAY: u8 = 23;

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

fn parse_input(input: String) -> Vec<Nanobot> {
    input
        .lines()
        .map(|l| Nanobot::from_str(l).unwrap())
        .collect()
}

fn part1(nanobots: &[Nanobot]) -> usize {
    let strongest = nanobots.iter().max_by_key(|n| n.r).unwrap();
    nanobots
        .iter()
        .filter(|n| strongest.distance(n.coord) <= strongest.r)
        .count()
}

fn part2(nanobots: &[Nanobot]) -> usize {
    let mut min_coord = [i32::MAX; 3];
    let mut max_coord = [i32::MIN; 3];
    for n in nanobots {
        for i in 0..3 {
            min_coord[i] = min_coord[i].min(n.coord[i] - n.r);
            max_coord[i] = max_coord[i].max(n.coord[i] + n.r);
        }
    }
    let mut max_size = 1;
    while max_size
        < max_coord
            .iter()
            .zip(min_coord.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    {
        max_size *= 2;
    }

    0
}

struct Nanobot {
    coord: [i32; 3],
    r: i32,
}

impl FromStr for Nanobot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        let cap = re.captures(s).ok_or(())?;
        let coord = [
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ];
        let r = cap[4].parse().unwrap();

        Ok(Nanobot { coord, r })
    }
}

impl Nanobot {
    fn distance(&self, point: [i32; 3]) -> i32 {
        self.coord
            .iter()
            .zip(point.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
}
