use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::num::ParseIntError;

struct Step {
    count: i32,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();

        Ok(Step {
            count: words[1].parse()?,
            from: words[3].parse()?,
            to: words[5].parse()?,
        })
    }
}

impl Step {
    fn execute(&self, stacks: &mut [Vec<char>]) -> &Self {
        let (to, from) = self.get_stacks(stacks);
        for _ in 0..self.count {
            to.push(from.pop().unwrap());
        }
        self
    }

    // having some fun with unsafe draining :D

    fn execute_ordered(&self, stacks: &mut [Vec<char>]) -> &Self {
        let (to, from) = self.get_stacks(stacks);
        let n = from.len() - self.count as usize;
        to.extend(from.drain(n..));
        self
    }

    fn get_stacks(&self, stacks: &mut [Vec<char>]) -> (&mut Vec<char>, &mut Vec<char>) {
        unsafe {
            let to = &mut *(stacks.get_unchecked_mut(self.to - 1) as *mut _);
            let from = &mut *(stacks.get_unchecked_mut(self.from - 1) as *mut _);
            (to, from)
        }
    }
}

fn find_empty_line(lines: &[String]) -> usize {
    lines
        .iter()
        .enumerate()
        .find_map(|(i, l)| if l.is_empty() { Some(i) } else { None })
        .unwrap()
}

fn main() {
    let reader = BufReader::new(File::open("input").unwrap());
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let si = find_empty_line(&lines);
    let (init, steps) = lines.split_at(si);

    let n = lines[si - 1].split_whitespace().count();
    let mut stacks = vec![Vec::new(); n];

    for line in init[..si - 1].into_iter().map(|s| s.as_bytes()).rev() {
        for i in 0..n {
            if i < line.len() && line[4 * i + 1] != ' ' as u8 {
                stacks[i].push(line[4 * i + 1] as char);
            }
        }
    }

    let mut stacks_copy = stacks.clone();

    for step in steps[1..].into_iter() {
        step.parse::<Step>()
            .unwrap()
            .execute(&mut stacks)
            .execute_ordered(&mut stacks_copy);
    }

    println!("1: {}", stacks.into_iter().map(|s| *s.last().unwrap()).collect::<String>());
    println!("1: {}", stacks_copy.into_iter().map(|s| *s.last().unwrap()).collect::<String>());
}
