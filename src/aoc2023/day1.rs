use lib::{aoc, challenge::Challenge};

pub struct Day1;

impl Challenge for Day1 {
    aoc!(year = 2023, day = 1);

    fn solve(input: String) -> (String, String) {
        let fst: usize = input
            .lines()
            .map(|line| {
                let x = line.chars().find_map(|c| c.to_digit(10)).unwrap();
                let y = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();

                (x * 10 + y) as usize
            })
            .sum();

        let digits = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let snd: usize = input
            .lines()
            .map(|line| {
                let x = prefixes(line)
                    .find_map(|prefix| {
                        for (text, value) in digits.iter().copied() {
                            if prefix.ends_with(text) {
                                return Some(value);
                            }
                        }

                        prefix.chars().last().and_then(|c| c.to_digit(10))
                    })
                    .unwrap();

                let y = suffixes(line)
                    .find_map(|suffix| {
                        for (text, value) in digits.iter().copied() {
                            if suffix.starts_with(text) {
                                return Some(value);
                            }
                        }

                        suffix.chars().next().and_then(|c| c.to_digit(10))
                    })
                    .unwrap();

                (x * 10 + y) as usize
            })
            .sum();

        (fst.to_string(), snd.to_string())
    }
}

fn prefixes(line: &str) -> impl Iterator<Item = &str> {
    (1..line.len() + 1).map(|i| &line[0..i])
}

fn suffixes(line: &str) -> impl Iterator<Item = &str> {
    (0..line.len()).rev().map(|i| &line[i..line.len()])
}
