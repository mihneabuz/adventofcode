use lib::aoc;
use lib::challenge::Challenge;

use std::cmp;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

type Graph = Vec<(u32, Vec<usize>)>;
type Cache = HashMap<(usize, u32, usize), u32>;

pub struct Day16;

impl Challenge for Day16 {
    aoc!(year = 2022, day = 16);

    fn solve(input: String) -> (String, String) {
        let valves = input.lines().map(parse_valve).collect::<Vec<_>>();
        let n = valves.len();

        let mut counter = 0usize;
        let mut valve_map = HashMap::new();

        let mut graph = vec![(0, Vec::new()); n];
        for valve in valves {
            let idx = *valve_map.entry(valve.0).or_insert_with(|| {
                counter += 1;
                counter - 1
            });
            graph[idx].0 = valve.1;

            for neigh in valve.2 {
                let neight_idx = *valve_map.entry(neigh).or_insert_with(|| {
                    counter += 1;
                    counter - 1
                });
                graph[idx].1.push(neight_idx);
            }
        }

        // let start = *valve_map.get("AA").unwrap();
        // let fst = solve(start, 30, &mut Vec::new(), &graph);
        // let snd = solve_2(start, 26, &mut Vec::new(), &graph);
        let fst = "Nope";
        let snd = "Nope";

        (fst.to_string(), snd.to_string())
    }
}

fn parse_valve(s: &str) -> (&str, u32, Vec<&str>) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (..(, .+)*)")
                .unwrap();
    }

    let cap = RE.captures(s).unwrap();

    let name = cap.get(1).unwrap().as_str();
    let pressure = cap.get(2).unwrap().as_str().parse().unwrap();

    let neighs = cap.get(3).unwrap().as_str().split(", ").collect::<Vec<_>>();

    (name, pressure, neighs)
}

fn hash_vec(v: &Vec<usize>) -> usize {
    v.iter().map(|x| 1 << x).sum()
}

fn _solve(idx: usize, time: u32, opened: &mut Vec<usize>, graph: &Graph, cache: &mut Cache) -> u32 {
    if time == 0 {
        return 0;
    }

    let entry = (idx, time, hash_vec(opened));
    if let Some(res) = cache.get(&entry) {
        return *res;
    }

    let node = &graph[idx];
    let mut best = 0;

    if node.0 > 0 && opened.iter().copied().find(|&i| i == idx).is_none() {
        opened.push(idx);
        best = cmp::max(
            best,
            node.0 * (time - 1) + _solve(idx, time - 1, opened, graph, cache),
        );
        opened.pop();
    }

    for neigh in node.1.iter().copied() {
        best = cmp::max(best, _solve(neigh, time - 1, opened, graph, cache));
    }

    cache.insert(entry, best);

    best
}

fn solve(idx: usize, time_remaining: u32, opened: &mut Vec<usize>, graph: &Graph) -> u32 {
    let mut cache = HashMap::new();
    _solve(idx, time_remaining, opened, graph, &mut cache)
}

fn _solve_2(
    idx1: usize,
    idx2: usize,
    time: u32,
    opened: &mut Vec<usize>,
    graph: &Graph,
    cache: &mut Cache,
) -> u32 {
    if time == 0 {
        return 0;
    }

    let entry = ((idx1 + 10) * (idx2 + 10), time, hash_vec(opened));
    if let Some(res) = cache.get(&entry) {
        return *res;
    }

    let node1 = &graph[idx1];
    let node2 = &graph[idx2];

    let mut best = 0;

    let has_valve1 = node1.0 > 0 && opened.iter().copied().find(|&i| i == idx1).is_none();
    let has_valve2 =
        idx1 != idx2 && node2.0 > 0 && opened.iter().copied().find(|&i| i == idx2).is_none();

    if has_valve1 && has_valve2 {
        opened.push(idx2);
        opened.push(idx1);
        best = cmp::max(
            best,
            (node1.0 + node2.0) * (time - 1) + _solve_2(idx1, idx2, time - 1, opened, graph, cache),
        );
        opened.pop();
        opened.pop();
    }

    if has_valve1 {
        opened.push(idx1);
        for neigh2 in node2.1.iter().copied() {
            best = cmp::max(
                best,
                node1.0 * (time - 1) + _solve_2(idx1, neigh2, time - 1, opened, graph, cache),
            );
        }
        opened.pop();
    }

    if has_valve2 {
        opened.push(idx2);
        for neigh1 in node1.1.iter().copied() {
            best = cmp::max(
                best,
                node2.0 * (time - 1) + _solve_2(neigh1, idx2, time - 1, opened, graph, cache),
            );
        }
        opened.pop();
    }

    for neigh1 in node1.1.iter().copied() {
        for neigh2 in node2.1.iter().copied() {
            best = cmp::max(
                best,
                _solve_2(neigh1, neigh2, time - 1, opened, graph, cache),
            );
        }
    }

    cache.insert(entry, best);

    best
}

fn solve_2(idx: usize, time_remaining: u32, opened: &mut Vec<usize>, graph: &Graph) -> u32 {
    let mut cache = HashMap::new();
    _solve_2(idx, idx, time_remaining, opened, graph, &mut cache)
}
