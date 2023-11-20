use lib::aoc;
use lib::challenge::Challenge;

pub struct Day7;

impl Challenge for Day7 {
    aoc!(year = 2022, day = 7);

    fn solve(input: String) -> (String, String) {
        let mut root = Entry::Dir(vec![], 0);
        let mut pwd = Vec::new();

        for cmd in input.split("$ ").skip(2) {
            build_fs(&mut root, &mut pwd, cmd);
        }
        calculate_sizes(&mut root);

        let fst = sum_directories::<100000>(&root);

        let to_free = if let Entry::Dir(_, tot) = root {
            tot - 40000000
        } else {
            unreachable!()
        };

        let snd = smallest_dir(&root, to_free).unwrap();

        (fst.to_string(), snd.to_string())
    }
}

enum Entry {
    Dir(Vec<(String, Entry)>, u64),
    File(u64),
}

fn parse_entry(s: &str) -> (String, Entry) {
    let (a, b) = s.split_once(' ').unwrap();
    match a {
        "dir" => (b.to_string(), Entry::Dir(Vec::new(), 0)),
        size => (b.to_string(), Entry::File(size.parse().unwrap())),
    }
}

fn calculate_sizes(e: &mut Entry) -> u64 {
    match e {
        Entry::File(size) => *size,
        Entry::Dir(v, size) => {
            *size = v.iter_mut().map(|e| calculate_sizes(&mut e.1)).sum();
            *size
        }
    }
}

fn sum_directories<const T: u64>(e: &Entry) -> u64 {
    match e {
        Entry::File(_) => 0,
        Entry::Dir(v, size) => {
            v.iter().map(|e| sum_directories::<T>(&e.1)).sum::<u64>()
                + if *size < T { *size } else { 0 }
        }
    }
}

fn smallest_dir(e: &Entry, target: u64) -> Option<u64> {
    match e {
        Entry::File(_) => None,
        Entry::Dir(v, size) => v
            .iter()
            .filter_map(|e| smallest_dir(&e.1, target))
            .chain(Some(*size))
            .filter(|&size| size >= target)
            .min(),
    }
}

fn build_fs(mut root: &mut Entry, pwd: &mut Vec<String>, cmd: &str) {
    if cmd.starts_with("cd") {
        match cmd[3..].trim() {
            ".." => {
                pwd.pop();
            }
            e => {
                pwd.push(String::from(e));
            }
        }
    } else if cmd.starts_with("ls") {
        for dir in pwd {
            root = match root {
                Entry::Dir(v, _) => &mut v.iter_mut().find(|e| e.0 == *dir).unwrap().1,
                _ => unreachable!(),
            }
        }

        if let Entry::Dir(v, _) = root {
            v.extend(
                cmd.split_once('\n')
                    .unwrap()
                    .1
                    .trim()
                    .split('\n')
                    .map(parse_entry),
            );
        }
    } else {
        unreachable!()
    }
}
