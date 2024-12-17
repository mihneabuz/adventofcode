use lib::{helpers, prelude::*};

use itertools::Itertools;

struct Computer {
    regs: Vec<u64>,
    prog: Vec<u8>,
    pc: usize,
}

impl Computer {
    fn new(regs: Vec<u64>, prog: Vec<u8>) -> Self {
        Self { regs, prog, pc: 0 }
    }

    fn step(&mut self, out: &mut Vec<u8>) -> bool {
        if self.pc >= self.prog.len() {
            return false;
        }

        let opcode = self.prog[self.pc];

        let literal = self.prog[self.pc + 1] as u64;
        let combo = match literal {
            0..=3 => literal,
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            _ => unreachable!(),
        };

        self.pc += 2;

        match opcode {
            0 => {
                self.regs[0] /= 2u64.pow(combo as u32);
            }
            1 => {
                self.regs[1] ^= literal;
            }
            2 => {
                self.regs[1] = combo % 8;
            }
            3 => {
                if self.regs[0] != 0 {
                    self.pc = literal as usize;
                }
            }
            4 => {
                self.regs[1] ^= self.regs[2];
            }
            5 => {
                out.push((combo % 8) as u8);
            }
            6 => {
                self.regs[1] = self.regs[0] / 2u64.pow(combo as u32);
            }
            7 => {
                self.regs[2] = self.regs[0] / 2u64.pow(combo as u32);
            }
            _ => unreachable!(),
        }

        true
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.step(&mut output) {}
        output
    }

    fn reset(&mut self, a: u64) -> &mut Self {
        self.regs[0] = a;
        self.pc = 0;
        self
    }

    fn solve(&mut self) -> u64 {
        self._solve(&mut vec![0; self.prog.len()], 0)
    }

    fn _solve(&mut self, sol: &mut Vec<u8>, len: usize) -> u64 {
        if len == sol.len() {
            return self._reg(sol);
        }

        for k in 0..8 {
            sol[len] = k;

            let a = self._reg(sol);
            let output = self.reset(a).run();

            let idx = sol.len() - len - 1;
            if output.get(idx).is_some_and(|b| *b == self.prog[idx]) {
                let res = self._solve(sol, len + 1);
                if res > 0 {
                    return res;
                }
            }
        }

        0
    }

    fn _reg(&self, terts: &[u8]) -> u64 {
        assert_eq!(self.prog.len(), terts.len());
        terts
            .iter()
            .copied()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, t)| acc + 2u64.pow(3 * i as u32) * t as u64)
    }
}

pub struct Day17;

impl Challenge for Day17 {
    aoc!(year = 2024, day = 17);

    fn solve(input: String) -> (String, String) {
        let (registers, program) = input.split_once("\n\n").unwrap();
        let program = program.split_once(": ").unwrap().1;

        let regs: Vec<u64> = registers
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
            .collect_vec();

        assert_eq!(regs.len(), 3);

        let prog = program
            .as_bytes()
            .iter()
            .step_by(2)
            .map(|b| *b - b'0')
            .collect_vec();

        let mut comp = Computer::new(regs, prog.clone());
        let res1 = comp.run();
        let res2 = comp.solve();

        let res1 = helpers::join(res1.into_iter(), ",");

        (res1.to_string(), res2.to_string())
    }
}
