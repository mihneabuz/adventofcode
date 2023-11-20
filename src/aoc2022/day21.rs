use std::collections::HashMap;
use std::fs;

enum Yell<'a> {
    Number(i64),
    Expr(&'a str, &'a str, char),
}

type Monkeys<'a> = HashMap<&'a str, Yell<'a>>;

fn parse_yell(s: &str) -> Yell {
    if let Ok(x) = s.parse() {
        Yell::Number(x)
    } else {
        let ss = s.split(" ").collect::<Vec<_>>();
        Yell::Expr(ss[0], ss[2], ss[1].chars().next().unwrap())
    }
}

fn parse_monkey(s: &str) -> (&str, Yell) {
    let (monkey, yell) = s.split_once(": ").unwrap();
    (monkey, parse_yell(yell))
}

fn reduce(monkeys: &mut Monkeys) {
    let keys = monkeys
        .iter()
        .filter_map(|(key, val)| match val {
            Yell::Number(_) => None,
            _ => Some(key),
        })
        .copied()
        .collect::<Vec<_>>();

    for key in keys {
        if let Yell::Expr(a, b, op) = monkeys.get(key).unwrap() {
            if let Yell::Number(a) = monkeys.get(a).unwrap() {
                if let Yell::Number(b) = monkeys.get(b).unwrap() {
                    monkeys.insert(key,
                        Yell::Number(match op {
                            '+' => a + b,
                            '-' => a - b,
                            '*' => a * b,
                            '/' => a / b,
                            _ => unreachable!(),
                        }),
                    );
                }
            }
        }
    }
}

fn value(monkeys: &Monkeys, key: &str, me: &str) -> Option<i64> {
    if key == me {
        None
    } else {
        match monkeys.get(key).unwrap() {
            Yell::Number(x) => Some(*x),
            Yell::Expr(a, b, op) => {
                let (a, b) = (value(monkeys, a, me)?, value(monkeys, b, me)?);
                Some(match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => unreachable!(),
                })
            }
        }
    }
}

fn solve(monkeys: &Monkeys, key: &str, res: i64, me: &str) -> i64 {
    if key == me {
        res
    } else if let Yell::Expr(a, b, op) = monkeys.get(key).unwrap() {
        let val_a = value(monkeys, a, me);
        let val_b = value(monkeys, b, me);

        return match (val_a, op, val_b) {
            (Some(x), '+', None) => solve(monkeys, b, res - x, me),
            (None, '+', Some(x)) => solve(monkeys, a, res - x, me),

            (Some(x), '*', None) => solve(monkeys, b, res / x, me),
            (None, '*', Some(x)) => solve(monkeys, a, res / x, me),

            (Some(x), '-', None) => solve(monkeys, b, x - res, me),
            (None, '-', Some(x)) => solve(monkeys, a, x + res, me),

            (Some(x), '/', None) => solve(monkeys, b, x / res, me),
            (None, '/', Some(x)) => solve(monkeys, a, x * res, me),

            _ => unreachable!()
        }
    } else {
        unreachable!()
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let res1 = {
        let mut monkeys = content.lines().map(parse_monkey).collect::<HashMap<_, _>>();
        loop {
            match monkeys.get("root").unwrap() {
                Yell::Number(x) => break *x,
                _ => reduce(&mut monkeys),
            }
        }
    };

    let res2 = {
        let monkeys2 = content.lines().map(parse_monkey).collect::<HashMap<_, _>>();
        if let Yell::Expr(a, b, _) = monkeys2.get("root").unwrap() {
            let mut values = vec![(value(&monkeys2, a, "humn"), a), (value(&monkeys2, b, "humn"), b)];
            values.sort();
            solve(&monkeys2, *values[0].1, values[1].0.unwrap(), "humn")
        } else {
            unreachable!()
        }
    };

    println!("1: {}", res1);
    println!("2: {}", res2);
}
