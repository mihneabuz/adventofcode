use lib::prelude::*;
use regex::Regex;

pub struct Day3;

impl Challenge for Day3 {
    aoc!(year = 2024, day = 3);

    fn solve(input: String) -> (String, String) {
        let re1 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let res1 = re1
            .captures_iter(&input)
            .map(|capture| {
                let x = capture.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let y = capture.get(2).unwrap().as_str().parse::<u64>().unwrap();
                x * y
            })
            .sum::<u64>();

        let re2 = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
        let res2 = re2
            .captures_iter(&input)
            .scan(true, |enabled, capture| {
                Some(match capture.get(0).unwrap().as_str() {
                    "do()" => {
                        *enabled = true;
                        0
                    }

                    "don't()" => {
                        *enabled = false;
                        0
                    }

                    _ => {
                        if *enabled {
                            let x = capture.get(1).unwrap().as_str().parse::<u64>().unwrap();
                            let y = capture.get(2).unwrap().as_str().parse::<u64>().unwrap();
                            x * y
                        } else {
                            0
                        }
                    }
                })
            })
            .sum::<u64>();

        (res1.to_string(), res2.to_string())
    }
}
