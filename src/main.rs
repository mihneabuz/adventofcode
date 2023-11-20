mod aoc2022;
mod aoc2023;

use std::time;

use clap::Parser;
use console::style;
use lib::{challenge::ChallengeObject, executor::AocExecutor, inputs::AocInputs};

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

    challenges.sort_by_key(|c| c.year * 10 + c.day);

    let count = challenges.len();

    AocInputs::new(args.inputs_cache.unwrap_or("cache".into()), args.download)?
        .get_inputs(&mut challenges)?;

    println!("Running {} challenges...", count);

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

    results.sort_by_key(|r| r.year * 10 + r.day);
    for result in results {
        let title = format!("{} day {:#2}", result.year, result.day);
        println!(
            " <=============>  {}  <=============>",
            style(title).bold().blue()
        );

        let (fst, snd) = result.solution;
        if !fst.contains('\n') && !snd.contains('\n') {
            println!("   {:#29} {}", style(fst).green(), style(snd).red());
        } else {
            println!("   {}", style(fst.replace('\n', "\n   ")).green());
            println!("   {}", style(snd.replace('\n', "\n   ")).red());
            println!();
        }
    }

    let footer = format!("Executed {} challanges in {:.2?}", count, time);
    println!("\n   {: ^42}", style(footer).bold().green());

    Ok(())
}
