use lib::prelude::*;

use itertools::Itertools;

pub struct Day19;

impl Challenge for Day19 {
    aoc!(year = 2024, day = 19);

    fn solve(input: String) -> (String, String) {
        let (towels, designs) = input.split_once("\n\n").unwrap();
        let towels = towels.split(", ").collect_vec();

        let mut cache = vec![0u64; 128];
        let mut combinations = |design: &str| {
            cache.fill(0);

            for towel in towels.iter() {
                if design.ends_with(towel) {
                    let i = design.len() - towel.len();
                    cache[i] = 1;
                }
            }

            for i in (0..design.len()).rev() {
                for towel in towels.iter() {
                    if design[i..].starts_with(towel) {
                        cache[i] += cache.get(i + towel.len()).copied().unwrap_or(0);
                    }
                }
            }

            cache[0]
        };

        let (res1, res2) = designs.lines().fold((0, 0), |(acc1, acc2), design| {
            let count = combinations(design);
            (acc1 + (count > 0) as u64, acc2 + count)
        });

        (res1.to_string(), res2.to_string())
    }
}
