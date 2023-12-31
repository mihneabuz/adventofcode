use itertools::Itertools;
use lib::{
    aoc,
    challenge::Challenge,
    helpers::{unchecked_parse, Segment, SegmentSequence},
};

pub struct Day5;

impl Challenge for Day5 {
    aoc!(year = 2023, day = 5);

    fn solve(input: String) -> (String, String) {
        let mut input = input.split("\n\n");

        let seeds = input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(unchecked_parse::<isize>)
            .collect_vec();

        let map = input
            .map(|m| {
                SegmentSequence::from_iterator(m.lines().skip(1).map(|line| {
                    let (dst, src, len) = line
                        .split_whitespace()
                        .map(unchecked_parse::<isize>)
                        .collect_tuple::<(_, _, _)>()
                        .unwrap();

                    let delta = dst - src;

                    Segment::new(src, src + len, delta).unwrap()
                }))
                .unwrap()
            })
            .reduce(merge)
            .unwrap();

        let fst = seeds
            .iter()
            .copied()
            .map(|seed| map.search(seed).map(|seg| seg.value + seed).unwrap_or(seed))
            .min()
            .unwrap();

        let snd = seeds
            .chunks(2)
            .map(|s| {
                map.range(s[0], s[0] + s[1])
                    .map(|seg| seg.lo + seg.value.unwrap_or(0))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap();

        (fst.to_string(), snd.to_string())
    }
}

fn merge(front: SegmentSequence<isize>, back: SegmentSequence<isize>) -> SegmentSequence<isize> {
    let mut acc = Vec::new();

    for seg in front.range(0, isize::MAX) {
        let value = seg.value.unwrap_or(0);
        let map_lo = seg.lo + value;
        let map_hi = seg.hi + value;

        for overlap in back.range(map_lo, map_hi) {
            let unmaped_lo = overlap.lo - value;
            let unmaped_hi = overlap.hi - value;
            acc.push(
                Segment::new(unmaped_lo, unmaped_hi, overlap.value.unwrap_or(0) + value).unwrap(),
            );
        }
    }

    SegmentSequence::from_iterator(acc.into_iter()).unwrap()
}
