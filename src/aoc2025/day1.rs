use lib::aoc;
use lib::challenge::Challenge;

pub struct Day1;

impl Challenge for Day1 {
    aoc!(year = 2025, day = 1);

    fn solve(input: String) -> (String, String) {
        let rules = input.lines().map(|line| {
            let dir = match line.as_bytes()[0] {
                b'L' => -1,
                b'R' => 1,
                _ => unreachable!(),
            };

            let count = line.split_at(1).1.parse::<i32>().unwrap();

            (dir, count)
        });

        let mut dial = 50;
        let (mut fst, mut snd) = (0, 0);

        for (dir, count) in rules {
            let next = dial + dir * count;

            if dir < 0 {
                snd += (if dial == 0 { 0 } else { 100 - dial } + count) / 100;
            } else {
                snd += (dial + count) / 100;
            }

            dial = if next < 0 {
                100 - (next.abs() % 100)
            } else {
                next
            } % 100;

            fst += (dial == 0) as i32;
        }

        (fst.to_string(), snd.to_string())
    }
}
