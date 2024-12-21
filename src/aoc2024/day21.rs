use lib::prelude::*;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::iter::repeat;

struct Keypad(HashMap<u8, (i32, i32)>, HashSet<(i32, i32)>);

impl Keypad {
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = (u8, (i32, i32))>,
    {
        let mut map = HashMap::new();
        let mut set = HashSet::new();

        for (b, pos) in iter {
            map.insert(b, pos);
            set.insert(pos);
        }

        Self(map, set)
    }

    fn get(&self, char: &u8) -> Option<(i32, i32)> {
        self.0.get(char).copied()
    }

    fn contains(&self, pos: &(i32, i32)) -> bool {
        self.1.contains(pos)
    }

    fn keys(&self) -> impl Iterator<Item = &u8> {
        self.0.keys()
    }
}

fn solve(code: &[u8], start: u8, keypad: &Keypad) -> Vec<Vec<u8>> {
    let mut solutions = vec![vec![]];
    let mut start = keypad.get(&start).unwrap();

    for button in code {
        let target = keypad.get(button).unwrap();
        let diff = (target.0 - start.0, target.1 - start.1);

        let mut next: Vec<Vec<u8>> = vec![];
        let di = if diff.0 > 0 { b'v' } else { b'^' };
        let dj = if diff.1 > 0 { b'>' } else { b'<' };

        if diff.0 == 0 {
            next.push(vec![dj; diff.1.unsigned_abs() as usize]);
        } else if diff.1 == 0 {
            next.push(vec![di; diff.0.unsigned_abs() as usize]);
        } else {
            if keypad.contains(&(start.0, target.1)) {
                next.push(Vec::from_iter(
                    repeat(dj)
                        .take(diff.1.unsigned_abs() as usize)
                        .chain(repeat(di).take(diff.0.unsigned_abs() as usize)),
                ));
            }

            if keypad.contains(&(target.0, start.1)) {
                next.push(Vec::from_iter(
                    repeat(di)
                        .take(diff.0.unsigned_abs() as usize)
                        .chain(repeat(dj).take(diff.1.unsigned_abs() as usize)),
                ));
            }
        }

        solutions = solutions
            .iter()
            .flat_map(|prev| {
                next.iter().map(|next| {
                    let mut new = prev.clone();
                    new.extend(next);
                    new.push(b'A');
                    new
                })
            })
            .collect_vec();

        start = target;
    }

    solutions
}

pub struct Day21;

impl Challenge for Day21 {
    aoc!(year = 2024, day = 21);

    fn solve(input: String) -> (String, String) {
        let numeric = Keypad::from_iter(
            [
                (b'7', (0, 0)),
                (b'8', (0, 1)),
                (b'9', (0, 2)),
                //
                (b'4', (1, 0)),
                (b'5', (1, 1)),
                (b'6', (1, 2)),
                //
                (b'1', (2, 0)),
                (b'2', (2, 1)),
                (b'3', (2, 2)),
                //
                (b'0', (3, 1)),
                (b'A', (3, 2)),
            ]
            .into_iter(),
        );

        let directional = Keypad::from_iter(
            [
                (b'^', (0, 1)),
                (b'A', (0, 2)),
                //
                (b'<', (1, 0)),
                (b'v', (1, 1)),
                (b'>', (1, 2)),
            ]
            .into_iter(),
        );

        let mut cache = HashMap::new();
        for prev in directional.0.keys() {
            for next in directional.0.keys() {
                cache.insert((*prev, *next), solve(&[*next], *prev, &directional));
            }
        }

        let mut expanded = HashMap::new();
        for prev in directional.keys() {
            for next in directional.keys() {
                let len = solve(&[*next], *prev, &directional)
                    .into_iter()
                    .map(|sol| sol.len())
                    .min()
                    .unwrap();

                expanded.insert((0u8, *prev, *next), len);
            }
        }

        let min_len = |sols: &Vec<Vec<u8>>, expanded: &HashMap<(u8, u8, u8), usize>, step| {
            sols.iter()
                .map(|sol| {
                    sol.windows(2)
                        .fold(expanded[&(step, b'A', sol[0])], |acc, sol| {
                            let (prev, next) = (sol[0], sol[1]);
                            acc + expanded[&(step, prev, next)]
                        })
                })
                .min()
                .unwrap()
        };

        for step in 1..25 {
            for prev in directional.0.keys() {
                for next in directional.0.keys() {
                    let sols = &cache[&(*prev, *next)];
                    let len = min_len(sols, &expanded, step - 1);
                    expanded.insert((step, *prev, *next), len);
                }
            }
        }

        let (res1, res2) = input.lines().fold((0, 0), |(acc1, acc2), code| {
            let sols = solve(code.as_bytes(), b'A', &numeric);

            let len1 = min_len(&sols, &expanded, 1);
            let len2 = min_len(&sols, &expanded, 24);

            let num = code[..3].parse::<usize>().unwrap();

            (acc1 + num * len1, acc2 + num * len2)
        });

        (res1.to_string(), res2.to_string())
    }
}
