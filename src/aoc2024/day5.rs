use lib::{helpers::unchecked_parse, prelude::*};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::{cmp::Ordering, convert::identity};

pub struct Day5;

impl Challenge for Day5 {
    aoc!(year = 2024, day = 5);

    fn solve(input: String) -> (String, String) {
        let (rules, updates) = input.split_once("\n\n").unwrap();

        let rules = rules
            .lines()
            .map(|line| {
                let (x, y) = line.split_once("|").unwrap();
                (unchecked_parse::<u32>(x), unchecked_parse::<u32>(y))
            })
            .collect_vec();

        let updates = updates
            .lines()
            .map(|line| line.split(',').map(unchecked_parse::<u32>).collect_vec())
            .collect_vec();

        let before: HashMap<u32, HashSet<u32>> =
            rules.iter().fold(HashMap::new(), |mut deps, &(x, y)| {
                deps.entry(x)
                    .and_modify(|v| {
                        v.insert(y);
                    })
                    .or_insert(HashSet::from_iter(Some(y)));
                deps
            });

        let is_ordered = |update: &[u32]| {
            update
                .iter()
                .scan(HashSet::new(), |set, page| {
                    if let Some(deps) = before.get(page) {
                        if deps.intersection(set).count() > 0 {
                            return Some(false);
                        }
                    }

                    set.insert(*page);
                    Some(true)
                })
                .all(identity)
        };

        let cmp_order = |x: u32, y: u32| {
            if before.get(&x).is_some_and(|deps| deps.contains(&y)) {
                Ordering::Less
            } else if before.get(&y).is_some_and(|deps| deps.contains(&x)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };

        let (res1, res2) = updates
            .into_iter()
            .fold((0, 0), |(res1, res2), mut update| {
                if is_ordered(&update) {
                    (res1 + update[update.len() / 2], res2)
                } else {
                    update.sort_by(|x, y| cmp_order(*x, *y));
                    (res1, res2 + update[update.len() / 2])
                }
            });

        (res1.to_string(), res2.to_string())
    }
}
