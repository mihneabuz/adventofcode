use crate::executor::WorkerGroup;

type Solver = Box<dyn Fn(String, &mut WorkerGroup) -> (String, String) + Send + 'static>;

pub trait Challenge {
    fn year() -> usize;
    fn day() -> usize;
    fn solve(input: String) -> (String, String);
}

pub trait ThreadedChallenge {
    fn year() -> usize;
    fn day() -> usize;
    fn worker_hint() -> Option<usize>;
    fn solve(input: String, workers: &mut WorkerGroup) -> (String, String);

    fn into_obj() -> ChallengeObject {
        ChallengeObject {
            year: Self::year(),
            day: Self::day(),
            worker_hint: Self::worker_hint(),
            solve: Box::new(move |input, workers| Self::solve(input, workers)),
            input: String::new(),
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
}

pub struct ChallengeResult {
    pub year: usize,
    pub day: usize,
    pub solution: (String, String),
}

impl ChallengeObject {
    pub fn solve(self, workers: &mut WorkerGroup) -> ChallengeResult {
        let solution = (self.solve)(self.input, workers);

        ChallengeResult {
            year: self.year,
            day: self.day,
            solution,
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
