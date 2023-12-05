use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;
use lib::{aoc, challenge::Challenge, example};

pub struct Day5;

impl Challenge for Day5 {
    aoc!(year = 2023, day = 5);

    example!(
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    );
    fn solve(input: String) -> (String, String) {
        let mut input = input.split("\n\n");

        let seeds = input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect_vec();

        let maps = input
            .map(|m| {
                m.lines()
                    .skip(1)
                    .map(|line| {
                        line.split_whitespace()
                            .map(parse::<isize>)
                            .collect_tuple::<(_, _, _)>()
                            .unwrap()
                    })
                    .sorted_by_key(|s| s.1)
                    .collect_vec()
            })
            .collect_vec();

        // if seeds.len() > 5 {
        //     return ("".into(), "".into());
        // }

        let fst = seeds
            .iter()
            .copied()
            .map(|seed| maps.iter().fold(seed, |seed, map| next_seed(seed, map)))
            .min()
            .unwrap();

        let snd = seeds
            .chunks_exact(2)
            .flat_map(|s| s[0]..s[0] + s[1])
            .map(|seed| maps.iter().fold(seed, |seed, map| next_seed(seed, map)))
            .min()
            .unwrap();

        // let last = maps[maps.len() - 1].clone();
        // let before_last = maps[maps.len() - 2].clone();
        // println!("{:?}", before_last);
        // println!("{:?}", last);
        // println!();
        // println!("{:?}", merge(before_last, last));

        (fst.to_string(), snd.to_string())
    }
}

fn find_seg(map: &[(isize, isize, isize)], point: isize) -> Result<usize, usize> {
    map.binary_search_by(|seg| {
        let (lo, hi) = (seg.1, seg.1 + seg.2);
        match (point >= lo, point < hi) {
            (true, true) => Ordering::Equal,
            (false, _) => Ordering::Greater,
            (true, _) => Ordering::Less,
        }
    })
}

fn next_seed(seed: isize, map: &[(isize, isize, isize)]) -> isize {
    find_seg(map, seed)
        .map(|pos| map[pos].0 + seed - map[pos].1)
        .unwrap_or(seed)
}

fn merge(
    new: Vec<(isize, isize, isize)>,
    old: Vec<(isize, isize, isize)>,
) -> Vec<(isize, isize, isize)> {
    let result = Vec::new();

    for seg in new {
        let first = find_seg(&old, seg.0);
        let last = find_seg(&old, seg.0 + seg.2);
        println!("> {:?}   {:?} - {:?}", seg, first, last);
    }

    result
}

fn parse<T>(s: &str) -> T
where
    T: FromStr,
{
    s.parse::<T>().ok().unwrap()
}
