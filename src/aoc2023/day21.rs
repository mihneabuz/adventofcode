use std::collections::VecDeque;

use itertools::Itertools;

use lib::{aoc, challenge::Challenge, helpers::Map};

pub struct Day21;

impl Challenge for Day21 {
    aoc!(year = 2023, day = 21);

    fn solve(input: String) -> (String, String) {
        let map = Map::from_slices(&input.lines().map(|line| line.as_bytes()).collect_vec());
        let mut dists: Map<u8> = Map::new(map.height(), map.width());

        let start = map.find(|cell| *cell == b'S').unwrap();

        let mut fill = VecDeque::new();
        fill.push_back(start);

        while let Some((i, j)) = fill.pop_front() {
            let dist = *dists.get(i, j);

            for (ni, nj) in map.neighs4(i, j) {
                let neigh = map.get(ni, nj);
                if matches!(*neigh, b'.' | b'S') && *dists.get(ni, nj) == 0 {
                    dists.set(ni, nj, dist + 1);
                    fill.push_back((ni, nj));
                }
            }
        }

        let fst = dists
            .cells()
            .filter(|&(_, &cell)| (1..=64).contains(&cell) && cell % 2 == 0)
            .count();

        let steps = 26501365;
        assert_eq!(map.height(), map.width());
        assert_eq!(steps % map.height(), map.height() / 2);

        let n = map.height();
        let mid = n / 2;

        let mut evens = [0; 2];
        let mut odds = [0; 2];

        for ((i, j), cell) in dists.cells().filter(|(_, cell)| **cell != 0) {
            let target = if *cell % 2 == 0 {
                &mut evens
            } else {
                &mut odds
            };

            if (i < mid && (j < mid - i || j > mid + i))
                || (i > mid && (j < i - mid || j > 3 * mid - i))
            {
                target[0] += 1;
            } else {
                target[1] += 1;
            }
        }

        let rings = steps / n;

        let even_centers = rings * rings / 4;
        let odd_centers = rings / 2 * (rings / 2 + 1);
        let other_bits = rings * (rings + 1);

        let snd = odds[1]
            + 4 * even_centers * evens[1]
            + 4 * odd_centers * odds[1]
            + other_bits * (evens[0] + odds[0]);

        (fst.to_string(), snd.to_string())
    }
}
