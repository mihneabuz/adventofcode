use lib::aoc;
use lib::challenge::Challenge;

use std::cmp;
use std::collections::HashSet;

pub struct Day9;

impl Challenge for Day9 {
    aoc!(year = 2022, day = 9);

    fn solve(input: String) -> (String, String) {
        let mut short_rope = vec![(0, 0); 2];
        let mut long_rope = vec![(0, 0); 10];

        let mut short_set = HashSet::new();
        let mut long_set = HashSet::new();

        for (dir, count) in input.lines().map(|s| s.split_once(" ").unwrap()) {
            for _ in 0..(count.parse().unwrap()) {
                let dir = dir.chars().next().unwrap();

                move_knots(dir, &mut short_rope);
                move_knots(dir, &mut long_rope);

                short_set.insert(short_rope.last().unwrap().clone());
                long_set.insert(long_rope.last().unwrap().clone());
            }
        }

        (short_set.len().to_string(), long_set.len().to_string())
    }
}

type Rope = Vec<(i32, i32)>;

fn move_knots(dir: char, rope: &mut Rope) {
    let (dx, dy) = match dir {
        'U' => (0, 1),
        'D' => (0, -1),
        'R' => (1, 0),
        'L' => (-1, 0),
        _ => unreachable!(),
    };

    rope[0].0 += dx;
    rope[0].1 += dy;

    for i in 1..rope.len() {
        if cmp::max(
            (rope[i - 1].0 - rope[i].0).abs(),
            (rope[i - 1].1 - rope[i].1).abs(),
        ) > 1
        {
            rope[i].0 += norm_diff(rope[i - 1].0, rope[i].0);
            rope[i].1 += norm_diff(rope[i - 1].1, rope[i].1);
        }
    }
}

fn norm_diff(x: i32, y: i32) -> i32 {
    match x - y {
        d if d < 0 => -1,
        d if d > 0 => 1,
        _ => 0,
    }
}
