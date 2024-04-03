use std::fs;

const YEAR: u16 = 2018;
const DAY: u8 = 8;

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

fn parse_input(input: String) -> Node {
    let numbers: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let (l, root) = tree_from_numbers(&numbers);
    assert!(l == numbers.len());
    root
}

// Recursively build the tree
fn tree_from_numbers(numbers: &[usize]) -> (usize, Node) {
    let n_children = numbers[0];
    let n_data = numbers[1];

    let mut children = Vec::with_capacity(n_children);
    let mut c = 2;
    for _ in 0..n_children {
        let (consumed, child) = tree_from_numbers(&numbers[c..]);
        c += consumed;
        children.push(child);
    }

    let data = numbers[c..c + n_data].to_vec();
    c += n_data;
    (c, Node { children, data })
}

struct Node {
    children: Vec<Node>,
    data: Vec<usize>,
}

impl Node {
    fn sum(&self) -> usize {
        let children_sum: usize = self.children.iter().map(|n| n.sum()).sum();
        let node_sum: usize = self.data.iter().sum();
        children_sum + node_sum
    }

    fn value(&self) -> usize {
        let n = self.children.len();
        let value = if n == 0 {
            self.data.iter().sum()
        } else {
            self.data
                .iter()
                .map(|i| {
                    if *i < n + 1 {
                        self.children[*i - 1].value()
                    } else {
                        0
                    }
                })
                .sum()
        };
        value
    }
}

fn part1(root: &Node) -> usize {
    root.sum()
}

fn part2(root: &Node) -> usize {
    root.value()
}
