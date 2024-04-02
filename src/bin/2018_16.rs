use enum_iterator::Sequence;
use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

const YEAR: u16 = 2018;
const DAY: u8 = 16;

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

type Register = [usize; 4];

#[derive(Debug)]
struct Example {
    before: Register,
    after: Register,
    instr: Instr,
}

impl FromStr for Example {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let before = get_digits(lines.next().unwrap());
        let instr = Instr::from_str(lines.next().unwrap())?;
        let after = get_digits(lines.next().unwrap());
        Ok(Example {
            before,
            after,
            instr,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Instr {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.trim().split(' ').map(|s| s.parse().unwrap());
        let opcode = i.next().unwrap();
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        let c = i.next().unwrap();
        Ok(Instr { opcode, a, b, c })
    }
}

fn get_digits(s: &str) -> Register {
    let mut d = s.chars().filter_map(|c| {
        let d = c.to_digit(10)?;
        Some(d as usize)
    });
    [
        d.next().unwrap(),
        d.next().unwrap(),
        d.next().unwrap(),
        d.next().unwrap(),
    ]
}

fn parse_input(input: String) -> (Vec<Example>, Vec<Instr>) {
    match input.split("\n\n\n").collect::<Vec<_>>().as_slice() {
        [part1, part2] => {
            let examples = part1
                .split("\n\n")
                .map(|s| Example::from_str(s).unwrap())
                .collect::<Vec<_>>();
            let program = part2
                .trim()
                .lines()
                .map(|s| Instr::from_str(s).unwrap())
                .collect();
            (examples, program)
        }
        _ => panic!(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Sequence)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn apply_op(op: &Op, register: Register, instr: Instr) -> Register {
    let mut register = register;
    let (a, b, c) = (instr.a, instr.b, instr.c);
    register[c] = match *op {
        Op::Addr => register[a] + register[b],
        Op::Addi => register[a] + b,
        Op::Mulr => register[a] * register[b],
        Op::Muli => register[a] * b,
        Op::Banr => register[a] & register[b],
        Op::Bani => register[a] & b,
        Op::Borr => register[a] | register[b],
        Op::Bori => register[a] | b,
        Op::Setr => register[a],
        Op::Seti => a,
        Op::Gtir => (a > register[b]) as usize,
        Op::Gtri => (register[a] > b) as usize,
        Op::Gtrr => (register[a] > register[b]) as usize,
        Op::Eqir => (a == register[b]) as usize,
        Op::Eqri => (register[a] == b) as usize,
        Op::Eqrr => (register[a] == register[b]) as usize,
    };
    register
}

fn check_op_match(op: &Op, example: &Example) -> bool {
    example.after == apply_op(op, example.before, example.instr)
}

fn count_matching_ops(example: &Example) -> usize {
    enum_iterator::all::<Op>()
        .filter(|op| check_op_match(op, example))
        .count()
}

fn part1((examples, _): &(Vec<Example>, Vec<Instr>)) -> usize {
    examples
        .iter()
        .filter(|e| count_matching_ops(e) >= 3)
        .count()
}

fn part2((examples, program): &(Vec<Example>, Vec<Instr>)) -> usize {
    // opcode -> {possible operations}
    let mut op_map = HashMap::new();
    for i in 0..16 {
        let all_ops = enum_iterator::all::<Op>().collect::<HashSet<Op>>();
        op_map.insert(i, all_ops);
    }

    // filter out operations not matching the examples
    for example in examples {
        op_map
            .get_mut(&example.instr.opcode)
            .unwrap()
            .retain(|op| apply_op(op, example.before, example.instr) == example.after);
    }

    // Filter out ops for which we found the opcode, until
    // all values of op_map have length one
    let mut assigned_ops: HashSet<_> = op_map
        .values()
        .filter(|ops| ops.len() == 1)
        .map(|ops| *ops.iter().next().unwrap())
        .collect();

    while assigned_ops.len() < 16 {
        op_map
            .values_mut()
            .map(|possible_ops| {
                if possible_ops.len() > 1 {
                    possible_ops.retain(|op| !assigned_ops.contains(op));

                    if possible_ops.len() == 1 {
                        assigned_ops.insert(*possible_ops.iter().next().unwrap());
                    }
                }
            })
            .count();
    }

    let op_map: HashMap<_, _> = op_map
        .iter()
        .map(|(i, ops)| (i, ops.iter().next().unwrap()))
        .collect();

    let mut register = [0, 0, 0, 0];
    for instr in program {
        let op = &op_map[&instr.opcode];
        register = apply_op(op, register, *instr);
    }
    register[0]
}
