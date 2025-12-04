mod aoc2022;
mod aoc2023;
mod aoc2024;
mod aoc2025;

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

    let mut challenges: Vec<ChallengeObject> = vec![
        aoc2022::challenges(),
        aoc2023::challenges(),
        aoc2024::challenges(),
        aoc2025::challenges(),
    ]
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
    show_results(results);

    let footer = format!("Executed {} challanges in {:.2?}", count, time);
    println!("\n {}", style(footer).bold().green());

    Ok(())
}

fn show_results(results: Vec<ChallengeResult>) {
    use cli_table::{Cell, Color, Style, Table, format::Justify, print_stdout};

    let longest = results
        .iter()
        .map(|r| r.duration)
        .max()
        .unwrap()
        .as_nanos()
        .ilog2();

    let mut table = Vec::new();
    for result in results.into_iter() {
        let duration = result.duration.as_nanos().ilog2();
        let rel_duration =
            String::from("█").repeat((duration * 10 / longest.max(1)).max(1) as usize);

        table.push(vec![
            result
                .year
                .cell()
                .foreground_color(Some(Color::Blue))
                .justify(Justify::Right),
            result
                .day
                .cell()
                .foreground_color(Some(Color::Green))
                .justify(Justify::Right),
            result.solution.0.cell(),
            result.solution.1.cell(),
            format!("{:.2?}", result.duration)
                .cell()
                .foreground_color(Some(Color::Yellow))
                .justify(Justify::Right),
            rel_duration
                .cell()
                .foreground_color(Some(Color::Red))
                .justify(Justify::Right),
        ]);
    }

    let header = ["Year", "Day", "Part 1", "Part 2", "Time", "Relative"]
        .iter()
        .map(|name| {
            style(name)
                .red()
                .bold()
                .to_string()
                .cell()
                .bold(true)
                .justify(Justify::Center)
        })
        .collect::<Vec<_>>();

    print_stdout(table.table().title(header)).unwrap();
}
