use std::{fs, str::FromStr, time::Instant};

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
    let mut c: usize = 0;
    let mut cc: u128 = 0;
    let start_time = Instant::now();
    while registers[*ip] < instructions.len() {
        instructions[registers[*ip]].apply(&mut registers);
        registers[*ip] += 1;
        c += 1;
        if c == 1_000_000_000 {
            c = 0;
            cc += 1;
            let total_seconds = start_time.elapsed().as_secs();
            let hh = total_seconds / 3600;
            let mm = (total_seconds % 3600) / 60;
            let ss = total_seconds % 60;
            let reg = format!("{:?}", registers);
            println!("{cc:<10} {reg:<40} time: {hh:02}:{mm:02}:{ss:02}");
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
            Op::Banr => registers[self.a] & registers[self.b],
            Op::Bani => registers[self.a] & self.b,
            Op::Borr => registers[self.a] | registers[self.b],
            Op::Bori => registers[self.a] | self.b,
            Op::Setr => registers[self.a],
            Op::Seti => self.a,
            Op::Gtir => (self.a > registers[self.b]) as usize,
            Op::Gtri => (registers[self.a] > self.b) as usize,
            Op::Gtrr => (registers[self.a] > registers[self.b]) as usize,
            Op::Eqir => (self.a == registers[self.b]) as usize,
            Op::Eqri => (registers[self.a] == self.b) as usize,
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
                "banr" => Op::Banr,
                "bani" => Op::Bani,
                "borr" => Op::Borr,
                "bori" => Op::Bori,
                "setr" => Op::Setr,
                "seti" => Op::Seti,
                "gtir" => Op::Gtir,
                "gtri" => Op::Gtri,
                "gtrr" => Op::Gtrr,
                "eqir" => Op::Eqir,
                "eqri" => Op::Eqri,
                "eqrr" => Op::Eqrr,
                _ => {
                    println!("{s}");
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
