use std::str::FromStr;

use itertools::Itertools;
use lib::{aoc, challenge::Challenge};

pub struct Day2;

impl Challenge for Day2 {
    aoc!(year = 2023, day = 2);

    fn solve(input: String) -> (String, String) {
        let games = input
            .lines()
            .enumerate()
            .map(|(id, line)| {
                let mut game = Game::new(id + 1);

                let rounds = line.split_once(": ").unwrap().1;
                for cubes in rounds.split("; ").flat_map(|r| r.split(", ")) {
                    let (count, color) = cubes.split_once(' ').unwrap();
                    let count = usize::from_str(count).unwrap();
                    match color {
                        "red"   => game.red(count),
                        "green" => game.green(count),
                        "blue"  => game.blue(count),
                        _       => unreachable!(),
                    }
                }

                game
            })
            .collect_vec();

        let fst = games
            .iter()
            .filter(|game| game.in_bounds(12, 13, 14))
            .map(|game| game.id)
            .sum::<usize>();

        let snd = games.into_iter().map(|game| game.power()).sum::<usize>();

        (fst.to_string(), snd.to_string())
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    r: usize,
    g: usize,
    b: usize,
}

impl Game {
    pub fn new(id: usize) -> Self {
        Self { id, r: 0, g: 0, b: 0 }
    }

    pub fn red(&mut self, r: usize) {
        self.r = self.r.max(r);
    }

    pub fn green(&mut self, g: usize) {
        self.g = self.g.max(g);
    }

    pub fn blue(&mut self, b: usize) {
        self.b = self.b.max(b);
    }

    pub fn in_bounds(&self, r: usize, g: usize, b: usize) -> bool {
        self.r <= r && self.g <= g && self.b <= b
    }

    pub fn power(&self) -> usize {
        self.r * self.g * self.b
    }
}
