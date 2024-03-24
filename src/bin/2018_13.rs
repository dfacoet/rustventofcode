use std::{
    collections::{BTreeMap, HashSet},
    fs,
};

use itertools::Itertools;

const YEAR: u16 = 2018;
const DAY: u8 = 13;

const SIZE: usize = 150;

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

fn parse_input(input: String) -> ([[char; SIZE]; SIZE], Vec<Cart>) {
    let mut map = [[' '; SIZE]; SIZE];
    let mut carts = Vec::new();
    let mut n_carts = 0;
    for (i, line) in input.lines().enumerate() {
        if line == "\n" {
            break;
        }
        for (j, c) in line.chars().enumerate() {
            match c {
                '-' | '/' | '|' | '\\' | '+' | ' ' => map[i][j] = c,
                '<' | '>' => {
                    map[i][j] = '-';
                    carts.push(Cart::new(n_carts, (i, j), c));
                    n_carts += 1;
                }
                '^' | 'v' => {
                    map[i][j] = '|';
                    carts.push(Cart::new(n_carts, (i, j), c));
                    n_carts += 1;
                }
                _ => panic!(),
            }
        }
    }
    (map, carts)
}

#[derive(Clone, Debug)]
struct Cart {
    id: usize,
    position: (usize, usize),
    direction: Direction,
    last_turn: Turn,
}

impl Cart {
    fn new(id: usize, position: (usize, usize), c: char) -> Cart {
        let direction = match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!(),
        };
        Cart {
            id,
            position,
            direction,
            last_turn: Turn::Right,
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.position.0 -= 1,
            Direction::Down => self.position.0 += 1,
            Direction::Left => self.position.1 -= 1,
            Direction::Right => self.position.1 += 1,
        }
    }

    fn turn(&mut self, cell: char) {
        match cell {
            '-' | '|' => {}
            '/' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                }
            }
            '\\' => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Down,
                }
            }
            '+' => {
                match self.last_turn {
                    Turn::Left => {
                        // Go straight
                        self.last_turn = Turn::Straight
                    }
                    Turn::Straight => {
                        // Turn right
                        self.direction = match self.direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        };
                        self.last_turn = Turn::Right
                    }
                    Turn::Right => {
                        // Turn left
                        self.direction = match self.direction {
                            Direction::Up => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Down => Direction::Right,
                            Direction::Right => Direction::Up,
                        };
                        self.last_turn = Turn::Left
                    }
                }
            }
            _ => panic!("Not a railway cell: {cell}"),
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
enum Turn {
    Right,
    Left,
    Straight,
}

fn part1((map, carts): &([[char; SIZE]; SIZE], Vec<Cart>)) -> String {
    let mut carts = carts.clone();
    let mut positions: HashSet<(usize, usize)> = carts.iter().map(|c| c.position).collect();

    loop {
        for cart in carts.iter_mut().sorted_by_key(|c| c.position) {
            if !positions.remove(&cart.position) {
                panic!("");
            }
            cart.step();
            let new_cell = map[cart.position.0][cart.position.1];
            cart.turn(new_cell);
            if !positions.insert(cart.position) {
                // Crash! Note x-y are swapped
                return format!("{},{}", cart.position.1, cart.position.0);
            };
        }
    }
}

fn part2((map, carts): &([[char; SIZE]; SIZE], Vec<Cart>)) -> String {
    let mut carts: BTreeMap<(usize, usize), Cart> =
        carts.iter().map(|c| (c.position, c.clone())).collect();

    let mut tick = 0;
    while carts.len() > 1 {
        let mut crashed_ids = HashSet::new();
        let mut new_carts: BTreeMap<(usize, usize), Cart> = BTreeMap::new();
        while let Some((_, mut cart)) = carts.pop_first() {
            if crashed_ids.contains(&cart.id) {
                continue;
            }
            cart.step();
            let new_cell = map[cart.position.0][cart.position.1];
            cart.turn(new_cell);

            // Ugly but it works
            if let Some(other_crashed) = carts.get(&cart.position) {
                crashed_ids.insert(cart.id);
                crashed_ids.insert(other_crashed.id);
            } else if let Some(other_crashed) = new_carts.get(&cart.position) {
                crashed_ids.insert(cart.id);
                crashed_ids.insert(other_crashed.id);
            } else {
                new_carts.insert(cart.position, cart);
            }
        }
        carts = new_carts
            .into_iter()
            .filter(|(_, c)| !crashed_ids.contains(&c.id))
            .collect();
        tick += 1;
    }

    println!("{tick}");
    let (p, _) = carts.pop_first().unwrap();
    format!("{},{}", p.1, p.0)
}
