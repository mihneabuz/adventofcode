use lib::aoc;
use lib::challenge::Challenge;

pub struct Day10;

impl Challenge for Day10 {
    aoc!(year = 2022, day = 10);

    fn solve(input: String) -> (String, String) {
        let instructions = input.lines().map(parse_ins).collect::<Vec<_>>();
        let mut cpu = CPU::new();

        let fst = cpu.run(instructions).to_string();
        let snd = cpu.output().trim().to_string();

        (fst, snd)
    }
}

enum Ins {
    Nop,
    Add(i32, i32),
}

fn parse_ins(s: &str) -> Ins {
    if s.starts_with("noop") {
        Ins::Nop
    } else if s.starts_with("addx") {
        Ins::Add(s.split_once(" ").unwrap().1.parse().unwrap(), 2)
    } else {
        unreachable!()
    }
}

struct CPU {
    reg: i32,
    pc: usize,
    out: String,
}

const LBR: char = '\n';
const FILL: char = '█';
const EMPTY: char = ' ';

impl CPU {
    fn new() -> Self {
        Self {
            reg: 1,
            pc: 0,
            out: String::new(),
        }
    }

    fn run(&mut self, mut ins: Vec<Ins>) -> i32 {
        let mut clock = 1;
        let mut res = 0;

        while self.pc < ins.len() {
            if clock % 40 - 20 == 0 {
                res += self.reg * clock;
            }

            let pixel = (clock - 1) % 40;
            if pixel == 0 {
                self.out.push(LBR);
            }

            if self.reg - 1 <= pixel && pixel <= self.reg + 1 {
                self.out.push(FILL);
            } else {
                self.out.push(EMPTY);
            }

            match ins.get_mut(self.pc).unwrap() {
                Ins::Nop => self.pc += 1,
                Ins::Add(x, 1) => {
                    self.reg += *x;
                    self.pc += 1;
                }
                Ins::Add(_, c) => *c -= 1,
            }

            clock += 1;
        }

        res
    }

    fn output(&self) -> &str {
        &self.out
    }
}
