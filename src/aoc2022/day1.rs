use std::fs;
use std::io::{self, BufRead};

fn line_reader<F>(mut f: F) -> io::Result<()>
where
    F: FnMut(&String) -> (),
{
    let input = fs::File::open("./input")?;
    let mut reader = io::BufReader::new(input);

    let mut line = String::new();
    while 0 < reader.read_line(&mut line)? {
        f(&line);
        line.clear();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut elves = Vec::new();
    let mut acc = 0u64;

    line_reader(|line| match line.len() > 1 {
        true => {
            acc += line[..line.len() - 1].parse::<u64>().unwrap();
        }
        false => {
            elves.push(acc);
            acc = 0;
        }
    })?;

    println!("1: {}", elves.iter().max().unwrap());

    let mut top3 = vec![elves[0], elves[1], elves[2]];
    top3.sort();

    elves[3..].into_iter().for_each(|&elf| {
        if elf > top3[0] {
            top3[0] = elf;
            top3.sort();
        }
    });

    println!("2: {}", top3.into_iter().sum::<u64>());

    Ok(())
}
