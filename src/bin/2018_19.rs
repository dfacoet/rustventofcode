use std::{fs, str::FromStr};

const YEAR: u16 = 2018;
const DAY: u8 = 19;

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

fn parse_input(input: String) -> (usize, Vec<Instr>) {
    let mut lines = input.lines();
    let ip = lines
        .next()
        .unwrap()
        .trim_start_matches("#ip ")
        .parse()
        .unwrap();
    let instructions = lines.map(|l| Instr::from_str(l).unwrap()).collect();
    (ip, instructions)
}

fn part1((ip, instructions): &(usize, Vec<Instr>)) -> usize {
    let mut registers = [0; 6];

    while registers[*ip] < instructions.len() {
        instructions[registers[*ip]].apply(&mut registers);
        registers[*ip] += 1;
    }
    registers[0]
}

fn part2((ip, instructions): &(usize, Vec<Instr>)) -> usize {
    let mut registers = [0; 6];
    registers[0] = 1;

    // Solved by inspecting the instructions:
    // lines 17-end only run once, to initialise register[5]. Lines 27-end only run
    // if registers[0] = 1, making the value bigger for part2.
    // Initialisation finishes when instruction 1 is executed.
    while registers[*ip] != 1 {
        instructions[registers[*ip]].apply(&mut registers);
        registers[*ip] += 1;
    }
    // lines 1-16 are equivalent to
    // for x in 1..=registers[5] {
    //     for y in 1..=registers[5] {
    //         if x * y == registers[5] {
    //             registers[0] += x;
    //         }
    //     }
    // }
    // Which just computes the sum of divisors of register[5]
    for x in 1..=registers[5] {
        if registers[5] % x == 0 {
            registers[0] += x;
        }
    }
    registers[0]
}

struct Instr {
    op: Op,
    a: usize,
    b: usize,
    c: usize,
}

impl Instr {
    fn apply(&self, registers: &mut [usize; 6]) {
        registers[self.c] = match self.op {
            Op::Addr => registers[self.a] + registers[self.b],
            Op::Addi => registers[self.a] + self.b,
            Op::Mulr => registers[self.a] * registers[self.b],
            Op::Muli => registers[self.a] * self.b,
            Op::Setr => registers[self.a],
            Op::Seti => self.a,
            Op::Gtrr => (registers[self.a] > registers[self.b]) as usize,
            Op::Eqrr => (registers[self.a] == registers[self.b]) as usize,
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [op, a, b, c] = s.split(' ').collect::<Vec<_>>().as_slice() {
            let op = match *op {
                "addr" => Op::Addr,
                "addi" => Op::Addi,
                "mulr" => Op::Mulr,
                "muli" => Op::Muli,
                "setr" => Op::Setr,
                "seti" => Op::Seti,
                "gtrr" => Op::Gtrr,
                "eqrr" => Op::Eqrr,
                _ => {
                    return Err(());
                }
            };
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            let c = c.parse().unwrap();
            Ok(Instr { op, a, b, c })
        } else {
            Err(())
        }
    }
}

enum Op {
    // Commented out ops are not found in the input
    Addr,
    Addi,
    Mulr,
    Muli,
    // Banr,
    // Bani,
    // Borr,
    // Bori,
    Setr,
    Seti,
    // Gtir,
    // Gtri,
    Gtrr,
    // Eqir,
    // Eqri,
    Eqrr,
}
