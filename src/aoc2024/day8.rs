use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use lib::{helpers, prelude::*};
use ndarray::Array2;

pub struct Day8;

impl Challenge for Day8 {
    aoc!(year = 2024, day = 8);

    fn solve(input: String) -> (String, String) {
        let map = helpers::array2::from_str(&input);
        let (n, m) = map.dim();
        let valid = |(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < m as i32;

        let antenas = map.indexed_iter().filter(|&(_, b)| *b != b'.').fold(
            HashMap::new(),
            |mut acc, (pos, b)| {
                let pos = (pos.0 as i32, pos.1 as i32);

                acc.entry(b)
                    .and_modify(|set: &mut HashSet<(i32, i32)>| {
                        set.insert(pos);
                    })
                    .or_insert(HashSet::from([pos]));

                acc
            },
        );

        let mut antinodes: Array2<u8> = Array2::zeros((n, m));
        for (_, antenas) in antenas.iter() {
            for (mut pos1, mut pos2) in antenas.iter().tuple_combinations::<(_, _)>() {
                if pos1.0 > pos2.0 {
                    std::mem::swap(&mut pos1, &mut pos2);
                }

                let diff = (pos2.0 - pos1.0, pos2.1 - pos1.1);

                let s1 = (pos1.0 - diff.0, pos1.1 - diff.1);
                if valid(s1) {
                    antinodes[(s1.0 as usize, s1.1 as usize)] = b'#';
                }

                let s2 = (pos2.0 + diff.0, pos2.1 + diff.1);
                if valid(s2) {
                    antinodes[(s2.0 as usize, s2.1 as usize)] = b'#';
                }
            }
        }

        let res1 = antinodes.iter().filter(|b| **b == b'#').count();

        for (_, antenas) in antenas {
            for (mut pos1, mut pos2) in antenas.iter().tuple_combinations::<(_, _)>() {
                if pos1.0 > pos2.0 {
                    std::mem::swap(&mut pos1, &mut pos2);
                }

                let diff = (pos2.0 - pos1.0, pos2.1 - pos1.1);

                let mut s1 = *pos1;
                while valid(s1) {
                    antinodes[(s1.0 as usize, s1.1 as usize)] = b'#';
                    s1 = (s1.0 - diff.0, s1.1 - diff.1);
                }

                let mut s2 = *pos2;
                while valid(s2) {
                    antinodes[(s2.0 as usize, s2.1 as usize)] = b'#';
                    s2 = (s2.0 + diff.0, s2.1 + diff.1);
                }
            }
        }

        let res2 = antinodes.iter().filter(|b| **b == b'#').count();

        (res1.to_string(), res2.to_string())
    }
}
