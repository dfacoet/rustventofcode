use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs,
    iter::repeat,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

use itertools::Itertools;

const YEAR: u16 = 2018;
const DAY: u8 = 24;

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

fn parse_input(input: String) -> (Army, Army) {
    let mut lines = input.lines();
    match lines.next() {
        Some("Immune System:") => (),
        _ => panic!(""),
    }
    let immune_system = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| Group::from_str(line).unwrap())
        .collect();

    match lines.next() {
        Some("Infection:") => (),
        _ => panic!(""),
    }
    let infection = lines.map(|line| Group::from_str(line).unwrap()).collect();

    (immune_system, infection)
}

fn part1((immune_system, infection): &(Army, Army)) -> String {
    let mut immune_system = immune_system.clone();
    let mut infection = infection.clone();

    let mut i = 0;
    while !immune_system.is_empty() && !infection.is_empty() {
        println!(
            "Round {} - Immune System: {} Infection {}",
            i,
            immune_system.len(),
            infection.len()
        );
        let target_map = select_targets(&immune_system, &infection)
            .into_iter()
            .chain(select_targets(&infection, &immune_system))
            .collect::<HashMap<_, _>>();

        let attack_order: Vec<_> = immune_system
            .iter()
            .zip(repeat(ArmyType::ImmuneSystem))
            .chain(infection.iter().zip(repeat(ArmyType::Infection)))
            .sorted_by_key(|(group, _)| Reverse(group.initiative))
            .map(|(group, army)| (group.id.clone(), army))
            .collect();

        let mut to_remove = HashSet::new();
        for (attacker_id, side) in attack_order {
            // TODO: make side a field of Group
            let (attacking_army, defending_army) = match side {
                ArmyType::ImmuneSystem => (&immune_system, &mut infection),
                ArmyType::Infection => (&infection, &mut immune_system),
            };
            // TODO: Armies as hashmaps
            let attacker = attacking_army
                .iter()
                .find(|g| g.id == attacker_id)
                .expect("Attacker ID not found");
            if attacker.units == 0 {
                continue;
            }
            if let Some(target_id) = target_map.get(&attacker_id).expect("Attacker ID not found") {
                let target = defending_army
                    .iter_mut()
                    .find(|g| g.id == *target_id)
                    .expect("Target ID not found");
                let units_lost = damage(attacker, target) / target.hp;
                target.units = if units_lost < target.units {
                    target.units - units_lost
                } else {
                    to_remove.insert(target.id);
                    0
                };
            }
        }

        immune_system.retain(|g| !to_remove.contains(&g.id));
        infection.retain(|g| !to_remove.contains(&g.id));
        if immune_system.len() < 2 {
            println!("{:?}", immune_system);
            println!("{:?}", infection);
        };
        i += 1;
    }
    for group in immune_system {
        println!("{:?}", group)
    }
    for group in infection {
        println!("{:?}", group)
    }
    "".to_string()
}

fn part2((immune_system, infection): &(Army, Army)) -> String {
    "".to_string()
}
type AttackType = String; // Could be enum, but types are not specified outside input.

#[derive(Clone, Debug)]
struct Group {
    id: usize,
    units: u64,
    hp: u64,
    attack: u64,
    attack_type: AttackType,
    initiative: u64,
    weaknesses: HashSet<AttackType>,
    immunities: HashSet<AttackType>,
}

impl Group {
    fn effective_power(&self) -> u64 {
        self.attack * self.units
    }
}

#[derive(Debug)]
struct ParseGroupError;
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

impl FromStr for Group {
    type Err = ParseGroupError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let units_hp_re = regex::Regex::new(r"(\d+) units each with (\d+) hit points").unwrap();
        let attack_re =
            regex::Regex::new(r"with an attack that does (\d+) (\w+) damage at initiative (\d+)")
                .unwrap();
        let weaknesses_immunities_re = regex::Regex::new(r"\((.*?)\)").unwrap();

        let units_hp_caps = units_hp_re.captures(line).ok_or(ParseGroupError)?;
        let attack_caps = attack_re.captures(line).ok_or(ParseGroupError)?;

        let units = units_hp_caps[1].parse().map_err(|_| ParseGroupError)?;
        let hp = units_hp_caps[2].parse().map_err(|_| ParseGroupError)?;
        let attack = attack_caps[1].parse().map_err(|_| ParseGroupError)?;
        let attack_type = attack_caps[2].to_string();
        let initiative = attack_caps[3].parse().map_err(|_| ParseGroupError)?;

        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();

        if let Some(caps) = weaknesses_immunities_re.captures(line) {
            let details = &caps[1];
            for detail in details.split("; ") {
                if let Some(stripped) = detail.strip_prefix("weak to ") {
                    weaknesses.extend(stripped.split(", ").map(String::from));
                } else if let Some(stripped) = detail.strip_prefix("immune to ") {
                    immunities.extend(stripped.split(", ").map(String::from));
                }
            }
        }

        Ok(Group {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            units,
            hp,
            attack,
            attack_type,
            initiative,
            weaknesses,
            immunities,
        })
    }
}

type Army = Vec<Group>;

#[derive(Clone)]
enum ArmyType {
    ImmuneSystem,
    Infection,
}

fn select_targets(attackers: &Army, defendants: &Army) -> HashMap<usize, Option<usize>> {
    let mut attackers = attackers.clone();
    attackers.sort_by_key(|group| (group.effective_power(), group.initiative));
    let mut defendants = defendants.clone();

    let mut target_map = HashMap::new();

    for attacker in attackers {
        let target = match defendants.iter().max_by_key(|target| {
            (
                damage(&attacker, target),
                target.effective_power(),
                target.initiative,
            )
        }) {
            Some(target) if damage(&attacker, target) > 0 => Some(target.id),
            _ => None,
        };
        if let Some(target_id) = target {
            // target selected, remove from available targets
            defendants.retain(|d| d.id != target_id);
        }
        target_map.insert(attacker.id, target);
    }
    target_map
}

fn damage(attacker: &Group, target: &Group) -> u64 {
    if target.immunities.contains(&attacker.attack_type) {
        0
    } else if target.weaknesses.contains(&attacker.attack_type) {
        2 * attacker.effective_power()
    } else {
        attacker.effective_power()
    }
}
