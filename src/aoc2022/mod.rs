use lib::challenge::{ChallengeObject, ThreadedChallenge};

automod::dir!(pub "src/aoc2022");

pub fn challenges() -> Vec<ChallengeObject> {
    vec![
        day1::Day1::into_obj(),
        day2::Day2::into_obj(),
        day3::Day3::into_obj(),
        day4::Day4::into_obj(),
        day5::Day5::into_obj(),
        day6::Day6::into_obj(),
        day7::Day7::into_obj(),
        day8::Day8::into_obj(),
        day9::Day9::into_obj(),
        day10::Day10::into_obj(),
    ]
}
