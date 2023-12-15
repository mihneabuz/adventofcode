use std::array;

use lib::{aoc, challenge::Challenge};

pub struct Day15;

impl Challenge for Day15 {
    aoc!(year = 2023, day = 15);

    fn solve(input: String) -> (String, String) {
        let sequences = input.split(',');

        let hash = |seq: &str| {
            seq.as_bytes()
                .iter()
                .fold(0, |acc, c| ((acc + *c as usize) * 17) % 256)
        };

        let fst = sequences.clone().map(hash).sum::<usize>();

        let boxes: [Vec<(&str, usize)>; 256] = array::from_fn(|_| Vec::new());
        let snd = sequences
            .fold(boxes, |mut acc, seq| {
                match parse(seq) {
                    Op::Insert { label, focal } => {
                        let h = hash(label);
                        match acc[h].iter().position(|l| l.0 == label) {
                            Some(idx) => acc[h][idx].1 = focal,
                            None => acc[h].push((label, focal)),
                        }
                    }

                    Op::Remove { label } => {
                        let h = hash(label);
                        if let Some(idx) = acc[h].iter().position(|l| l.0 == label) {
                            acc[h].remove(idx);
                        }
                    }
                }

                acc
            })
            .into_iter()
            .enumerate()
            .fold(0, |acc, (idx, lenses)| {
                acc + (idx + 1)
                    * lenses
                        .into_iter()
                        .enumerate()
                        .fold(0, |acc, (idx, (_, focal))| acc + (idx + 1) * focal)
            });

        (fst.to_string(), snd.to_string())
    }
}

enum Op<'a> {
    Insert { label: &'a str, focal: usize },
    Remove { label: &'a str },
}

fn parse(seq: &str) -> Op {
    if *seq.as_bytes().last().unwrap() == b'-' {
        return Op::Remove {
            label: seq.trim_end_matches('-'),
        };
    }

    let (label, focal) = seq.split_once('=').unwrap();
    Op::Insert {
        label,
        focal: focal.parse().unwrap(),
    }
}
