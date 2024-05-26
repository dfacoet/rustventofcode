use std::{
    cmp::{min, Reverse},
    collections::HashMap,
    fs,
};

use itertools::Itertools;
use priority_queue::PriorityQueue;

const YEAR: u16 = 2018;
const DAY: u8 = 22;

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

fn parse_input(input: String) -> (usize, (usize, usize)) {
    let mut lines = input.lines();
    let depth = lines
        .next()
        .unwrap()
        .trim_start_matches("depth: ")
        .parse()
        .unwrap();
    let target = lines
        .next()
        .unwrap()
        .trim_start_matches("target: ")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect_tuple()
        .unwrap();
    (depth, target)
}

fn part1((depth, target): &(usize, (usize, usize))) -> usize {
    build_grid(depth, target, &(0, 0)).iter().flatten().sum()
}

fn part2((depth, target): &(usize, (usize, usize))) -> usize {
    // let depth = &7305;
    // let target = &(13, 734);
    // let depth = &510;
    // let target = &(10, 10);
    // For now: make the grid bigger and hope the finite-grid optimal path
    // is also the optimal path on the infinite grid.
    // Idea to improve: infinite search with cutoff at target distance
    let grid = build_grid(depth, target, &(200, 200));

    // Nodes: (x, y, tool) with tool in {0, 1, 2}. grid[y][x] == tool is not allowed.
    // 0: rocky / no equipment
    // 1: wet / torch
    // 2: narrow / climbing gear
    let start = (0, 0, 1);

    let mut dist = HashMap::new();
    dist.insert(start, 0);
    let mut q = PriorityQueue::new();
    q.push_increase(start, Reverse(0usize));

    while let Some((node, rev_d)) = q.pop() {
        assert!(*dist.get(&node).unwrap() == rev_d.0);
        for (neighbor, weight) in get_neighbors(&node, &grid) {
            let new_d = rev_d.0 + weight;
            if new_d < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
                dist.insert(neighbor, new_d);
                q.push_increase(neighbor, Reverse(new_d));
            }
        }
    }
    // *dist.get(&(target.0, target.1, 1)).unwrap()
    min(
        *dist.get(&(target.0, target.1, 1)).unwrap(),
        *dist.get(&(target.0, target.1, 2)).unwrap() + 7,
    )
}

// 1081 is too high
// 1070, 1071, 1073, 1078 are wrong

fn build_grid(depth: &usize, target: &(usize, usize), extra: &(usize, usize)) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let w = target.0 + extra.0 + 1;
    let h = target.1 + extra.1 + 1;
    grid.push((0..w).map(|x| (x * 16807 + *depth) % 20183).collect());
    for y in 1..h {
        let mut row = vec![(y * 48271 + *depth) % 20183];
        for x in 1..w {
            let geologic_index = grid[y - 1][x] * row[x - 1];
            let erosion_level = (geologic_index + *depth) % 20183;
            row.push(erosion_level);
        }
        grid.push(row);
    }
    grid[target.1][target.0] = 0;
    grid.iter_mut().flatten().for_each(|v| *v %= 3);
    grid
}

type Node = (usize, usize, usize);

fn get_neighbors(node: &Node, grid: &Vec<Vec<usize>>) -> HashMap<Node, usize> {
    let (x, y, tool) = *node;
    let mut grid_neighbors = Vec::new();
    if x > 0 {
        grid_neighbors.push((x - 1, y));
    }
    if y > 0 {
        grid_neighbors.push((x, y - 1));
    }
    if x < grid[0].len() - 1 {
        grid_neighbors.push((x + 1, y));
    }
    if y < grid.len() - 1 {
        grid_neighbors.push((x, y + 1));
    }

    // // Move to an allowed neighbouring cell
    // let mut neighbors: HashMap<_, _> = grid_neighbors
    //     .into_iter()
    //     .filter_map(|(nx, ny)| {
    //         if grid[ny][nx] != tool {
    //             Some(((nx, ny, tool), 1))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // // Or change tool
    // let new_tool = 3 - (grid[y][x] + tool);
    // neighbors.insert((x, y, new_tool), 7);
    // neighbors

    grid_neighbors
        .into_iter()
        .map(|(nx, ny)| {
            if grid[ny][nx] == tool {
                // Change tool only if necessary to move
                let new_tool = 3 - (grid[y][x] + tool);
                ((nx, ny, new_tool), 8)
            } else {
                ((nx, ny, tool), 1)
            }
        })
        .collect()
}
