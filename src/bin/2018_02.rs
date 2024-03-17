use rustventofcode::parse::to_strings;
use std::collections::HashMap;
use std::fs;

const YEAR: u16 = 2018;
const DAY: u8 = 2;
fn main() {
    let input_file = format!("inputs/{YEAR}_{DAY:02}.txt");
    let input = fs::read_to_string(input_file).expect("Input file not found");
    let parsed_input = to_strings(input);

    let sol1 = part1(&parsed_input);
    let sol2 = part2(&parsed_input);

    println!("{YEAR} day {DAY}");
    println!("================");
    println!("Part 1: {sol1}");
    println!("Part 2: {sol2}");
}

fn part1(ids: &[String]) -> i32 {
    let mut count_two: u16 = 0;
    let mut count_three: u16 = 0;

    for id in ids.iter() {
        let counter = count_chars(id);

        if counter.values().any(|&count| count == 2) {
            count_two += 1;
        }
        if counter.values().any(|&count| count == 3) {
            count_three += 1;
        }
    }

    (count_two * count_three) as i32
}

fn count_chars(s: &str) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    for c in s.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    counter
}

fn part2(strings: &Vec<String>) -> String {
    for i in 0..strings.len() {
        for j in i + 1..strings.len() {
            if differ_by_one(&strings[i], &strings[j]) {
                return common_chars(&strings[i], &strings[j]);
            }
        }
    }
    panic!("No match found")
}

fn differ_by_one(s1: &str, s2: &str) -> bool {
    s1.chars()
        .zip(s2.chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count()
        == 1
}

fn common_chars(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|&(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect()
}
