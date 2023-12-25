use std::collections::HashMap;

use rand::Rng;

use lib::{aoc, challenge::Challenge};

pub struct Day25;

impl Challenge for Day25 {
    aoc!(year = 2023, day = 25);

    fn solve(input: String) -> (String, String) {
        let mut nodes_map = StrMapper::new();

        let mut nodes: HashMap<usize, usize> = HashMap::new();
        let mut edges = Vec::new();

        for line in input.lines() {
            let (name, rest) = line.split_once(": ").unwrap();
            let from = nodes_map.get(name);
            nodes.insert(from, 1);

            for other in rest.split_whitespace() {
                let to = nodes_map.get(other);
                nodes.insert(to, 1);
                edges.push((from, to));
            }
        }

        let mut rng = rand::thread_rng();
        let fst = loop {
            let mut nodes = nodes.clone();
            let mut edges = edges.clone();

            while nodes.len() > 2 {
                let i = rng.gen_range(0..edges.len());
                let (n1, n2) = edges[i];

                edges.swap_remove(i);
                *nodes.get_mut(&n1).unwrap() += nodes.remove(&n2).unwrap();

                for edge in edges.iter_mut() {
                    if edge.0 == n2 {
                        edge.0 = n1;
                    }

                    if edge.1 == n2 {
                        edge.1 = n1;
                    }
                }

                edges.retain(|edge| edge.0 != edge.1);
            }

            if edges.len() == 3 {
                break nodes.values().product::<usize>();
            }
        };

        (fst.to_string(), "".to_string())
    }
}

struct StrMapper {
    map: HashMap<String, usize>,
    k: usize,
}

impl StrMapper {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            k: 0,
        }
    }

    fn get(&mut self, s: &str) -> usize {
        match self.map.entry(s.to_owned()) {
            std::collections::hash_map::Entry::Occupied(o) => *o.get(),
            std::collections::hash_map::Entry::Vacant(v) => {
                self.k += 1;
                *v.insert(self.k)
            }
        }
    }
}
