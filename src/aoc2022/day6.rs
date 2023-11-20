use lib::aoc;
use lib::challenge::Challenge;

pub struct Day6;

impl Challenge for Day6 {
    aoc!(year = 2022, day = 6);

    fn solve(input: String) -> (String, String) {
        let fst = first_distinct_seq(&input, 4).unwrap();
        let snd = first_distinct_seq(&input, 14).unwrap();

        (fst.to_string(), snd.to_string())
    }
}

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
