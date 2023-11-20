use lib::aoc;
use lib::challenge::Challenge;

pub struct Day25;

impl Challenge for Day25 {
    aoc!(year = 2022, day = 25);

    fn solve(input: String) -> (String, String) {
        let res = input.lines().map(String::from).reduce(add_snafu).unwrap();
        (res.to_string(), "".into())
    }
}

fn decimal(c: char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn snafu(i: i32) -> (char, i32) {
    match i {
        -5 => ('0', -1),
        -4 => ('1', -1),
        -3 => ('2', -1),
        -2 => ('=', 0),
        -1 => ('-', 0),
        0 => ('0', 0),
        1 => ('1', 0),
        2 => ('2', 0),
        3 => ('=', 1),
        4 => ('-', 1),
        5 => ('0', 1),
        _ => unreachable!(),
    }
}

fn add_snafu(num1: String, num2: String) -> String {
    let mut res = Vec::new();

    let mut iter1 = num1.chars().rev();
    let mut iter2 = num2.chars().rev();

    let mut remainder = 0;
    let mut ch;

    loop {
        let next1 = iter1.next();
        let next2 = iter2.next();

        if next1.is_none() && next2.is_none() {
            break;
        }

        let i1 = next1.map(decimal).unwrap_or(0);
        let i2 = next2.map(decimal).unwrap_or(0);
        (ch, remainder) = snafu(i1 + i2 + remainder);
        res.push(ch);
    }

    if remainder > 0 {
        res.push(snafu(remainder).0);
    }

    res.into_iter().rev().collect::<String>()
}
