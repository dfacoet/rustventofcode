use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fs, vec,
};

use itertools::Itertools;

const YEAR: u16 = 2018;
const DAY: u8 = 15;

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

pub fn parse_input(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect()
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    // (turns, coordinates) -> (HP, race)
    let mut creatures = BTreeMap::<(usize, (usize, usize)), (u8, Race)>::new();
    let mut counter = HashMap::<Race, usize>::new();
    counter.insert(Race::Goblin, 0);
    counter.insert(Race::Elf, 0);

    let mut map = map.clone();
    for (i, line) in map.iter_mut().enumerate() {
        for (j, c) in line.iter_mut().enumerate() {
            match c {
                'G' => {
                    creatures.insert((0, (i, j)), (200, Race::Goblin));
                    *counter.get_mut(&Race::Goblin).unwrap() += 1;
                    // *c = '.';
                }
                'E' => {
                    creatures.insert((0, (i, j)), (200, Race::Elf));
                    *counter.get_mut(&Race::Elf).unwrap() += 1;
                }
                _ => {}
            }
        }
    }

    while let Some(((t, p), (hp, race))) = creatures.pop_first() {
        // println!("{} {:?}", creatures.len(), creatures);
        if creatures.len() < 4 {
            println!("{} {:?}", creatures.len(), creatures);
            break;
        }
        let new_p = match find_move(&map, &t, &p, &race, &creatures) {
            Move::Attack(target_p, target_turn) => {
                let (target_hp, target_race) = creatures.get_mut(&(target_turn, target_p)).unwrap();
                println!(
                    "{:?} is attacking {:?} with {target_hp}HP",
                    race, target_race
                );
                if *target_hp > 3 {
                    *target_hp -= 3;
                    println!("New HP: {target_hp}");
                } else {
                    println!("Dead");
                    *counter.get_mut(target_race).unwrap() -= 1;
                    creatures.remove(&(target_turn, target_p));
                    map[target_p.0][target_p.1] = '.';
                    println!("{:?}", counter);
                    if counter.values().any(|c| *c == 0) {
                        break;
                    }
                }
                // creatures.insert((t + 1, p), (hp, race));
                p
            }
            Move::Walk(new_p) => {
                println!("{:?} is moving to {}, {}", race, new_p.0, new_p.1);
                // creatures.insert((t + 1, new_p), (hp, race));
                map[new_p.0][new_p.1] = map[p.0][p.1];
                map[p.0][p.1] = '.';
                new_p
            }
            Move::Pass => p,
        };
        creatures.insert((t + 1, new_p), (hp, race));
    }
    let t = creatures.first_key_value().unwrap().0 .0;
    let total_score: usize = creatures.iter().map(|(_, (hp, _))| *hp as usize).sum();
    t + total_score
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Race {
    Goblin,
    Elf,
}

enum Move {
    Attack((usize, usize), usize), // (target_position, target_turn)
    Walk((usize, usize)),
    Pass,
}

fn neighbors(p: &(usize, usize)) -> [(usize, usize); 4] {
    let (i, j) = *p;
    // No need to check boundary (safety wall)
    // Order implements "reading order"
    [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)]
}

fn find_move(
    map: &[Vec<char>],
    t: &usize,
    p: &(usize, usize),
    race: &Race,
    creatures: &BTreeMap<(usize, (usize, usize)), (u8, Race)>,
) -> Move {
    for n in neighbors(p) {
        // if let Some(x) = ... && condition(x) is unstable.
        // Would like to match for _any_ turn, meaning this is not
        // the best structure. TODO: think about it
        if let Some((_, neighbor_race)) = creatures.get(&(*t, n)) {
            if neighbor_race != race {
                return Move::Attack(n, *t);
            }
        } else if let Some((_, neighbor_race)) = creatures.get(&(*t + 1, n)) {
            if neighbor_race != race {
                return Move::Attack(n, t + 1);
            }
        }
    }

    let mut paths = VecDeque::from(vec![vec![*p]]);
    let mut reached = HashSet::new();
    reached.insert(*p);

    // print!("Thinking about move for {:?} at {},{}", race, p.0, p.1);
    while let Some(path) = paths.pop_front() {
        for n in neighbors(path.last().unwrap())
            .iter()
            .filter(|n| !path.contains(n))
        {
            if map[n.0][n.1] == '.' {
                for nn in neighbors(n) {
                    match map[nn.0][nn.1] {
                        'G' if *race == Race::Elf => {
                            // Found a path leading to attack.
                            // Paths are generated in the right (lenght, reading) order
                            // so return the first step
                            let cell = if path.len() == 1 { nn } else { path[1] };
                            return Move::Walk(cell);
                        }
                        'E' if *race == Race::Goblin => {
                            let cell = if path.len() == 1 { nn } else { path[1] };
                            return Move::Walk(cell);
                        }
                        _ => {}
                    }
                }
                // There's probably a better way to represent branching paths
                if reached.insert(*n) {
                    let new_path = path
                        .iter()
                        .cloned()
                        .chain(std::iter::once(*n))
                        .collect_vec();
                    paths.push_back(new_path);
                }
            }
        }
    }

    Move::Pass
}

fn part2(_map: &Vec<Vec<char>>) -> usize {
    0
}
