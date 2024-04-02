use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
    str::FromStr,
};

use itertools::Itertools;

const YEAR: u16 = 2018;
const DAY: u8 = 17;

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

fn parse_input(input: String) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect()
}

#[derive(Debug)]
enum Line {
    Horizontal((usize, usize), usize),
    Vertical(usize, (usize, usize)),
}

impl Line {
    fn points(&self) -> HashSet<(usize, usize)> {
        match *self {
            Line::Horizontal((left, right), y) => (left..=right).map(|x| (x, y)).collect(),
            Line::Vertical(x, (bottom, top)) => (bottom..=top).map(|y| (x, y)).collect(),
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p, range)) = s
            .split(", ")
            .map(|s| s.split('=').last().unwrap())
            .collect_tuple()
        {
            let p = p.parse().unwrap();
            if let Some((rmin, rmax)) = range
                .split("..")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
            {
                return match s.chars().next() {
                    Some('x') => Ok(Line::Vertical(p, (rmin, rmax))),
                    Some('y') => Ok(Line::Horizontal((rmin, rmax), p)),
                    _ => Err(()),
                };
            }
        }
        Err(())
    }
}

fn part1(clay_lines: &[Line]) -> usize {
    let (mut xmin, mut xmax) = (usize::MAX, usize::MIN);
    let (mut ymin, mut ymax) = (usize::MAX, usize::MIN);
    for line in clay_lines {
        let (&left, &right, &bottom, &top) = match line {
            Line::Horizontal((left, right), y) => (left, right, y, y),
            Line::Vertical(x, (bottom, top)) => (x, x, bottom, top),
        };
        xmin = min(xmin, left);
        xmax = max(xmax, right);
        ymin = min(ymin, bottom);
        ymax = max(ymax, top);
    }

    let mut grid = vec![vec!['.'; xmax + 2]; ymax + 2]; // wasting some space to avoid dealing with offsets
    for line in clay_lines {
        for point in line.points() {
            grid[point.1][point.0] = '#';
        }
    }
    flow(500, 0, &mut grid);
    grid.iter()
        .skip(ymin)
        .take(ymax - ymin + 1)
        .map(|row| {
            row.iter()
                .skip(xmin - 1) // TODO: Fix filling algorithm that creates a spill at xmin-1 instead of skipping
                .filter(|&&c| c == '|' || c == '~')
                .count()
        })
        .sum()
}

// fn print_grid(grid: &[Vec<char>], name: &str, xmin: &usize) {
//     let grid_str = grid
//         .iter()
//         .map(|row| row.iter().skip(*xmin - 1).collect::<String>())
//         .collect::<Vec<String>>()
//         .join("\n");
//     let fname = format!("1817_grid_{name}.txt");
//     fs::write(fname, grid_str).unwrap();
// }

fn flow(x0: usize, y0: usize, grid: &mut Vec<Vec<char>>) {
    if y0 == grid.len() - 1 || grid[y0 + 1][x0] == '|' {
        return;
    }
    if grid[y0 + 1][x0] == '.' {
        grid[y0 + 1][x0] = '|';
        return flow(x0, y0 + 1, grid);
    }
    // hit a shelf
    let mut left_wall = None;
    let mut right_wall = None;
    for x in (0..x0).rev() {
        if grid[y0][x] == '#' {
            left_wall = Some(x);
            break;
        }
        grid[y0][x] = '|';
        if grid[y0 + 1][x] == '.' {
            flow(x, y0, grid);
            break;
        }
    }
    for x in x0 + 1..grid[y0].len() {
        if grid[y0][x] == '#' {
            right_wall = Some(x);
            break;
        }
        grid[y0][x] = '|';
        if grid[y0 + 1][x] == '.' {
            flow(x, y0, grid);
            break;
        }
    }

    if let (Some(xl), Some(xr)) = (left_wall, right_wall) {
        // two walls, fill with ~
        for x in xl + 1..xr {
            grid[y0][x] = '~';
        }
        grid[y0 - 1][x0] = '|';
        flow(x0, y0 - 1, grid);
    }
}

fn part2(clay_lines: &[Line]) -> usize {
    let (mut xmin, mut xmax) = (usize::MAX, usize::MIN);
    let (mut ymin, mut ymax) = (usize::MAX, usize::MIN);
    for line in clay_lines {
        let (&left, &right, &bottom, &top) = match line {
            Line::Horizontal((left, right), y) => (left, right, y, y),
            Line::Vertical(x, (bottom, top)) => (x, x, bottom, top),
        };
        xmin = min(xmin, left);
        xmax = max(xmax, right);
        ymin = min(ymin, bottom);
        ymax = max(ymax, top);
    }
    let mut grid = vec![vec!['.'; xmax + 2]; ymax + 2]; // wasting some space to avoid dealing with offsets
    for line in clay_lines {
        for point in line.points() {
            grid[point.1][point.0] = '#';
        }
    }
    flow(500, 0, &mut grid);
    // fix incomplete filling of rows
    // TODO: fix the filling algorithm instead
    for y in 1..ymax {
        for x in xmin..xmax {
            if grid[y][x] == '|' && grid[y - 1][x] == '~' {
                grid[y][x] = '~';
            }
        }
    }
    grid.iter()
        .skip(ymin)
        .take(ymax - ymin + 1)
        .map(|row| row.iter().filter(|&&c| c == '~').count())
        .sum()
}
