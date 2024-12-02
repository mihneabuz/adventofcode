use std::time::{self, Duration};

use crate::executor::WorkerGroup;

type Solver =
    Box<dyn Fn(String, &mut WorkerGroup) -> ((String, String), Duration) + Send + 'static>;

pub trait Challenge {
    fn year() -> usize;
    fn day() -> usize;
    fn solve(input: String) -> (String, String);

    fn example() -> Option<&'static str> {
        None
    }
}

pub trait ThreadedChallenge {
    fn year() -> usize;
    fn day() -> usize;
    fn worker_hint() -> Option<usize>;
    fn solve(input: String, workers: &mut WorkerGroup) -> (String, String);

    fn example() -> Option<&'static str> {
        None
    }

    fn into_obj() -> ChallengeObject {
        ChallengeObject {
            year: Self::year(),
            day: Self::day(),
            worker_hint: Self::worker_hint(),
            solve: Box::new(move |input, workers| {
                let start = time::Instant::now();
                let solution = Self::solve(input, workers);
                let time = time::Instant::now() - start;
                (solution, time)
            }),
            input: String::default(),
            example: Self::example(),
        }
    }
}

impl<T> ThreadedChallenge for T
where
    T: Challenge,
{
    fn year() -> usize {
        T::year()
    }

    fn day() -> usize {
        T::day()
    }

    fn worker_hint() -> Option<usize> {
        None
    }

    fn example() -> Option<&'static str> {
        T::example()
    }

    fn solve(input: String, _: &mut WorkerGroup) -> (String, String) {
        T::solve(input)
    }
}

pub struct ChallengeObject {
    pub year: usize,
    pub day: usize,
    pub worker_hint: Option<usize>,
    pub solve: Solver,
    pub input: String,
    pub example: Option<&'static str>,
}

pub struct ChallengeResult {
    pub year: usize,
    pub day: usize,
    pub solution: (String, String),
    pub duration: Duration,
}

impl ChallengeObject {
    pub fn solve(self, workers: &mut WorkerGroup) -> ChallengeResult {
        let input = match self.example {
            Some(example) => example.to_string(),
            None => self.input,
        };

        let (solution, duration) = (self.solve)(input, workers);

        ChallengeResult {
            year: self.year,
            day: self.day,
            solution,
            duration,
        }
    }
}

#[macro_export]
macro_rules! year {
    ($x:expr) => {
        fn year() -> usize {
            $x
        }
    };
}

#[macro_export]
macro_rules! day {
    ($x:expr) => {
        fn day() -> usize {
            $x
        }
    };
}

#[macro_export]
macro_rules! workers {
    ($x:expr) => {
        fn worker_hint() -> Option<usize> {
            Some($x)
        }
    };
}

#[macro_export]
macro_rules! example {
    ($x:expr) => {
        fn example() -> Option<&'static str> {
            Some($x)
        }
    };
}

#[macro_export]
macro_rules! aoc {
    (year = $y:expr, day = $d:expr) => {
        fn year() -> usize {
            $y
        }

        fn day() -> usize {
            $d
        }
    };
}
