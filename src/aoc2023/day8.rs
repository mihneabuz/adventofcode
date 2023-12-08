use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

use lib::{aoc, challenge::Challenge};

pub struct Day8;

impl Challenge for Day8 {
    aoc!(year = 2023, day = 8);

    fn solve(input: String) -> (String, String) {
        let (steps, map) = input.split_once("\n\n").unwrap();
        let map = Map::from_iterator(map.lines().map(parse));

        let fst = steps
            .chars()
            .cycle()
            .zip(1..)
            .try_fold("AAA", |node, (step, count)| {
                let next = map.step(node, step);
                if next == "ZZZ" {
                    Err(count)
                } else {
                    Ok(next)
                }
            })
            .unwrap_err();

        let snd = map
            .nodes()
            .filter(|k| k.ends_with('A'))
            .map(|mut node| {
                let cycle = steps
                    .chars()
                    .cycle()
                    .zip(1..)
                    .filter(|(step, count)| {
                        node = map.step(node, *step);
                        node.ends_with('Z') && count % steps.len() == 0
                    })
                    .map(|(_, count)| count)
                    .take(2)
                    .collect_vec();

                cycle[1] - cycle[0]
            })
            .reduce(|acc, count| acc.lcm(&count))
            .unwrap();

        (fst.to_string(), snd.to_string())
    }
}

struct Map<'a> {
    inner: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    fn from_iterator(iter: impl Iterator<Item = (&'a str, (&'a str, &'a str))>) -> Self {
        Self {
            inner: iter.collect(),
        }
    }

    fn nodes(&self) -> impl Iterator<Item = &str> {
        self.inner.keys().copied()
    }

    fn step(&self, from: &str, dir: char) -> &str {
        let adj = self.inner.get(from).unwrap();
        match dir {
            'L' => adj.0,
            'R' => adj.1,
            _ => unreachable!(),
        }
    }
}

fn parse(line: &str) -> (&str, (&str, &str)) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    };

    let matches = RE
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|c| c.unwrap().as_str())
        .collect_vec();

    (matches[0], (matches[1], matches[2]))
}
