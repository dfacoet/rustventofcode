use regex::Regex;
use std::cmp;
use std::{fs, str::FromStr};

const YEAR: u16 = 2018;
const DAY: u8 = 10;

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

fn parse_input(input: String) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| Point::from_str(line).ok())
        .collect()
}

#[derive(Clone, Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rs = r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)";
        let re = Regex::new(rs).unwrap();
        if let Some(caps) = re.captures(s) {
            let [x, y, vx, vy] =
                [&caps[1], &caps[2], &caps[3], &caps[4]].map(|s| s.parse().unwrap());
            return Ok(Point {
                position: (x, y),
                velocity: (vx, vy),
            });
        }
        Err(())
    }
}

impl Point {
    fn evolve(&mut self, n: i32) {
        self.position.0 += n * self.velocity.0;
        self.position.1 += n * self.velocity.1;
    }
}

fn part1(points: &[Point]) -> usize {
    let mut points = points.to_owned();
    let mut height = get_height(&points);

    loop {
        for p in &mut points {
            p.evolve(1);
        }
        let new_heigth = get_height(&points);
        if new_heigth > height {
            break;
        }
        height = new_heigth
    }
    for p in &mut points {
        p.evolve(-1);
    }
    draw(&points);
    0
}

fn range(points: &[Point]) -> (i32, i32, i32, i32) {
    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    for p in points {
        xmin = cmp::min(xmin, p.position.0);
        xmax = cmp::max(xmax, p.position.0);
        ymin = cmp::min(ymin, p.position.1);
        ymax = cmp::max(ymax, p.position.1);
    }
    (xmin, xmax, ymin, ymax)
}

fn get_height(points: &[Point]) -> usize {
    let (_, _, ymin, ymax) = range(points);
    (ymax - ymin) as usize
}

fn draw(points: &[Point]) {
    let (xmin, xmax, ymin, ymax) = range(points);

    let h = (ymax - ymin + 1) as usize;
    let w = (xmax - xmin + 1) as usize;
    let mut matrix = vec![vec![' '; w]; h];
    for p in points {
        let x = (p.position.0 - xmin) as usize;
        let y = (p.position.1 - ymin) as usize;
        matrix[y][x] = '#';
    }
    for line in matrix {
        println!("{}", line.iter().collect::<String>())
    }
}

fn part2(points: &[Point]) -> usize {
    let mut points = points.to_owned();
    let mut height = get_height(&points);
    let mut s = 0;

    loop {
        for p in &mut points {
            p.evolve(1);
        }
        s += 1;
        let new_heigth = get_height(&points);
        if new_heigth > height {
            break;
        }
        height = new_heigth;
    }
    s - 1
}
