use lib::helpers::Trie;
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

        let mut trie: Trie<u32, 256> = Trie::new();
        for (path, value) in digits.iter().copied() {
            trie.add(path.chars().map(|c| c as usize), value);
        }

        let mut rev_trie: Trie<u32, 256> = Trie::new();
        for (path, value) in digits.iter().copied() {
            rev_trie.add(path.chars().rev().map(|c| c as usize), value);
        }

        let snd: usize = input
            .lines()
            .map(|line| {
                let x = (0..line.len())
                    .map(|i| &line[i..])
                    .find_map(|prefix| {
                        trie.first_match(prefix.chars().map(|c| c as usize))
                            .copied()
                            .or_else(|| prefix.chars().nth(0).unwrap().to_digit(10))
                    })
                    .unwrap();

                let y = (1..line.len() + 1)
                    .rev()
                    .map(|i| &line[0..i])
                    .find_map(|suffix| {
                        rev_trie
                            .first_match(suffix.chars().rev().map(|c| c as usize))
                            .copied()
                            .or_else(|| suffix.chars().last().unwrap().to_digit(10))
                    })
                    .unwrap();

                (x * 10 + y) as usize
            })
            .sum();

        (fst.to_string(), snd.to_string())
    }
}
