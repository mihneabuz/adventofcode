use lib::aoc;
use lib::challenge::Challenge;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Shl;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day16;

impl Challenge for Day16 {
    aoc!(year = 2022, day = 16);

    fn solve(input: String) -> (String, String) {
        let valves = input.lines().map(parse_valve).collect::<Vec<_>>();

        let mut graph = Graph::new();
        for valve in valves {
            graph.add_node(valve.0, valve.1);
            for neigh in valve.2 {
                graph.add_edge(valve.0, neigh, 1);
            }
        }

        let optimized = graph.simplify(0, vec!["AA"]).optimize();

        let mut solver = Solver::new(&optimized);
        let res1 = solver.run::<1>("AA", 30);
        let res2 = solver.run::<2>("AA", 26);

        (res1.to_string(), res2.to_string())
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
    let press = cap.get(2).unwrap().as_str().parse().unwrap();
    let neighs = cap.get(3).unwrap().as_str().split(", ").collect::<Vec<_>>();

    (name, press, neighs)
}

struct Graph<N, V> {
    values: HashMap<N, V>,
    neighs: HashMap<N, HashMap<N, usize>>,
}

impl<N, V> Graph<N, V>
where
    N: Eq + Hash + Clone,
    V: Eq,
{
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            neighs: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: N, value: V) {
        self.values.insert(node, value);
    }

    fn add_edge(&mut self, node1: N, node2: N, cost: usize) {
        self.neighs
            .entry(node1.clone())
            .or_default()
            .entry(node2.clone())
            .and_modify(|old| *old = cost.min(*old))
            .or_insert(cost);

        self.neighs
            .entry(node2.clone())
            .or_default()
            .entry(node1.clone())
            .and_modify(|old| *old = cost.min(*old))
            .or_insert(cost);
    }

    fn neighbors(&self, node: N) -> Vec<(N, usize)> {
        self.neighs
            .get(&node)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    fn prune(&mut self) {
        let nodes: HashSet<N> = self.values.keys().cloned().collect();
        self.neighs.retain(|k, _| nodes.contains(k));
        for neighs in self.neighs.values_mut() {
            neighs.retain(|k, _| nodes.contains(k));
        }
    }

    fn simplify(self, dead: V, retain: Vec<N>) -> Self {
        let mut new_graph = Self::new();

        for (node, value) in self.values.into_iter() {
            if !value.eq(&dead) || retain.contains(&node) {
                if let Some(neighs) = self.neighs.get(&node) {
                    for (neigh, cost) in neighs.iter() {
                        new_graph.add_edge(node.clone(), neigh.clone(), *cost);
                    }
                }

                new_graph.add_node(node, value);
            } else if let Some(neighs) = self.neighs.get(&node) {
                let neighs = new_graph
                    .neighbors(node)
                    .into_iter()
                    .chain(neighs.iter().map(|(k, v)| (k.clone(), *v)));

                for ((n1, c1), (n2, c2)) in neighs
                    .tuple_combinations::<(_, _)>()
                    .filter(|(a, b)| a.0 != b.0)
                {
                    new_graph.add_edge(n1.clone(), n2.clone(), c1 + c2);
                    new_graph.add_edge(n2.clone(), n1.clone(), c1 + c2);
                }
            }
        }

        new_graph.prune();

        new_graph
    }
}

impl<N, V> Graph<N, V>
where
    N: Eq + Hash + Clone,
    V: Eq + Into<u32>,
{
    fn optimize(self) -> OptimizedGraph<N> {
        let n = self.values.len();

        let mut node_map = HashMap::new();

        let mut new_graph = vec![(0, Vec::new()); n];

        for (index, (node, value)) in self.values.into_iter().enumerate() {
            node_map.insert(node.clone(), index);
            new_graph[index].0 = value.into();
        }

        for (node, neighs) in self.neighs.into_iter() {
            let index = node_map.get(&node).copied().unwrap();
            new_graph[index].1 = neighs
                .into_iter()
                .map(|(n, c)| (node_map.get(&n).copied().unwrap(), c as u32))
                .collect();
        }

        OptimizedGraph {
            translation: node_map,
            inner: new_graph,
        }
    }
}

struct OptimizedGraph<N> {
    translation: HashMap<N, usize>,
    inner: Vec<(u32, Vec<(usize, u32)>)>,
}

impl<N> OptimizedGraph<N>
where
    N: Eq + Hash,
{
    fn translate(&self, node: &N) -> Option<usize> {
        self.translation.get(node).copied()
    }

    fn get_value(&self, idx: usize) -> u32 {
        self.inner[idx].0
    }

    fn get_neighs(&self, idx: usize) -> &[(usize, u32)] {
        &self.inner[idx].1
    }
}

#[derive(Clone, Copy)]
struct Bitmap {
    inner: usize,
}

impl Bitmap {
    fn new() -> Self {
        Self { inner: 0 }
    }

