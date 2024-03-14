use core::panic;
use itertools::Itertools;
use std::{collections::HashMap, fs};

use time::{format_description::well_known::Iso8601, PrimitiveDateTime};

const YEAR: u16 = 2018;
const DAY: u8 = 4;

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

fn parse_input(input: String) -> Guards {
    let mut lines: Vec<_> = input.lines().map(|line| line.to_string()).collect();
    // Sorting the timestamps as strings gives chronological order
    lines.sort();
    Guards::from_strings(&lines)
}

fn part1(guards: &Guards) -> usize {
    let sleepy_id = guards.find_most_sleepy();
    let sleepy_minute = guards
        .minutes
        .get(&sleepy_id)
        .unwrap()
        .iter()
        .position_max()
        .unwrap();
    sleepy_id * sleepy_minute
}

fn part2(guards: &Guards) -> usize {
    // TODO: consistent design with part1
    let mut consistent_id = 0;
    let mut minute = 0;
    let mut max_times = 0;
    for (guard_id, guard_mins) in guards.minutes.iter() {
        let best_minute = guard_mins.iter().position_max().unwrap();
        if guard_mins[best_minute] > max_times {
            max_times = guard_mins[best_minute];
            consistent_id = *guard_id;
            minute = best_minute;
        }
    }
    if consistent_id == 0 {
        panic!("No guard found");
    }
    consistent_id * minute
}

struct Guards {
    minutes: HashMap<usize, [usize; 60]>,
}

impl Guards {
    fn new() -> Guards {
        Guards {
            minutes: HashMap::new(), // guard_id -> minute -> count asleep
        }
    }

    fn from_strings(strings: &Vec<String>) -> Guards {
        let mut guards = Guards::new();
        let mut guard_id = 0;
        let mut sleep_start: Option<PrimitiveDateTime> = None;
        for s in strings {
            let mut split = s.splitn(2, ']');
            let timestamp = split.next().unwrap().trim_matches('[').replace(' ', "T");
            let timestamp = PrimitiveDateTime::parse(&timestamp, &Iso8601::DEFAULT).unwrap();
            let words: Vec<&str> = split.next().unwrap().trim_start().split(' ').collect();

            match words[..] {
                ["Guard", guard, "begins", "shift"] => {
                    guard_id = guard.trim_start_matches('#').parse().unwrap();
                    guards.minutes.entry(guard_id).or_insert([0; 60]);
                }
                ["falls", "asleep"] => match sleep_start {
                    Some(_) => panic!("Guard is already asleep"),
                    None => sleep_start = Some(timestamp),
                },
                ["wakes", "up"] => {
                    if guard_id == 0 {
                        panic!("Guard ID is not set")
                    }
                    if let Some(start) = sleep_start {
                        let start_min = start.minute() as usize;
                        let sleep_minutes = (timestamp - start).whole_minutes() as usize;
                        let guard_minutes = guards.minutes.get_mut(&guard_id).unwrap();
                        for i in 0..sleep_minutes {
                            guard_minutes[(start_min + i) % 60] += 1;
                        }
                        sleep_start = None;
                    } else {
                        panic!("Guard is not asleep");
                    }
                }
                _ => panic!("Invalid input {s}"),
            };
        }
        guards
    }

    fn find_most_sleepy(&self) -> usize {
        // TODO: functional?
        let mut highest = 0;
        let mut highest_id = 0;
        for (k, mins) in self.minutes.iter() {
            let sum = mins.iter().sum();
            if sum > highest {
                highest = sum;
                highest_id = *k;
            }
        }
        highest_id
    }
}
