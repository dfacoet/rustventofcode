use std::fs;

const YEAR: u16 = 2018;
const DAY: u8 = 11;

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
    input.lines().next().unwrap().parse().unwrap()
}

fn part1(serial_number: &usize) -> String {
    // brute force
    let mut grid = [[0; 300]; 300];
    for i in 1..=300 {
        for j in 1..=300 {
            grid[i - 1][j - 1] =
                ((((((i + 10) * j) + serial_number) * (i + 10)) / 100) % 10) as i32 - 5
        }
    }

    let mut best_cell = [0, 0];
    let mut max_total_power = i32::MIN;
    for i in 0..298 {
        for j in 0..298 {
            let total_power = (0..3)
                .map(|d| grid[i + d][j..j + 3].iter().sum::<i32>())
                .sum::<i32>();
            if total_power > max_total_power {
                max_total_power = total_power;
                best_cell = [i + 1, j + 1];
            }
        }
    }
    format!("{},{}", best_cell[0], best_cell[1])
}

fn part2(serial_number: &usize) -> String {
    // brute force with repeated sums is still doable (few s) in rust
    // precomputing sums would be faster, but the numbers involved seem too big
    let mut grid = [[0; 300]; 300];
    for i in 1..=300 {
        for j in 1..=300 {
            grid[i - 1][j - 1] =
                ((((((i + 10) * j) + serial_number) * (i + 10)) / 100) % 10) as i32 - 5
        }
    }

    let mut best_cell = [0, 0];
    let mut best_size = 0;
    let mut max_total_power = i32::MIN;
    for size in 1..=300 {
        for i in 0..301 - size {
            for j in 0..301 - size {
                let total_power = (0..size)
                    .map(|d| grid[i + d][j..j + size].iter().sum::<i32>())
                    .sum::<i32>();
                if total_power > max_total_power {
                    max_total_power = total_power;
                    best_cell = [i + 1, j + 1];
                    best_size = size;
                }
            }
        }
    }
    format!("{},{},{}", best_cell[0], best_cell[1], best_size)
}
