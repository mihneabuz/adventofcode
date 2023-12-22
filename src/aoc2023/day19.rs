use std::{array, cmp::Ordering, collections::HashMap, ops::Range};

use itertools::Itertools;

use lib::{aoc, challenge::Challenge};

pub struct Day19;

impl Challenge for Day19 {
    aoc!(year = 2023, day = 19);

    fn solve(input: String) -> (String, String) {
        let (rules, parts) = input.split_once("\n\n").unwrap();

        let rules = rules.lines().map(parse_rule).collect::<HashMap<_, _>>();
        let parts = parts.lines().map(parse_part).collect_vec();

        let fst = parts
            .into_iter()
            .map(|part| {
                let mut rule = rules.get("in").unwrap();
                loop {
                    let jump = rule
                        .checks
                        .iter()
                        .find(|(rating, order, value, _)| part.get(rating).cmp(value) == *order)
                        .map(|(_, _, _, jump)| *jump)
                        .unwrap_or(rule.fallback);

                    match jump {
                        "A" => return part.total(),
                        "R" => return 0,

                        other => rule = rules.get(other).unwrap(),
                    }
                }
            })
            .sum::<usize>();

        fn accepted(mut ranges: [Range<u32>; 4], rule: &str, rules: &HashMap<&str, Rule>) -> usize {
            match rule {
                "A" => {
                    return ranges
                        .into_iter()
                        .map(|range| range.try_len().unwrap())
                        .product()
                }
                "R" => return 0,
                _ => {}
            };

            let rule = rules.get(rule).unwrap();

            let mut total = 0;
            for (rating, order, value, jump) in &rule.checks {
                let idx = (*rating as u8) as usize;
                let value = *value as u32;

                if !ranges[idx].contains(&value) {
                    continue;
                }

                let mut next = ranges.clone();
                match order {
                    Ordering::Less => {
                        next[idx] = next[idx].start..value;
                        ranges[idx] = value..ranges[idx].end;
                    }

                    Ordering::Greater => {
                        next[idx] = value + 1..next[idx].end;
                        ranges[idx] = ranges[idx].start..value + 1;
                    }

                    _ => unreachable!(),
                }

                total += accepted(next, jump, rules);
            }

            total + accepted(ranges, rule.fallback, rules)
        }

        let snd = accepted(array::from_fn(|_| (1..4001)), "in", &rules);

        (fst.to_string(), snd.to_string())
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn total(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, rating: &Rating) -> usize {
        match rating {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Rating {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

struct Rule<'a> {
    checks: Vec<(Rating, Ordering, usize, &'a str)>,
    fallback: &'a str,
}

fn parse_rule(input: &str) -> (&str, Rule) {
    let (name, rest) = input.trim_end_matches('}').split_once('{').unwrap();

    let mut checks = Vec::new();
    for split in rest.split(',') {
        match split.split_once(':') {
            Some((cond, jump)) => {
                let rating = match cond.as_bytes()[0] {
                    b'x' => Rating::X,
                    b'm' => Rating::M,
                    b'a' => Rating::A,
                    b's' => Rating::S,
                    _ => unreachable!(),
                };

                let order = match cond.as_bytes()[1] {
                    b'<' => Ordering::Less,
                    b'>' => Ordering::Greater,
                    _ => unreachable!(),
                };

                let value = cond.split_at(2).1.parse().unwrap();

                checks.push((rating, order, value, jump));
            }
            None => {
                return (
                    name,
                    Rule {
                        checks,
                        fallback: split,
                    },
                )
            }
        }
    }

    unreachable!()
}

fn parse_part(input: &str) -> Part {
    let mut attrs = input
        .trim_matches('{')
        .trim_end_matches('}')
        .split(',')
        .flat_map(|attr| attr.split_once('=').unwrap().1.parse::<usize>());

    let x = attrs.next().unwrap();
    let m = attrs.next().unwrap();
    let a = attrs.next().unwrap();
    let s = attrs.next().unwrap();

    Part { x, m, a, s }
}
