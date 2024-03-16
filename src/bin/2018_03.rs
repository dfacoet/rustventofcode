use core::panic;
use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
    str::FromStr,
};

const YEAR: u16 = 2018;
const DAY: u8 = 3;

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

fn parse_input(input: String) -> Vec<Claim> {
    input
        .lines()
        .map(|line| Claim::from_str(line).unwrap())
        .collect()
}

#[derive(Clone, Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Claim {
    fn xmax(&self) -> usize {
        self.x + self.w - 1
    }
    fn ymax(&self) -> usize {
        self.y + self.h - 1
    }

    fn contains(&self, x: &usize, y: &usize) -> bool {
        self.x <= *x && *x <= self.xmax() && self.y <= *y && *y <= self.ymax()
    }

    fn overlaps(&self, other: &Claim) -> bool {
        let overlaps_x = self.x <= other.xmax() && self.xmax() >= other.x;
        let overlaps_y = self.y <= other.ymax() && self.ymax() >= other.y;
        overlaps_x && overlaps_y
    }
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 4 || parts[1] != "@" {
            return Err(());
        }

        let id = parts[0].trim_start_matches('#').parse().map_err(|_| ())?;
        let x = parts[2]
            .split(',')
            .next()
            .unwrap()
            .parse()
            .map_err(|_| ())?;
        let y = parts[2]
            .trim_end_matches(':')
            .split(',')
            .nth(1)
            .unwrap()
            .parse()
            .map_err(|_| ())?;
        let w = parts[3]
            .split('x')
            .next()
            .unwrap()
            .parse()
            .map_err(|_| ())?;
        let h = parts[3]
            .split('x')
            .nth(1)
            .unwrap()
            .parse()
            .map_err(|_| ())?;

        Ok(Claim { id, x, y, w, h })
    }
}

fn part1(claims: &[Claim]) -> usize {
    let mut count = 0;

    // Find range
    let (mut xmin, mut xmax) = (usize::MAX, usize::MIN);
    let (mut ymin, mut ymax) = (usize::MAX, usize::MIN);
    for c in claims {
        xmin = min(xmin, c.x);
        xmax = max(xmax, c.xmax());
        ymin = min(ymin, c.y);
        ymax = max(ymax, c.ymax());
    }

    for i in xmin..=xmax {
        for j in ymin..=ymax {
            // claims.iter().filter(|&c| *c.contains(&i, &j)).count();\
            let mut overlap = 0;
            for c in claims {
                if c.contains(&i, &j) {
                    overlap += 1;
                    if overlap > 1 {
                        count += 1;
                        break;
                    }
                };
            }
        }
    }

    count
}

fn part2(claims: &[Claim]) -> usize {
    let mut overlapping = HashSet::new();
    for c1 in claims.iter() {
        if overlapping.contains(&c1.id) {
            continue;
        }
        let mut overlap = false;
        for c2 in claims.iter() {
            if c1.id != c2.id && c1.overlaps(c2) {
                overlap = true;
                overlapping.insert(c2.id);
                break;
            }
        }
        if !overlap {
            return c1.id;
        };
    }
    panic!("Non-overlapping claim not found")
}
