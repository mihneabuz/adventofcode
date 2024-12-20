use lib::{helpers, prelude::*};

use std::collections::VecDeque;

use itertools::Itertools;

pub struct Day20;

impl Challenge for Day20 {
    aoc!(year = 2024, day = 20);

    fn solve(input: String) -> (String, String) {
        let map = helpers::Map::from_text(&input);
        let mut costs = helpers::Map::<u32>::new(map.height(), map.width());

        let start = map.findi(|b| *b == b'S').unwrap();
        let mut queue = VecDeque::from([start]);
        while let Some(pos) = queue.pop_front() {
            for (_, neigh) in map.neighs4i(pos) {
                if map[neigh] != b'#' && costs[neigh] == 0 {
                    costs[neigh] = costs[pos] + 1;
                    queue.push_back(neigh);
                }
            }
        }

        let res1 = map
            .cellsi()
            .filter(|(_, b)| **b == b'#')
            .map(|(pos, _)| {
                map.neighs4i(pos)
                    .map(|(_, pos)| pos)
                    .filter(|neigh| map[*neigh] != b'#')
                    .tuple_combinations::<(_, _)>()
                    .map(|(n1, n2)| costs[n1].abs_diff(costs[n2]) - 2)
                    .filter(|save| *save >= 100)
                    .count()
            })
            .sum::<usize>();

        const CHEAT: i32 = 20;

        let res2 = map
            .cellsi()
            .filter(|(_, b)| **b != b'#')
            .fold(0, |mut acc, (pos, _)| {
                for di in -CHEAT..=CHEAT {
                    let diff = CHEAT - di.abs();
                    let i = pos.0 + di;

                    for dj in -diff..=diff {
                        let j = pos.1 + dj;

                        if map.valid((i, j)) && map[(i, j)] != b'#' {
                            let dist = di.abs() + dj.abs();
                            let save = (costs[(i, j)] - costs[pos]) as i32 - dist;
                            if save >= 100 {
                                acc += 1;
                            }
                        }
                    }
                }

                acc
            });

        (res1.to_string(), res2.to_string())
    }
}
