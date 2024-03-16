use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    hash::Hash,
};

const YEAR: u16 = 2018;
const DAY: u8 = 7;

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

fn parse_input(input: String) -> HashMap<char, Vec<char>> {
    let mut dag_dict = HashMap::new();

    let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let [parent, child] = [&caps[1], &caps[2]].map(|s| s.chars().next().unwrap());
            dag_dict.entry(parent).or_insert(Vec::new()).push(child);
        }
    }

    dag_dict
}

fn part1(dag: &HashMap<char, Vec<char>>) -> String {
    let mut rev_dag = invert_dag(dag);

    // Put the roots in a priority queue
    // Need Reverse when pushing and .0 when popping to get min heap
    let mut roots = BinaryHeap::new();
    for c in dag.keys() {
        if !rev_dag.contains_key(c) {
            roots.push(Reverse(c));
        }
    }

    let mut topological_sort = String::new();
    while let Some(root) = roots.pop() {
        let root = root.0;
        topological_sort.push(*root);
        if let Some(children) = dag.get(root) {
            for c in children {
                let coparents = rev_dag.get_mut(c).unwrap();
                coparents.remove(root);
                if coparents.is_empty() {
                    roots.push(Reverse(c));
                }
            }
        }
    }

    topological_sort
}

fn part2(dag: &HashMap<char, Vec<char>>) -> usize {
    let mut rev_dag = invert_dag(dag);

    let mut roots = BinaryHeap::new();
    for c in dag.keys() {
        if !rev_dag.contains_key(c) {
            roots.push(Reverse(c));
        }
    }
    // Reverse((completion time, &letter))
    let mut in_progress = BinaryHeap::from([Reverse((0, &' '))]);

    let workers = 5;
    let mut t = 0;

    while let Some(step) = in_progress.pop() {
        let (next, completed) = step.0;
        t = next;
        if let Some(children) = dag.get(completed) {
            for c in children {
                let coparents = rev_dag.get_mut(c).unwrap();
                coparents.remove(completed);
                if coparents.is_empty() {
                    roots.push(Reverse(c));
                }
            }
        }
        while !roots.is_empty() && in_progress.len() < workers {
            let c = roots.pop().unwrap().0;
            in_progress.push(Reverse((t + duration(c), c)))
        }
    }
    t
}

fn invert_dag<T: Eq + Hash + Clone>(dag: &HashMap<T, Vec<T>>) -> HashMap<T, HashSet<T>> {
    let mut rev_dag = HashMap::new();
    for (p, children) in dag.iter() {
        for c in children {
            rev_dag
                .entry(c.clone())
                .or_insert(HashSet::new())
                .insert(p.clone());
        }
    }
    rev_dag
}

fn duration(c: &char) -> usize {
    *c as usize - 'A' as usize + 61
}
