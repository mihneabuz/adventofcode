use lib::aoc;
use lib::challenge::Challenge;

use itertools::Itertools;

pub struct Day1;

impl Challenge for Day1 {
    aoc!(year = 2022, day = 1);

    fn solve(input: String) -> (String, String) {
        let weights = input
            .lines()
            .map(|line| line.trim().parse::<usize>().unwrap_or(0));

        let elfs = weights
            .group_by(|&w| w > 0)
            .into_iter()
            .map(|(_, elf)| elf.sum::<usize>())
            .collect::<Vec<_>>();

        let fst = elfs.iter().copied().max().unwrap();

        let snd = elfs.into_iter().sorted().rev().take(3).sum::<usize>();

        (fst.to_string(), snd.to_string())
    }
}
