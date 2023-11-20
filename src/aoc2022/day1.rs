use lib::aoc;
use lib::challenge::Challenge;

pub struct Day1;

impl Challenge for Day1 {
    aoc!(year = 2022, day = 1);

    fn solve(input: String) -> (String, String) {
        let mut elves = Vec::new();
        let mut acc = 0u64;

        input.lines().for_each(|line| {
            if line.len() > 1 {
                acc += line[..line.len() - 1].parse::<u64>().unwrap();
            } else {
                elves.push(acc);
                acc = 0;
            }
        });

        let first = elves.iter().max().unwrap();

        let mut top3 = vec![elves[0], elves[1], elves[2]];
        top3.sort();

        elves[3..].into_iter().for_each(|&elf| {
            if elf > top3[0] {
                top3[0] = elf;
                top3.sort();
            }
        });

        let second = top3.into_iter().sum::<u64>();

        (first.to_string(), second.to_string())
    }
}
