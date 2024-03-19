use std::{
    cmp::{max, min, Ordering},
    collections::HashSet,
    fs, usize,
};

const YEAR: u16 = 2018;
const DAY: u8 = 6;

fn main() {
    let input_file: String = format!("inputs/{YEAR}_{DAY:02}.txt");
    let input = fs::read_to_string(input_file).expect("Input file not found");
    let parsed_input = parse_input(input);

    let sol1 = part1(&parsed_input);
    let sol2 = part2(&parsed_input);

    println!("{YEAR} day {DAY}");
    println!("================");
    println!("Part 1: {sol1}");
    println!("Part 2: {sol2}");
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line: &str| {
            let parts: Vec<&str> = line.splitn(2, ',').collect();
            (
                parts[0].trim().parse::<usize>().unwrap(),
                parts[1].trim().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn part1(coords: &[(usize, usize)]) -> usize {
    let ((xmin, xmax), (ymin, ymax)) = get_range(coords);

    // naively iterate over all points in the grid and all centres
    // better idea: bubble out from each centre, checking for boundaries
    let mut areas: Vec<usize> = vec![0; coords.len()];
    let mut infinite = HashSet::new();
    for x in xmin..xmax + 1 {
        for y in ymin..ymax + 1 {
            let mut min_dist = usize::MAX;
            let mut closest = vec![];
            for (i, p) in coords.iter().enumerate() {
                let d = l1(*p, (x, y));
                match d.cmp(&min_dist) {
                    Ordering::Less => {
                        min_dist = d;
                        closest.clear();
                        closest.push(i);
                    }
                    Ordering::Equal => closest.push(i),
                    _ => {}
                }
            }
            if closest.len() == 1 {
                // Use Option instead?
                areas[closest[0]] += 1;
                if x == xmin || x == xmax || y == ymin || y == ymax {
                    infinite.insert(closest[0]);
                }
            }
        }
    }
    *areas
        .iter()
        .enumerate()
        .filter(|(i, _)| !infinite.contains(i))
        .map(|(_, a)| a)
        .max()
        .unwrap()
}

fn l1((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    (x1 as isize - x2 as isize).unsigned_abs() + (y1 as isize - y2 as isize).unsigned_abs()
}

fn get_range(coords: &[(usize, usize)]) -> ((usize, usize), (usize, usize)) {
    let (mut xmin, mut xmax) = (usize::MAX, 0);
    let (mut ymin, mut ymax) = (usize::MAX, 0);
    for (x, y) in coords {
        xmin = min(*x, xmin);
        xmax = max(*x, xmax);
        ymin = min(*y, ymin);
        ymax = max(*y, ymax);
    }
    ((xmin, xmax), (ymin, ymax))
}

fn part2(coords: &[(usize, usize)]) -> usize {
    let ((xmin, xmax), (ymin, ymax)) = get_range(coords);

    let mut c = 0;
    // Cartesian product iter?
    for x in xmin..xmax + 1 {
        for y in ymin..ymax + 1 {
            let total_distance = coords.iter().map(|p| l1(*p, (x, y))).sum::<usize>();
            if total_distance < 10_000 {
                c += 1;
            }
        }
    }
    c
}
