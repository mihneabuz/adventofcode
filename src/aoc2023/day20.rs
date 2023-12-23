use std::collections::HashMap;

use itertools::Itertools;

use lib::{aoc, challenge::Challenge};

pub struct Day20;

impl Challenge for Day20 {
    aoc!(year = 2023, day = 20);

    fn solve(input: String) -> (String, String) {
        let mut mods = parse(&input);

        let output = mods
            .values()
            .find(|module| module.output.contains(&"rx"))
            .unwrap();

        let mut cycles = vec![Vec::with_capacity(2); output.input.len()];
        let output_name = output.name;

        let (mut lows, mut highs) = (0, 0);
        for i in 0.. {
            let mut signals = vec![(false, "", "broadcaster")];
            while !signals.is_empty() {
                if i < 1000 {
                    for signal in signals.iter() {
                        match signal.0 {
                            false => lows += 1,
                            true => highs += 1,
                        }
                    }
                }

                signals = step(signals, &mut mods);

                let output = mods.get(output_name).unwrap();
                if let ModuleType::Conjunction(inputs) = &output.op {
                    for idx in inputs
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, &sign)| sign.then_some(idx))
                    {
                        if cycles[idx].last().copied().unwrap_or(0) != i {
                            cycles[idx].push(i);
                        }
                    }
                }
            }

            if cycles.iter().all(|cycle| cycle.len() >= 2) {
                break;
            }
        }

        let fst = lows * highs;
        let snd = cycles
            .into_iter()
            .map(|cycle| cycle[1] - cycle[0])
            .product::<usize>();

        (fst.to_string(), snd.to_string())
    }
}

#[derive(Clone, Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(Vec<bool>),
    Broadcast(bool),
}

#[derive(Clone, Debug)]
struct Module<'a> {
    name: &'a str,
    op: ModuleType,
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

type Signal<'a> = (bool, &'a str, &'a str);

fn step<'a>(signals: Vec<Signal<'a>>, mods: &mut HashMap<&'a str, Module<'a>>) -> Vec<Signal<'a>> {
    let mut touched = Vec::new();

    for (signal, from, to) in signals {
        let Some(module) = mods.get_mut(to) else {
            continue;
        };

        let emit = match (signal, &mut module.op) {
            (sign, ModuleType::Broadcast(old)) => {
                *old = sign;
                true
            }

            (false, ModuleType::FlipFlop(sign)) => {
                *sign = !*sign;
                true
            }

            (sign, ModuleType::Conjunction(inputs)) => {
                let idx = module.input.iter().position(|inp| *inp == from).unwrap();
                inputs[idx] = sign;
                true
            }

            _ => false,
        };

        if emit {
            touched.push(to);
        }
    }

    touched.into_iter().fold(Vec::new(), |mut acc, name| {
        let module = mods.get(name).unwrap();

        let sign = match &module.op {
            ModuleType::FlipFlop(sign) => *sign,
            ModuleType::Conjunction(signs) => signs.iter().any(|sign| !*sign),
            ModuleType::Broadcast(sign) => *sign,
        };

        acc.extend(module.output.iter().map(|out| (sign, module.name, *out)));

        acc
    })
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut mods = HashMap::new();

    for line in input.lines() {
        let (def, next) = line.split_once(" -> ").unwrap();
        let (op, name) = match def.as_bytes()[0] {
            b'%' => (ModuleType::FlipFlop(false), def.split_at(1).1),
            b'&' => (ModuleType::Conjunction(Vec::new()), def.split_at(1).1),
            b'b' => (ModuleType::Broadcast(false), def),
            _ => unreachable!(),
        };

        let module = Module {
            name,
            op,
            input: Vec::new(),
            output: next.split(", ").collect_vec(),
        };

        mods.insert(module.name, module);
    }

    for line in input.lines() {
        let (def, next) = line.split_once(" -> ").unwrap();
        let from = match def.as_bytes()[0] {
            b'%' | b'&' => def.split_at(1).1,
            b'b' => def,
            _ => unreachable!(),
        };

        for to in next.split(", ") {
            if let Some(to) = mods.get_mut(to) {
                to.input.push(from);
                if let ModuleType::Conjunction(inputs) = &mut to.op {
                    inputs.push(false);
                }
            }
        }
    }

    mods
}
