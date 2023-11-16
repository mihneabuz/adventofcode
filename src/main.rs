mod aoc2023;

use clap::Parser;
use lib::{challenge::ChallengeObject, executor::AocExecutor};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    worker_threads: Option<usize>,

    #[arg(short, long)]
    year: Option<usize>,

    #[arg(short, long)]
    day: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let mut challenges: Vec<ChallengeObject> = aoc2023::challanges().into_iter().collect();

    if let Some(year) = args.year {
        challenges.retain(|c| c.year == year);
    }

    if let Some(day) = args.day {
        challenges.retain(|c| c.day == day);
    }

    challenges.sort_by_key(|c| c.year * 10 + c.day);

    println!("Running {} challenges...", challenges.len());

    let mut executor = match args.worker_threads {
        Some(workers) => AocExecutor::with_workers(workers),
        None => AocExecutor::default(),
    };

    let mut results = executor.run_all(challenges);

    results.sort_by_key(|r| r.year * 10 + r.day);

    for result in results {
        println!("[{} day {}] {:?}", result.year, result.day, result.solution);
    }
}
