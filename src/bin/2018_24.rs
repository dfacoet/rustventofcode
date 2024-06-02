use std::{collections::HashSet, fs, str::FromStr};

use regex::Regex;

const YEAR: u16 = 2018;
const DAY: u8 = 24;

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

fn parse_input(input: String) -> (Army, Army) {
    let mut immune_system_groups = vec![];
    let mut lines = input.lines();
    lines.next();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        immune_system_groups.push(Group::from_str(line).unwrap());
    }
    lines.next();
    let infection_groups = lines.map(|line| Group::from_str(line).unwrap()).collect();

    (
        Army {
            groups: immune_system_groups,
        },
        Army {
            groups: infection_groups,
        },
    )
}

fn part1((immune_system, infection): &(Army, Army)) -> usize {
    let mut immune_system = immune_system.clone();
    let mut infection = infection.clone();
    immune_system.sort_by_power();
    infection.sort_by_power();
    while immune_system.total_units() * infection.total_units() > 0 {
        // target selection
        break;
    }
    0
}

fn part2((immune_system, infection): &(Army, Army)) -> usize {
    0
}

#[derive(Clone, PartialEq, Eq)]
struct Group {
    units: usize,
    hp: usize,
    damage: usize,
    attack_type: String,
    initiative: usize,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+) units each with (\d+) hit points\s*(\(.*?\))?\s*with an attack that does (\d+) (\w+) damage at initiative (\d+)$").unwrap();
        let cap = re.captures(s).ok_or(())?;

        let units = cap[1].parse::<usize>().unwrap();
        let hp = cap[2].parse::<usize>().unwrap();
        let damage = cap[4].parse::<usize>().unwrap();
        let attack_type = cap[5].to_string();
        let initiative = cap[6].parse::<usize>().unwrap();

        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();
        if let Some(modifier_cap) = cap.get(3) {
            let modifier_str = modifier_cap
                .as_str()
                .trim_start_matches("(")
                .trim_end_matches(")");
            for section in modifier_str.split("; ") {
                if section.starts_with("weak to ") {
                    for weakness in section[8..].split(", ") {
                        weaknesses.insert(weakness.to_string());
                    }
                } else if section.starts_with("immune to ") {
                    for immunity in section[10..].split(", ") {
                        immunities.insert(immunity.to_string());
                    }
                } else {
                    panic!("Invalid section: {}", section);
                }
            }
        }

        Ok(Group {
            units,
            hp,
            damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
        })
    }
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }
}

#[derive(Clone)]
struct Army {
    groups: Vec<Group>,
}

impl Army {
    fn total_units(&self) -> usize {
        self.groups.iter().map(|g| g.units).sum()
    }

    fn sort_by_power(&mut self) {
        self.groups
            .sort_by_key(|g| (g.effective_power(), g.initiative));
    }
}
