use lib::challenge::{ChallengeObject, ThreadedChallenge};

automod::dir!(pub "src/aoc2023");

pub fn challanges() -> Vec<ChallengeObject> {
    vec![
        day1::Test::into_obj(),
        day1::Test::into_obj(),
        day1::Test::into_obj(),
        day1::ThreadedTest::into_obj(),
        day1::ThreadedTest::into_obj(),
        day1::Test::into_obj(),
        day1::ThreadedTest::into_obj(),
        day1::Test::into_obj(),
        day1::Test::into_obj(),
        day1::ThreadedTest::into_obj(),
        day1::ThreadedTest::into_obj(),
        day1::Test::into_obj(),
        day1::ThreadedTest::into_obj(),
    ]
}
