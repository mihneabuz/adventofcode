use lib::prelude::*;
use smallvec::SmallVec;

pub struct Day25;

impl Challenge for Day25 {
    aoc!(year = 2024, day = 25);

    fn solve(input: String) -> (String, String) {
        let (mut keys, mut locks) = (Vec::new(), Vec::new());

        for input in input.split("\n\n") {
            let lines = input.lines().collect::<SmallVec<[_; 8]>>();

            let value = (0..5usize)
                .map(|i| {
                    lines
                        .iter()
                        .filter(|line| line.as_bytes()[i] == b'#')
                        .count()
                })
                .collect::<SmallVec<[_; 6]>>();

            if lines[0] == "#####" {
                keys.push(value);
            } else if lines[6] == "#####" {
                locks.push(value);
            }
        }

        let mut res1 = 0;
        for lock in locks.iter() {
            for key in keys.iter() {
                let fits = lock.iter().zip(key.iter()).all(|(l, k)| *l + *k <= 7);
                res1 += fits as u32;
            }
        }

        (res1.to_string(), "".to_string())
    }
}
