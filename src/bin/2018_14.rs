use std::fs;

const YEAR: u16 = 2018;
const DAY: u8 = 14;

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

fn parse_input(input: String) -> usize {
    // Should return string or Vec of digits to include leading zeros
    input.lines().next().unwrap().parse().unwrap()
}

fn part1(input: &usize) -> String {
    let (mut c1, mut c2) = (0, 1);
    let mut scores = vec![3, 7];
    while scores.len() < input + 10 {
        let new_score = scores[c1] + scores[c2];
        scores.extend(
            new_score
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap() as usize),
        );
        c1 = (c1 + 1 + scores[c1]) % scores.len();
        c2 = (c2 + 1 + scores[c2]) % scores.len();
    }
    scores[scores.len() - 10..]
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
}
fn part2(input: &usize) -> usize {
    let target: Vec<usize> = input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();

    // Start a few steps later to avoid checks on first steps
    let mut scores = vec![3, 7, 1, 0, 1, 0, 1, 2, 4];
    let (mut c1, mut c2) = (4, 8);
    assert!(scores.len() >= target.len());

    loop {
        let new_score = scores[c1] + scores[c2];
        if new_score < 10 {
            scores.push(new_score);
        } else {
            scores.push(1);
            scores.push(new_score % 10);
            if scores[scores.len() - target.len() - 1..scores.len() - 1] == target {
                return scores.len() - target.len() - 1;
            }
        }
        if scores[scores.len() - target.len()..] == target {
            return scores.len() - target.len();
        }
        c1 = (c1 + 1 + scores[c1]) % scores.len();
        c2 = (c2 + 1 + scores[c2]) % scores.len();
    }
}
