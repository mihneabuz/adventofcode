use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use lib::{
    aoc,
    challenge::Challenge,
    helpers::{Segment, SegmentSequence},
};

pub struct Day18;

impl Challenge for Day18 {
    aoc!(year = 2023, day = 18);

    fn solve(input: String) -> (String, String) {
        let commands = input.lines().map(parse);

        let fst = solve(commands.clone().map(|(dir, count, _)| (dir, count)));
        let snd = solve(commands.map(|(_, _, meta)| {
            let (init, last) = meta.split_at(meta.len() - 1);

            let dir = match last {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => unreachable!(),
            };

            let count = isize::from_str_radix(init, 16).unwrap();

            (dir, count)
        }));

        (fst.to_string(), snd.to_string())
    }
}

fn solve(iter: impl Iterator<Item = (char, isize)>) -> isize {
    let sequence_width = |sequence: &SegmentSequence<bool>| {
        sequence
            .segments()
            .filter_map(|segment| segment.value.then_some(segment.hi - segment.lo + 1))
            .sum::<isize>()
    };

    iter.scan((0, 0), |(i, j), (dir, count)| {
        Some(match dir {
            'U' => {
                *i -= count;
                Some(((*i, *i + count), *j))
            }
            'D' => {
                *i += count;
                Some(((*i - count, *i), *j))
            }
            'L' => {
                *j -= count;
                None
            }
            'R' => {
                *j += count;
                None
            }
            _ => unreachable!(),
        })
    })
    .flatten()
    .sorted_by_key(|(_, offset)| *offset)
    .chunk_by(|(_, offset)| *offset)
    .into_iter()
    .fold(
        (SegmentSequence::<bool>::new(), None, 0),
        |(mut sequence, last_offset, total), (offset, walls)| {
            let width = sequence_width(&sequence);

            let skipped = last_offset
                .map(|last| width * (offset - 1 - last))
                .unwrap_or(0);

            let mut current = width;
            for ((start, end), _) in walls {
                let mut removed = 0;
                sequence.insert(Segment::new(start, end, true).unwrap(), |_, old| {
                    removed = old.hi - old.lo;
                    false
                });

                sequence = sequence.simplify();

                if removed > 0 {
                    current += removed;
                }
            }

            sequence.retain(|segment| segment.value);

            (sequence, Some(offset), total + skipped + current)
        },
    )
    .2 + 1
}

fn parse(line: &str) -> (char, isize, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([UDRL]) (\d+) \(#(.+)\)").unwrap();
    }

    let captures = RE.captures(line).unwrap();

    let direction = captures.get(1).unwrap().as_str().chars().next().unwrap();
    let count = captures.get(2).unwrap().as_str().parse().unwrap();
    let color = captures.get(3).unwrap().as_str();

    (direction, count, color)
}
