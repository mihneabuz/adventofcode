use lib::aoc;
use lib::challenge::Challenge;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day11;

impl Challenge for Day11 {
    aoc!(year = 2022, day = 11);

    fn solve(input: String) -> (String, String) {
        let mut monkeys1 = input.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
        let mut monkeys2 = monkeys1.clone();
        let modulo: i64 = monkeys1
            .iter()
            .map(|m| m.test)
            .reduce(|acc, x| acc * x / gcd(acc, x))
            .unwrap();

        for _ in 0..20 {
            for i in 0..monkeys1.len() {
                for (dest, val) in monkeys1[i].throw(3, modulo) {
                    monkeys1[dest as usize].items.push(val);
                }
            }
        }
        let mut inspects1 = monkeys1.iter().map(|m| m.inspects).collect::<Vec<_>>();
        inspects1.sort();

        for _ in 0..10000 {
            for i in 0..monkeys2.len() {
                for (dest, val) in monkeys2[i].throw(1, modulo) {
                    monkeys2[dest as usize].items.push(val);
                }
            }
        }
        let mut inspects2 = monkeys2.iter().map(|m| m.inspects).collect::<Vec<_>>();
        inspects2.sort();

        let fst = inspects1[inspects1.len() - 1] * inspects1[inspects1.len() - 2];
        let snd = inspects2[inspects2.len() - 1] * inspects2[inspects2.len() - 2];

        (fst.to_string(), snd.to_string())
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    expr: Expr,
    test: i64,
    true_branch: i32,
    false_branch: i32,
    inspects: i64,
}

#[derive(Debug, Clone)]
struct Expr {
    op: char,
    operand1: Operand,
    operand2: Operand,
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Int(i64),
}

impl Expr {
    fn execute(&self, old: i64) -> i64 {
        let op1 = match self.operand1 {
            Operand::Old => old,
            Operand::Int(x) => x,
        };

        let op2 = match self.operand2 {
            Operand::Old => old,
            Operand::Int(x) => x,
        };

        match self.op {
            '+' => op1 + op2,
            '*' => op1 * op2,
            _ => unreachable!(),
        }
    }
}

impl Monkey {
    fn throw(&mut self, relief: i64, modulo: i64) -> Vec<(i32, i64)> {
        self.items
            .drain(..)
            .map(|item| {
                self.inspects += 1;
                let new = (self.expr.execute(item) / relief) % modulo;
                let next = if new % self.test == 0 {
                    self.true_branch
                } else {
                    self.false_branch
                };
                (next, new)
            })
            .collect()
    }
}

fn parse_monkey(s: &str) -> Monkey {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Monkey (\d):\n  Starting items: ([\d, ]*)\n  Operation: new = (.*)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap();
    }

    let capture = RE.captures(s).unwrap();

    let expr = parse_expr(capture.get(3).unwrap().as_str());
    let test = capture.get(4).unwrap().as_str().parse().unwrap();
    let true_branch = capture.get(5).unwrap().as_str().parse().unwrap();
    let false_branch = capture.get(6).unwrap().as_str().parse().unwrap();

    let items = capture
        .get(2)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    Monkey {
        items,
        expr,
        test,
        true_branch,
        false_branch,
        inspects: 0,
    }
}

fn parse_expr(s: &str) -> Expr {
    let splits = s.split_whitespace().collect::<Vec<_>>();

    let op = splits[1].chars().next().unwrap();

    let operand1 = match splits[0] {
        "old" => Operand::Old,
        d @ _ => Operand::Int(d.parse().unwrap()),
    };

    let operand2 = match splits[2] {
        "old" => Operand::Old,
        d @ _ => Operand::Int(d.parse().unwrap()),
    };

    Expr {
        op,
        operand1,
        operand2,
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    match a.cmp(&b) {
        std::cmp::Ordering::Equal => a,
        std::cmp::Ordering::Less => gcd(a, b % a),
        std::cmp::Ordering::Greater => gcd(b, a % b),
    }
}
