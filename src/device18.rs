use std::str::FromStr;

pub fn parse_input(input: String) -> (usize, Vec<Instr>) {
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

pub struct Instr {
    op: Op,
    a: usize,
    b: usize,
    c: usize,
}

impl Instr {
    pub fn apply(&self, registers: &mut [usize; 6]) {
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
                    println!("Unknown op: {op}");
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
