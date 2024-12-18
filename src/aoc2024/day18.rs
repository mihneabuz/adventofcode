use lib::{
    helpers::{self, unchecked_parse},
    prelude::*,
};

use std::collections::VecDeque;

pub struct Day18;

impl Challenge for Day18 {
    aoc!(year = 2024, day = 18);

    fn solve(input: String) -> (String, String) {
        let n = 71;
        let m = 1024;
        let mut positions = input.lines().map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (unchecked_parse::<i32>(y), unchecked_parse::<i32>(x))
        });

        let mut map = helpers::Map::new(n, n);
        map.fill(b'.');
        for pos in (&mut positions).take(m) {
            map[pos] = b'#';
        }

        let solve = |map: &helpers::Map<u8>| {
            let mut costs = helpers::Map::new(n, n);

            let (start, end) = ((0i32, 0i32), (n as i32 - 1, n as i32 - 1));
            let mut queue = VecDeque::from([(start, 0)]);
            while let Some((pos, len)) = queue.pop_front() {
                if pos == end {
                    return len;
                }

                for (_, neigh) in map.neighs4i(pos) {
                    if map[neigh] != b'#' && costs[neigh] == 0 {
                        costs[neigh] = len + 1;
                        queue.push_back((neigh, len + 1));
                    }
                }
            }

            -1
        };

        let res1 = solve(&map);
        let res2 = positions
            .find(|pos| {
                map[*pos] = b'#';
                solve(&map) == -1
            })
            .unwrap();

        (res1.to_string(), format!("{},{}", res2.1, res2.0))
    }
}
