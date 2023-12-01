use lib::challenge::{ChallengeObject, ThreadedChallenge};

automod::dir!(pub "src/aoc2023");

pub fn challenges() -> Vec<ChallengeObject> {
    vec![day1::Day1::into_obj()]
}
