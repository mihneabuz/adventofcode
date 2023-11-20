use lib::{aoc, challenge::Challenge};

pub struct Day1;

impl Challenge for Day1 {
    aoc!(year = 2023, day = 1);

    fn solve(input: String) -> (String, String) {
        (input, "world".into())
    }
}
