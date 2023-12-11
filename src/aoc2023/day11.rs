use itertools::Itertools;
use lib::{aoc, challenge::Challenge};

pub struct Day11;

impl Challenge for Day11 {
    aoc!(year = 2023, day = 11);

    fn solve(input: String) -> (String, String) {
        let galaxies = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(j, _)| (i, j))
            })
            .collect_vec();

        let mut rows = [1u64; 256];
        let mut cols = [1u64; 256];
        for &(i, j) in galaxies.iter() {
            rows[i] = 1 << (u64::BITS / 2);
            cols[j] = 1 << (u64::BITS / 2);
        }

        let dists = galaxies
            .into_iter()
            .tuple_combinations()
            .map(|(g1, g2)| {
                rows[g1.0.min(g2.0)..g1.0.max(g2.0)].iter().sum::<u64>()
                    + cols[g1.1.min(g2.1)..g1.1.max(g2.1)].iter().sum::<u64>()
            })
            .sum::<u64>();

        let expansion = |exp| {
            (dists >> (u64::BITS / 2)) as usize
                + (dists & ((1 << (u64::BITS / 2)) - 1)) as usize * exp
        };

        let fst = expansion(2);
        let snd = expansion(1_000_000);

        (fst.to_string(), snd.to_string())
    }
}
