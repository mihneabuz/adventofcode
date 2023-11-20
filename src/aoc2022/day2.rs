use std::fs;
use std::io::{self, BufRead};

fn score(p1: &str, p2: &str) -> i32 {
    let piece = match p2 {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => unreachable!(),
    };

    let result = match (p1 == p2, p1, p2) {
        (true, _, _) => 3,
        (false, "A", "B") => 6,
        (false, "B", "C") => 6,
        (false, "C", "A") => 6,
        (false, _, _) => 0,
    };

    return piece + result;
}

fn choice1(p2: &str) -> &str {
    match p2 {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => unreachable!(),
    }
}

fn choice2<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    match (p1, p2) {
        (_, "Y") => p1,

        ("A", "X") => "C",
        ("A", "Z") => "B",

        ("B", "X") => "A",
        ("B", "Z") => "C",

        ("C", "X") => "B",
        ("C", "Z") => "A",

        _ => unreachable!(),
    }
}

fn main() {
    let reader = io::BufReader::new(fs::File::open("./input").unwrap());
    let (res1, res2) = reader
        .lines()
        .map(|l| l.unwrap())
        .fold((0, 0), |acc, line| {
            let (p1, p2) = line.split_once(" ").unwrap();
            (
                acc.0 + score(p1, choice1(p2)),
                acc.1 + score(p1, choice2(p1, p2)),
            )
        });

    println!("1: {}", res1);
    println!("2: {}", res2);
}
