mod aoc2022;
mod aoc2023;

use std::time;

use clap::Parser;
use console::style;
use lib::{
    challenge::{ChallengeObject, ChallengeResult},
    executor::AocExecutor,
    inputs::AocInputs,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    worker_threads: Option<usize>,

    #[arg(short, long)]
    year: Option<usize>,

    #[arg(short, long)]
    day: Option<usize>,

    #[arg(long)]
    download: Option<String>,

    #[arg(long)]
    inputs_cache: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut challenges: Vec<ChallengeObject> = vec![aoc2022::challenges(), aoc2023::challenges()]
        .into_iter()
        .flatten()
        .collect();

    if let Some(year) = args.year {
        challenges.retain(|c| c.year == year);
    }

    if let Some(day) = args.day {
        challenges.retain(|c| c.day == day);
    }

    if challenges.is_empty() {
        println!("No challenges to run.");
        return Ok(());
    }

    challenges.sort_by_key(|c| c.year * 10 + c.day);

    let count = challenges.len();

    AocInputs::new(args.inputs_cache.unwrap_or("cache".into()), args.download)?
        .get_inputs(&mut challenges)?;

    let header = format!("Running {} challenges", count);
    println!("\n {}", style(header).bold().green());

    let mut executor = match args.worker_threads {
        Some(workers) => AocExecutor::with_workers(workers),
        None => AocExecutor::default(),
    };

    let start = time::Instant::now();
    let mut results = if count > 1 {
        executor.run_all(challenges)
    } else {
        vec![executor.run_one(challenges.pop().unwrap())]
    };
    let time = time::Instant::now() - start;

    println!();

    results.sort_by_key(|r| (r.year, r.day));
    show_results(&results);

    let footer = format!("Executed {} challanges in {:.2?}", count, time);
    println!("\n {}", style(footer).bold().green());

    Ok(())
}

fn show_results(results: &[ChallengeResult]) {
    use prettytable::{Cell, Row, Table};

    if let Some(result) = results.iter().find(|r| r.example.is_some()) {
        let solution = result.example.as_ref().unwrap();
        println!("Example: {} {}", solution.0, solution.1);
    }

    let mut table = Table::new();

    let header = ["Year", "Day", "Part 1", "Part 2", "Time"]
        .iter()
        .map(|name| style(name).red().bold().to_string())
        .collect::<Vec<_>>();

    table.add_row(Row::new(
        header.iter().map(|name| Cell::new(name)).collect(),
    ));

    let longest = results
        .iter()
        .map(|r| r.duration)
        .max()
        .unwrap()
        .as_micros()
        .ilog2();

    for result in results {
        let duration = result.duration.as_micros().ilog2();
        let rel_duration =
            String::from("█").repeat((duration * 10 / longest.max(0)).max(1) as usize);

        table.add_row(prettytable::row![
            style(format!("{:?}", result.year)).blue(),
            style(format!("{:?}", result.day)).green(),
            result.solution.0,
            result.solution.1,
            style(format!("{:.2?}", result.duration)).yellow(),
            style(rel_duration.to_string()).red(),
        ]);
    }

    table.printstd();
}
