use std::sync::Arc;

use lib::challenge::{Challenge, ThreadedChallenge};
use lib::executor::WorkerGroup;
use lib::{aoc, workers};

pub struct Test;

impl Challenge for Test {
    aoc!(year = 2023, day = 1);

    fn solve(_: String) -> (String, String) {
        std::thread::sleep(std::time::Duration::from_secs(1));
        ("hello".into(), "world".into())
    }
}

pub struct ThreadedTest;

impl ThreadedChallenge for ThreadedTest {
    aoc!(year = 2023, day = 2);

    workers!(4);
    fn solve(_: String, workers: &mut WorkerGroup) -> (String, String) {
        let nums = (0..10000).collect::<Arc<_>>();

        let mut handles = Vec::new();

        let chunk_size = nums.len() / workers.available();
        for (i, worker) in workers.iter_mut().enumerate() {
            let nums = Arc::clone(&nums);

            let handle = worker.run(move || {
                let slice = &nums[chunk_size * i..chunk_size * (i + 1)];
                slice.iter().sum::<usize>()
            });

            handles.push(handle);
        }

        let total = handles.into_iter().map(|h| h.join()).sum::<usize>();

        ("nums".into(), total.to_string())
    }
}
