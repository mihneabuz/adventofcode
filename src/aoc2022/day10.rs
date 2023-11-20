use std::fs;

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
    ins: Vec<Ins>,
}

const FILL: char = '█';
const EMPT: char = ' ';

impl CPU {
    fn new(ins: Vec<Ins>) -> Self {
        Self { reg: 1, pc: 0, ins }
    }

    fn run(&mut self) -> i32 {
        let mut clock = 1;
        let mut res = 0;

        while self.pc < self.ins.len() {
            if clock % 40 - 20 == 0 {
                res += self.reg * clock;
            }

            let pixel = (clock - 1) % 40;
            if pixel == 0 {
                println!();
            }

            if self.reg - 1 <= pixel && pixel <= self.reg + 1 {
                print!("{}", FILL);
            } else {
                print!("{}", EMPT);
            }

            match self.ins.get_mut(self.pc).unwrap() {
                Ins::Nop => self.pc += 1,
                Ins::Add(x, 1) => {
                    self.reg += *x;
                    self.pc += 1;
                }
                Ins::Add(_, c) => *c -= 1,
            }

            clock += 1;
        }

        println!();

        res
    }
}

fn main() {
    let instructions = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(parse_ins)
        .collect::<Vec<_>>();

    println!("{}", CPU::new(instructions).run());
}
