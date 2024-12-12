use lib::{helpers, prelude::*};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

pub struct Day8;

impl Challenge for Day8 {
    aoc!(year = 2024, day = 8);

    fn solve(input: String) -> (String, String) {
        let map = helpers::Map::from_text(&input);
        let (n, m) = map.dims();

        let antenas =
            map.cells()
                .filter(|&(_, b)| *b != b'.')
                .fold(HashMap::new(), |mut acc, (pos, b)| {
                    let pos = (pos.0 as i32, pos.1 as i32);

                    acc.entry(b)
                        .and_modify(|set: &mut HashSet<(i32, i32)>| {
                            set.insert(pos);
                        })
                        .or_insert(HashSet::from([pos]));

                    acc
                });

        let mut antinodes: helpers::Map<u8> = helpers::Map::new(n, m);
        for (_, antenas) in antenas.iter() {
            for (mut pos1, mut pos2) in antenas.iter().tuple_combinations::<(_, _)>() {
                if pos1.0 > pos2.0 {
                    std::mem::swap(&mut pos1, &mut pos2);
                }

                let diff = (pos2.0 - pos1.0, pos2.1 - pos1.1);

                let s1 = (pos1.0 - diff.0, pos1.1 - diff.1);
                if map.valid(s1) {
                    antinodes[s1] = b'#';
                }

                let s2 = (pos2.0 + diff.0, pos2.1 + diff.1);
                if map.valid(s2) {
                    antinodes[s2] = b'#';
                }
            }
        }

        let res1 = antinodes.cells().filter(|(_, b)| **b == b'#').count();

        for (_, antenas) in antenas {
            for (mut pos1, mut pos2) in antenas.iter().tuple_combinations::<(_, _)>() {
                if pos1.0 > pos2.0 {
                    std::mem::swap(&mut pos1, &mut pos2);
                }

                let diff = (pos2.0 - pos1.0, pos2.1 - pos1.1);

                let mut s1 = *pos1;
                while map.valid(s1) {
                    antinodes[s1] = b'#';
                    s1 = (s1.0 - diff.0, s1.1 - diff.1);
                }

                let mut s2 = *pos2;
                while map.valid(s2) {
                    antinodes[s2] = b'#';
                    s2 = (s2.0 + diff.0, s2.1 + diff.1);
                }
            }
        }

        let res2 = antinodes.cells().filter(|(_, b)| **b == b'#').count();

        (res1.to_string(), res2.to_string())
    }
}
