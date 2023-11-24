use lib::aoc;
use lib::challenge::Challenge;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day19;

impl Challenge for Day19 {
    aoc!(year = 2022, day = 19);

    fn solve(input: String) -> (String, String) {
        let blueprints = input.lines().map(parse_blueprint).collect::<Vec<_>>();

        let res1 = blueprints
            .iter()
            .enumerate()
            .map(|(i, bp)| (i + 1) * solve(24, 1000, bp))
            .sum::<usize>();

        let res2 = blueprints
            .iter()
            .take(3)
            .map(|bp| solve(32, 2000, bp))
            .product::<usize>();

        (res1.to_string(), res2.to_string())
    }
}

fn parse_blueprint(s: &str) -> Blueprint {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(
                r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."
            ).unwrap();
    }

    let cap = RE.captures(s).unwrap();

    let ore = cap.get(2).unwrap().as_str().parse().unwrap();
    let clay = cap.get(3).unwrap().as_str().parse().unwrap();
    let obs1 = cap.get(4).unwrap().as_str().parse().unwrap();
    let obs2 = cap.get(5).unwrap().as_str().parse().unwrap();
    let geo1 = cap.get(6).unwrap().as_str().parse().unwrap();
    let geo2 = cap.get(7).unwrap().as_str().parse().unwrap();

    Blueprint::new(ore, clay, (obs1, obs2), (geo1, geo2))
}

struct Blueprint {
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32),

    ore_val: i32,
    clay_val: i32,
    obsidian_val: i32,
    geode_val: i32,
}

impl Blueprint {
    fn new(ore: i32, clay: i32, obsidian: (i32, i32), geode: (i32, i32)) -> Self {
        let (mut ore_val, mut clay_val) = (1, 1);

        if clay > ore {
            clay_val = (clay / ore).max(1);
        } else {
            ore_val = (ore / clay).max(1);
        }

        let obsidian_val = obsidian.0 * ore_val + obsidian.1 * clay_val;
        let geode_val = geode.0 * ore_val + geode.1 * obsidian_val;

        Self {
            ore,
            clay,
            obsidian,
            geode,
            ore_val,
            clay_val,
            obsidian_val,
            geode_val,
        }
    }
}

#[derive(Clone)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,

    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: i32,
    geode_robot: i32,
}

impl State {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }

    fn step(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
}

fn solve(time: i32, max_branches: usize, bp: &Blueprint) -> usize {
    let mut states = vec![State::new()];
    for i in 0..time {
        let mut next_states = Vec::new();

        for mut state in states.into_iter() {
            state.step();

            next_states.push(state.clone());

            if state.ore >= bp.ore {
                state.ore -= bp.ore;
                state.ore -= 1;
                state.ore_robot += 1;
                next_states.push(state.clone());
                state.ore_robot -= 1;
                state.ore += 1;
                state.ore += bp.ore;
            }

            if state.ore >= bp.clay {
                state.ore -= bp.clay;
                state.clay -= 1;
                state.clay_robot += 1;
                next_states.push(state.clone());
                state.clay_robot -= 1;
                state.clay += 1;
                state.ore += bp.clay;
            }

            if state.ore >= bp.obsidian.0 && state.clay >= bp.obsidian.1 {
                state.ore -= bp.obsidian.0;
                state.clay -= bp.obsidian.1;
                state.obsidian -= 1;
                state.obsidian_robot += 1;
                next_states.push(state.clone());
                state.obsidian_robot -= 1;
                state.obsidian += 1;
                state.clay += bp.obsidian.1;
                state.ore += bp.obsidian.0;
            }

            if state.ore >= bp.geode.0 && state.obsidian >= bp.geode.1 {
                state.ore -= bp.geode.0;
                state.obsidian -= bp.geode.1;
                state.geode -= 1;
                state.geode_robot += 1;
                next_states.push(state.clone());
                state.geode_robot -= 1;
                state.geode += 1;
                state.obsidian += bp.geode.1;
                state.ore += bp.geode.0;
            }
        }

        if next_states.len() > max_branches {
            next_states.sort_by_key(|s| -euristic(s, bp, i, time));
            next_states.drain(max_branches..);
        }

        states = next_states;
    }

    states.into_iter().map(|s| s.geode).max().unwrap() as usize
}

fn euristic(s: &State, bp: &Blueprint, t: i32, total: i32) -> i32 {
    let robots = s.ore_robot * bp.ore_val
        + s.clay_robot * bp.clay_val
        + s.obsidian_robot * bp.obsidian_val * 4
        + s.geode_robot * bp.geode_val * 16;

    let rocks = s.obsidian * bp.obsidian_val * 4 + (s.geode + 1) * bp.geode_val * 16;

    robots * (total - t) + rocks
}
