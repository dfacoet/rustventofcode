use std::{cmp::min, collections::HashMap, fs};

const YEAR: u16 = 2018;
const DAY: u8 = 18;

const SIZE: usize = 50;
type Grid = [[char; SIZE]; SIZE];

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

fn parse_input(input: String) -> Grid {
    let mut grid = [['x'; SIZE]; SIZE];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
        }
    }
    grid
}

fn part1(grid: &Grid) -> usize {
    let mut grid = *grid;
    for _ in 0..10 {
        grid = step(&grid);
    }

    estimate_value(&grid)
}

fn part2(grid: &Grid) -> usize {
    let mut grid = *grid;
    let mut seen = HashMap::new();

    let n_iter = 1_000_000_000;

    for i in 0..n_iter {
        if let Some(j) = seen.insert(grid, i) {
            if (n_iter - i) % (i - j) == 0 {
                // Integer number of periods away from target
                break;
            }
        }
        grid = step(&grid);
    }

    estimate_value(&grid)
}

// fn print_grid(grid: &Grid) {
//     let grid_str = grid
//         .iter()
//         .map(|row| row.iter().collect::<String>())
//         .collect::<Vec<String>>()
//         .join("\n");
//     println!("{grid_str}")
// }

fn estimate_value(grid: &Grid) -> usize {
    let mut n_wood = 0;
    let mut n_yard = 0;
    let mut counts = HashMap::new();
    ".|#".chars().for_each(|c| {
        counts.insert(c, 0);
    });
    for row in grid.iter() {
        for c in row.iter() {
            match c {
                '|' => n_wood += 1,
                '#' => n_yard += 1,
                _ => {}
            }
        }
    }
    n_wood * n_yard
}

fn neighbours(i: usize, j: usize, grid: &Grid) -> HashMap<char, u8> {
    let mut counts = HashMap::new();
    ".|#".chars().for_each(|c| {
        counts.insert(c, 0);
    });

    let xmin = if i == 0 { 0 } else { i - 1 };
    let xmax = min(i + 1, SIZE - 1);
    let ymin = if j == 0 { 0 } else { j - 1 };
    let ymax = min(j + 1, SIZE - 1);
    for ni in xmin..=xmax {
        for nj in ymin..=ymax {
            if (ni, nj) != (i, j) {
                *counts.get_mut(&grid[nj][ni]).unwrap() += 1;
            }
        }
    }
    counts
}

fn step(grid: &Grid) -> Grid {
    let mut new_grid = [['x'; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            new_grid[j][i] = match grid[j][i] {
                '.' => {
                    if *neighbours(i, j, grid).get(&'|').unwrap() >= 3 {
                        '|'
                    } else {
                        '.'
                    }
                }
                '|' => {
                    if *neighbours(i, j, grid).get(&'#').unwrap() >= 3 {
                        '#'
                    } else {
                        '|'
                    }
                }
                '#' => {
                    if *neighbours(i, j, grid).get(&'#').unwrap() >= 1
                        && *neighbours(i, j, grid).get(&'|').unwrap() >= 1
                    {
                        '#'
                    } else {
                        '.'
                    }
                }
                _ => panic!("Not a valid map"),
            }
        }
    }
    new_grid
}
