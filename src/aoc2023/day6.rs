use itertools::Itertools;
use lib::{aoc, challenge::Challenge};

pub struct Day6;

impl Challenge for Day6 {
    aoc!(year = 2023, day = 6);

    fn solve(input: String) -> (String, String) {
        let (time_line, dist_line) = input.lines().collect_tuple().unwrap();

        let times = time_line
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();

        let dists = dist_line
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();

        let fst = times
            .iter()
            .copied()
            .zip(dists.iter().copied())
            .map(|(time, dist)| (1..time).filter(|x| x * (time - x) > dist).count())
            .product::<usize>();

        let time = times
            .into_iter()
            .map(|t| t.to_string())
            .reduce(|acc, s| acc + &s)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let dist = dists
            .into_iter()
            .map(|t| t.to_string())
            .reduce(|acc, s| acc + &s)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let snd = (1..time).filter(|x| x * (time - x) > dist).count();

        (fst.to_string(), snd.to_string())
    }
}
