use lib::prelude::*;

pub struct Day3;

impl Challenge for Day3 {
    aoc!(year = 2025, day = 3);

    fn solve(input: String) -> (String, String) {
        let banks = input.lines().map(|line| line.as_bytes());

        let max = |bytes: &[u8]| {
            bytes
                .iter()
                .copied()
                .enumerate()
                .fold(
                    (0, 0),
                    |best, curr| if curr.1 > best.1 { curr } else { best },
                )
                .0
        };

        let solve = |bank: &[u8], count| {
            let mut acc = 0;
            let mut start = 0;
            for digit in (0..count).rev() {
                let max_pos = start + max(&bank[start..bank.len() - digit]);
                acc = acc * 10 + (bank[max_pos] - b'0') as u64;
                start = max_pos + 1;
            }
            acc
        };

        let (mut fst, mut snd) = (0, 0);
        for bank in banks {
            fst += solve(bank, 2);
            snd += solve(bank, 12);
        }

        (fst.to_string(), snd.to_string())
    }
}
