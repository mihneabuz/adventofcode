use lib::prelude::*;

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Wire<'a> {
    Value(bool),
    Gate(&'a str, &'a str, &'a str),
}

struct Circuit<'a> {
    state: HashMap<&'a str, Wire<'a>>,
}

impl<'a> Circuit<'a> {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &'a str, value: Wire<'a>) {
        let _ = self.state.insert(name, value);
    }

    pub fn resolve(&mut self, name: &'a str) -> bool {
        match self.state.get(name).cloned().unwrap() {
            Wire::Value(val) => val,
            Wire::Gate(typ, x, y) => {
                let x = self.resolve(x);
                let y = self.resolve(y);

                let res = match typ {
                    "OR" => x | y,
                    "AND" => x & y,
                    "XOR" => x ^ y,
                    _ => unreachable!(),
                };

                self.insert(name, Wire::Value(res));

                res
            }
        }
    }

    // pub fn resolve_ref(&self, name: &'a str) -> bool {
    //     match self.state.get(name).cloned().unwrap() {
    //         Wire::Value(val) => val,
    //         Wire::Gate(typ, x, y) => {
    //             let x = self.resolve_ref(x);
    //             let y = self.resolve_ref(y);
    //
    //             let res = match typ {
    //                 "OR" => x | y,
    //                 "AND" => x & y,
    //                 "XOR" => x ^ y,
    //                 _ => unreachable!(),
    //             };
    //
    //             res
    //         }
    //     }
    // }
    //
    // pub fn get(&self, name: &str) -> &Wire<'_> {
    //     &self.state[name]
    // }
    //
    // pub fn trace(&self, name: &str) -> HashSet<&str> {
    //     match self.state[name] {
    //         Wire::Value(_) => HashSet::new(),
    //         Wire::Gate(_, x, y) => self
    //             .trace(x)
    //             .union(&self.trace(y))
    //             .copied()
    //             .chain([x, y])
    //             .collect(),
    //     }
    // }
    //
    // pub fn rename<'b>(&mut self, from: &'a str, to: &'b str)
    // where
    //     'b: 'a,
    // {
    //     let wire = self.state.remove(from).unwrap();
    //     self.state.insert(to, wire);
    //     for wire in self.state.values_mut() {
    //         if let Wire::Gate(_, x, y) = wire {
    //             if *x == from {
    //                 *x = to;
    //             }
    //
    //             if *y == from {
    //                 *y = to;
    //             }
    //         }
    //     }
    // }
    //
    // pub fn swap(&mut self, n1: &'a str, n2: &'a str) {
    //     let w1 = self.state.remove(n1).unwrap();
    //     let w2 = self.state.remove(n2).unwrap();
    //
    //     self.insert(n1, w2);
    //     self.insert(n2, w1);
    // }
    //
    // pub fn find(&mut self, g: &str, n1: &str, n2: &str) {
    //     let f = self
    //         .state
    //         .iter()
    //         .find(|(_, v)| {
    //             if let Wire::Gate(r, x, y) = v {
    //                 if g == *r {
    //                     if (*x == n1 && *y == n2) || (*x == n2 && *y == n1) {
    //                         return true;
    //                     }
    //                 }
    //             }
    //
    //             false
    //         })
    //         .unwrap();
    //
    //     println!("> {f:?}");
    // }
}

pub struct Day24;

impl Challenge for Day24 {
    aoc!(year = 2024, day = 24);

    fn solve(input: String) -> (String, String) {
        let (initial, gates) = input.split_once("\n\n").unwrap();

        let circuit = initial
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .fold(Circuit::new(), |mut state, (wire, value)| {
                let value = value.parse::<i32>().unwrap() > 0;
                state.insert(wire, Wire::Value(value));
                state
            });

        let mut end = Vec::new();
        let mut circuit = gates
            .lines()
            .map(|line| {
                let (gate, output) = line.split_once(" -> ").unwrap();
                let (input1, gate, input2) = gate.split_whitespace().collect_tuple().unwrap();
                (output, Wire::Gate(gate, input1, input2))
            })
            .fold(circuit, |mut state, (wire, gate)| {
                if wire.starts_with('z') {
                    end.push(wire);
                }
                state.insert(wire, gate);
                state
            });

        end.sort();

        // circuit.swap("z12", "vdc");
        // circuit.swap("z21", "nhn");
        // circuit.swap("tvb", "khg");
        // circuit.swap("z33", "gst");
        // for i in 1..end.len() - 1 {
        //     let wire = circuit.get(&format!("z{i:02}")).clone();
        //     if let Wire::Gate(g, carry, partial) = wire {
        //         if g != "XOR" {
        //             continue;
        //         }
        //         let carry = carry.to_string().leak();
        //         let partial = partial.to_string().leak();
        //         circuit.rename(carry, format!("c{:02}", i - 1).leak());
        //         circuit.rename(partial, format!("p{:02}", i).leak());
        //     }
        // }
        //
        // let mut used = HashSet::new();
        // let mut carry = 0;
        // for i in (0..end.len() - 1).take(55) {
        //     let (x, y, z) = (format!("x{i:02}"), format!("y{i:02}"), format!("z{i:02}"));
        //     println!("{} {}", x, y);
        //
        //     let (wx, wy, wz) = (
        //         circuit.resolve_ref(&x) as u32,
        //         circuit.resolve_ref(&y) as u32,
        //         circuit.resolve_ref(&z) as u32,
        //     );
        //     println!(" CIRCUIT {} {} -> {}", wx, wy, wz);
        //
        //     let s = wx + wy + carry;
        //     let gz = s % 2;
        //     let gcarry = (s >= 2) as u32;
        //     println!(
        //         " CORRECT {} {} carry {} -> {} carry {}",
        //         wx, wy, carry, gz, gcarry
        //     );
        //     carry = gcarry;
        //
        //     let gates = circuit.trace(&z);
        //     let curr = gates.difference(&used).sorted().collect_vec();
        //     println!(" GATES: {:?}", curr);
        //     println!("  {z} > {:?}", circuit.get(&z));
        //     for g in curr {
        //         if g.starts_with('x') || g.starts_with('y') {
        //             continue;
        //         }
        //         println!("  {g} > {:?}", circuit.get(g));
        //     }
        //
        //     used = used.union(&gates).copied().collect();
        //
        //     if wz != gz {
        //         println!("ERROR ERROR !");
        //         break;
        //     }
        //
        //     println!();
        // }
        // std::mem::drop(used);

        let res1 = end
            .into_iter()
            .enumerate()
            .fold(0u64, |mut acc, (i, wire)| {
                if circuit.resolve(wire) {
                    acc |= 1 << i;
                }
                acc
            });

        (res1.to_string(), "manual".to_string())
    }
}
