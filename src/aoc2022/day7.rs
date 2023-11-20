use std::fs;

enum Entry {
    Dir(Vec<(String, Entry)>, u64),
    File(u64),
}

fn parse_entry(s: &str) -> (String, Entry) {
    let (a, b) = s.split_once(" ").unwrap();
    match a {
        "dir" => (b.to_string(), Entry::Dir(Vec::new(), 0)),
        size @ _ => (b.to_string(), Entry::File(size.parse().unwrap())),
    }
}

fn print_tree(e: &Entry) {
    match e {
        Entry::Dir(v, s) => {
            println!("- / (dir, size={})", s);
            v.iter().for_each(|e| print_indent(e, 1));
        }
        _ => unreachable!(),
    }
}

fn print_indent(e: &(String, Entry), indent: usize) {
    match e {
        (a, Entry::Dir(v, size)) => {
            println!("{}- {} (dir, size={})", " ".repeat(indent * 2), a, size);
            v.iter().for_each(|e| print_indent(e, indent + 1));
        }
        (a, Entry::File(size)) => {
            println!("{}- {} (file, size={})", " ".repeat(indent * 2), a, size);
        }
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
            ".." => { pwd.pop(); }
            e @ _ => { pwd.push(String::from(e)); }
        }
    } else if cmd.starts_with("ls") {
        for dir in pwd {
            root = match root {
                Entry::Dir(v, _) => &mut v.iter_mut().find(|e| e.0 == *dir).unwrap().1,
                _ => unreachable!(),
            }
        }

        if let Entry::Dir(v, _) = root {
            v.extend(cmd.split_once("\n").unwrap().1.trim().split("\n").map(parse_entry));
        }
    } else {
        unreachable!()
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut root = Entry::Dir(vec![], 0);
    let mut pwd = Vec::new();

    for cmd in content.split("$ ").skip(2) {
        build_fs(&mut root, &mut pwd, cmd);
    }
    calculate_sizes(&mut root);

    print_tree(&root);
    println!();

    println!("1: {}", sum_directories::<100000>(&root));

    let to_free = if let Entry::Dir(_, tot) = root {
        tot - 40000000
    } else {
        unreachable!()
    };

    println!("2: {}", smallest_dir(&root, to_free).unwrap());
}