    fn set<T>(&self, bit: T) -> Self
    where
        T: Ord,
        usize: Shl<T, Output = usize>,
    {
        Self {
            inner: self.inner | (1 << bit),
        }
    }

    fn has<T>(&self, bit: T) -> bool
    where
        usize: Shl<T, Output = usize>,
    {
        self.inner & (1 << bit) != 0
    }

    fn inner(&self) -> usize {
        self.inner
    }
}

struct Solver<'a> {
    graph: &'a OptimizedGraph<&'a str>,
    cache: HashMap<(usize, usize, u32), usize>,
    results: HashMap<usize, usize>,
    best: usize,
    heuristic: Option<u32>,
}

impl<'a> Solver<'a> {
    pub fn new(graph: &'a OptimizedGraph<&'a str>) -> Self {
        Self {
            graph,
            cache: HashMap::new(),
            results: HashMap::new(),
            best: 0,
            heuristic: None,
        }
    }

    pub fn run<const I: usize>(&mut self, start: &str, time: u32) -> usize {
        let idx = self.graph.translate(&start).unwrap();

        match I {
            1 => self.solve(idx, time, Bitmap::new()),
            2 => {
                self.heuristic = Some(time / 3);
                self.solve2(0, idx, time, Bitmap::new());

                let mut results: Vec<(usize, usize)> = self.results.drain().collect();
                results.sort_by_key(|r| std::cmp::Reverse(r.1));

                results
                    .into_iter()
                    .tuple_combinations()
                    .filter_map(|(a, b)| {
                        if a.0 & b.0 == 0 {
                            Some(a.1 + b.1)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap()
            }

            _ => panic!("too many runners"),
        }
    }

    fn solve(&mut self, idx: usize, time: u32, opened: Bitmap) -> usize {
        if time == 0 {
            return 0;
        }

        let key = (idx, opened.inner(), time);
        if let Some(value) = self.cache.get(&key) {
            return *value;
        }

        let value = self.graph.get_value(idx);
        let mut best = 0;

        if value > 0 && !opened.has(idx) {
            let pressure = (value * (time - 1)) as usize;
            best = best.max(pressure + self.solve(idx, time - 1, opened.set(idx)));
        }

        for &(neigh, cost) in self.graph.get_neighs(idx) {
            if time >= cost {
                best = best.max(self.solve(neigh, time - cost, opened));
            }
        }

        self.cache.insert(key, best);

        best
    }

    fn solve2(&mut self, total: usize, idx: usize, time: u32, opened: Bitmap) {
        let mut stuck = true;

        if let Some(heuristic) = self.heuristic {
            if time < heuristic && total < self.best / 2 {
                return;
            }
        }

        let value = self.graph.get_value(idx);
        if value > 0 && !opened.has(idx) && time > 0 {
            let pressure = (value * (time - 1)) as usize;
            self.solve2(total + pressure, idx, time - 1, opened.set(idx));
            stuck = false;
        }

        for &(neigh, cost) in self.graph.get_neighs(idx) {
            if time >= cost {
                self.solve2(total, neigh, time - cost, opened);
                stuck = false;
            }
        }

        if stuck && total > 0 {
            self.best = self.best.max(total);

            self.results
                .entry(opened.inner())
                .and_modify(|e| *e = (*e).max(total))
                .or_insert(total);
        }
    }
}
