use lib::{helpers, prelude::*};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use smallvec::SmallVec;
use std::{cmp::Reverse, collections::BTreeSet, hash::Hash};

struct Mapper<S> {
    forward: HashMap<S, usize>,
    backward: HashMap<usize, S>,
    index: usize,
}

impl<S> Mapper<S>
where
    S: Hash + Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            backward: HashMap::new(),
            index: 0,
        }
    }

    fn index(&mut self) -> usize {
        let index = self.index;
        self.index += 1;
        index
    }

    pub fn get(&mut self, value: S) -> usize {
        if let Some(index) = self.forward.get(&value) {
            *index
        } else {
            let index = self.index();
            self.forward.insert(value.clone(), index);
            self.backward.insert(index, value);
            index
        }
    }

    pub fn rev(&self, key: usize) -> Option<S> {
        self.backward.get(&key).cloned()
    }
}

struct Graph {
    edges: HashSet<(usize, usize)>,
}

impl Graph {
    pub fn from_edges<I>(iter: I) -> Self
    where
        I: Iterator<Item = (usize, usize)>,
    {
        Self {
            edges: iter.map(|(v1, v2)| (v1.min(v2), v1.max(v2))).collect(),
        }
    }

    fn has_edge(&self, (v1, v2): (usize, usize)) -> bool {
        self.edges.contains(&(v1.min(v2), v1.max(v2)))
    }

    fn cliques(&self, max: usize) -> Vec<(usize, Vec<BTreeSet<usize>>)> {
        let mut res: Vec<(usize, Vec<BTreeSet<usize>>)> = vec![];
        res.push((
            2,
            self.edges
                .iter()
                .map(|&(n1, n2)| BTreeSet::from_iter([n1, n2]))
                .collect(),
        ));

        loop {
            let (k, cliques) = &res.last().unwrap();
            if *k == max {
                break;
            }

            let mut next = BTreeSet::new();
            for (u, v) in cliques.iter().tuple_combinations::<(_, _)>() {
                let w = u
                    .symmetric_difference(v)
                    .copied()
                    .collect::<SmallVec<[_; 16]>>();

                if w.len() == 2 && self.has_edge((w[0], w[1])) {
                    let mut x = u.clone();
                    x.insert(w[0]);
                    x.insert(w[1]);
                    next.insert(x);
                }
            }

            if next.is_empty() {
                break;
            }

            res.push((k + 1, next.into_iter().collect_vec()));
        }

        res
    }
}

impl FromIterator<(usize, usize)> for Graph {
    fn from_iter<T: IntoIterator<Item = (usize, usize)>>(iter: T) -> Self {
        Graph::from_edges(iter.into_iter())
    }
}

pub struct Day23;

impl Challenge for Day23 {
    aoc!(year = 2024, day = 23);

    fn solve(input: String) -> (String, String) {
        let mut mapper = Mapper::new();

        let graph = input
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .map(|(s1, s2)| (mapper.get(s1), mapper.get(s2)))
            .collect::<Graph>();

        let cliques = graph.cliques(3);

        let res1 = cliques
            .iter()
            .find(|(k, _)| *k == 2)
            .unwrap()
            .1
            .iter()
            .filter(|clique| {
                clique
                    .iter()
                    .map(|v| mapper.rev(*v).unwrap())
                    .any(|c| c.starts_with('t'))
            })
            .count();

        let best = &cliques
            .iter()
            .sorted_by_key(|(k, _)| Reverse(*k))
            .next()
            .unwrap()
            .1[0];

        let res2 = helpers::join(best.iter().map(|n| mapper.rev(*n).unwrap()).sorted(), ",");

        (res1.to_string(), res2)
    }
}
