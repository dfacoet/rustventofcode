use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

use std::sync::atomic::{AtomicUsize, Ordering};

use itertools::Itertools;

const YEAR: u16 = 2018;
const DAY: u8 = 15;

const SIZE: usize = 32;

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

type Grid = [[CellState; SIZE]; SIZE];
type Position = (usize, usize);

fn parse_input(input: String) -> Grid {
    let mut grid = [[CellState::Wall; SIZE]; SIZE];
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid[i][j] = match ch {
                '.' => CellState::Empty,
                '#' => CellState::Wall,
                'G' => CellState::Unit(Unit::new('G', (i, j))),
                'E' => CellState::Unit(Unit::new('E', (i, j))),
                _ => panic!("Invalid character in input: {} at ({},{})", ch, i, j),
            };
        }
    }
    grid
}

fn part1(input: &Grid) -> String {
    let (grid, n_rounds, _) = run_battle(input, 3);

    let total_hp: u64 = get_units(&grid).iter().map(|unit| unit.hp).sum();
    let outcome = n_rounds * total_hp;
    outcome.to_string()
}

fn part2(input: &Grid) -> String {
    for elf_atk in 4..200 {
        let (_, _, n_deads) = run_battle(input, elf_atk);
        // print_grid(&grid);
        if n_deads == 0 {
            return elf_atk.to_string();
        }
    }
    panic!("The elves cannot win without deaths");
}

// 20 is not the right answer

fn run_battle(grid: &Grid, elf_atk: u64) -> (Grid, u64, u64) {
    let mut grid = *grid;

    let mut n_rounds = 0;
    let mut n_dead_elves = 0;
    loop {
        let mut units = get_units(&grid);
        for unit in &mut units {
            // Check if the unit is still there
            match grid[unit.position.0][unit.position.1] {
                CellState::Unit(grid_unit) if (grid_unit.id == unit.id) => *unit = grid_unit,
                _ => continue,
            };

            // TODO: refactor. Remove get_targets_position and don't look at targets here
            // - after moving, look at neighbors and attack the one with the least HP (if any)
            // - after checking to attack, check if any target is left (no need to get their position)
            //   if not, break.

            if let Some((i, j)) = find_move(unit, &grid) {
                grid[unit.position.0][unit.position.1] = CellState::Empty;
                unit.position = (i, j);
                grid[i][j] = CellState::Unit(*unit);
            }

            if let Some((_, ti, tj)) = neighbor_indices(&unit.position)
                .iter()
                .filter_map(|(ni, nj)| match grid[*ni][*nj] {
                    CellState::Unit(target) if target.unit_type != unit.unit_type => {
                        Some((target.hp, ni, nj))
                    }
                    _ => None,
                })
                .min()
            {
                match &mut grid[*ti][*tj] {
                    CellState::Unit(ref mut target) if target.unit_type != unit.unit_type => {
                        let atk = match unit.unit_type {
                            UnitType::Elf => elf_atk,
                            UnitType::Goblin => 3,
                        };
                        if target.hp > atk {
                            target.hp -= atk;
                        } else {
                            grid[*ti][*tj] = CellState::Empty;
                            if unit.unit_type == UnitType::Goblin {
                                n_dead_elves += 1
                            };
                        }
                    }
                    _ => panic!("No target found at {:},{:}", ti, tj),
                };
            }

            if !grid.iter().any(|row| {
                row.iter().any(|cell| match cell {
                    CellState::Unit(u) => u.unit_type != unit.unit_type,
                    _ => false,
                })
            }) {
                // no enemies left, battle is over
                return (grid, n_rounds, n_dead_elves);
            }
        }
        n_rounds += 1;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum UnitType {
    Goblin,
    Elf,
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
const MAX_HP: u64 = 200;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Unit {
    id: usize,
    unit_type: UnitType,
    position: Position,
    hp: u64,
}

impl Unit {
    fn new(unit_type: char, position: Position) -> Self {
        let unit_type = match unit_type {
            'G' => UnitType::Goblin,
            'E' => UnitType::Elf,
            _ => panic!("Invalid unit type"),
        };
        Unit {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            unit_type,
            position,
            hp: MAX_HP,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellState {
    Empty,
    Wall,
    Unit(Unit),
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Empty => write!(f, "."),
            CellState::Wall => write!(f, "#"),
            CellState::Unit(unit) => match unit.unit_type {
                UnitType::Goblin => write!(f, "G"),
                UnitType::Elf => write!(f, "E"),
            },
        }
    }
}

// fn print_grid(grid: &Grid) {
//     for row in grid.iter() {
//         let mut hps = Vec::new();
//         for &cell in row.iter() {
//             if let CellState::Unit(unit) = cell {
//                 hps.push(unit.hp);
//             }
//             print!("{:}", cell);
//         }
//         if !hps.is_empty() {
//             print!("    {:?}", hps)
//         };
//         println!();
//     }
// }

fn get_units(grid: &Grid) -> Vec<Unit> {
    let mut units = Vec::new();
    for row in grid.iter() {
        for cell in row.iter() {
            if let CellState::Unit(ref unit) = cell {
                units.push(*unit);
            }
        }
    }
    units
}

fn find_move(unit: &Unit, grid: &Grid) -> Option<Position> {
    let mut paths = VecDeque::new();
    let mut reached = HashSet::new();
    paths.push_front(vec![unit.position]);
    reached.insert(unit.position);

    while !paths.is_empty() {
        let mut new_paths = VecDeque::<Vec<Position>>::new();
        let mut target_paths = Vec::<Vec<Position>>::new();

        for path in paths {
            for (ni, nj) in neighbor_indices(path.last().unwrap())
                .into_iter()
                .filter(|n| reached.insert(*n))
            {
                let new_path = path
                    .clone()
                    .into_iter()
                    .chain(std::iter::once((ni, nj)))
                    .collect_vec();
                match grid[ni][nj] {
                    CellState::Unit(target) if target.unit_type != unit.unit_type => {
                        // Found a path leading to attack
                        if new_path.len() == 2 {
                            // path is [current, target]. Do not move.
                            return None;
                        }
                        target_paths.push(new_path);
                    }
                    CellState::Empty => {
                        new_paths.push_back(new_path);
                    }
                    _ => (),
                };
            }
        }
        // At each step of the outer loop, paths have fixed length L.
        // If there are paths leading to a target at distance L,
        // - pick the best one (sorting by reading order of the attack tile, path[L-2])
        // - return the first step
        if let Some(best_path) = target_paths.iter().min_by_key(|path| path[path.len() - 2]) {
            return Some(best_path[1]);
        }
        // otherwise, look for paths of length L+1
        paths = new_paths;
    }

    None
}

fn neighbor_indices(pos: &Position) -> [Position; 4] {
    let (i, j) = *pos;
    // no need to check boundaries - due to walls
    // this will only be called with 0 < i, j < SIZE - 1
    [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
}
