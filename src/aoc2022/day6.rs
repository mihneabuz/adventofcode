use std::fs;

fn distinc_chars(s: &str) -> bool {
    let mut seen = vec![false; 26];
    for b in s.bytes().map(|b| (b - 'a' as u8) as usize) {
        if seen[b] {
            return false;
        }

        seen[b] = true;
    }

    true
}

fn first_distinct_seq(s: &str, w: usize) -> Option<usize> {
    for i in 0..s.len() - w {
        if distinc_chars(&s[i..i + w]) {
            return Some(i + w)
        }
    }

    None
}

fn main() {
    let stream = fs::read_to_string("input").unwrap();
    println!("1: {}", first_distinct_seq(&stream, 4).unwrap());
    println!("2: {}", first_distinct_seq(&stream, 14).unwrap());
}
