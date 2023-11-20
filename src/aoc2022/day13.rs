use lib::aoc;
use lib::challenge::Challenge;

use std::cmp::Ordering;
use std::str;

pub struct Day13;

impl Challenge for Day13 {
    aoc!(year = 2022, day = 13);

    fn solve(input: String) -> (String, String) {
        let mut lists = input.split_whitespace().map(parse_list).collect::<Vec<_>>();

        let res1 = lists
            .chunks(2)
            .map(|l| compare_lists(&l[0], &l[1]) == Ordering::Less)
            .enumerate()
            .filter(|(_, b)| *b)
            .fold(0, |acc, (i, _)| acc + i + 1);

        lists.sort_by(compare_lists);

        let div1 = NestedList::List(vec![NestedList::List(vec![NestedList::Item(2)])]);
        let idx1 = lists
            .binary_search_by(|l| compare_lists(l, &div1))
            .unwrap_err();
        lists.insert(idx1, div1);

        let div2 = NestedList::List(vec![NestedList::List(vec![NestedList::Item(6)])]);
        let idx2 = lists
            .binary_search_by(|l| compare_lists(l, &div2))
            .unwrap_err();
        lists.insert(idx2, div2);

        (res1.to_string(), ((idx1 + 1) * (idx2 + 1)).to_string())
    }
}

#[derive(Clone)]
enum NestedList<T> {
    List(Vec<NestedList<T>>),
    Item(T),
}

fn parse_list(s: &str) -> NestedList<i64> {
    if let Ok(x) = s.parse::<i64>() {
        NestedList::Item(x)
    } else {
        let mut b = s.as_bytes();
        b = &b[1..b.len() - 1];

        let mut res = Vec::new();
        let mut i = 0;

        while i < b.len() {
            let mut j = i + 1;

            if b[i] == '[' as u8 {
                let mut open = 1;
                while open > 0 {
                    if b[j] == '[' as u8 {
                        open += 1;
                    }

                    if b[j] == ']' as u8 {
                        open -= 1;
                    }

                    j += 1;
                }
            } else {
                while j < b.len() && b[j] != ',' as u8 {
                    j += 1;
                }
            }

            res.push(parse_list(str::from_utf8(&b[i..j]).unwrap()));
            i = j + 1;
        }

        NestedList::List(res)
    }
}

fn compare_lists(l1: &NestedList<i64>, l2: &NestedList<i64>) -> Ordering {
    match (l1, l2) {
        (NestedList::Item(a), NestedList::Item(b)) => a.cmp(&b),
        (NestedList::Item(a), NestedList::List(b)) => compare_lists(
            &NestedList::List(vec![NestedList::Item(*a)]),
            &NestedList::List(b.clone()),
        ),
        (NestedList::List(a), NestedList::Item(b)) => compare_lists(
            &NestedList::List(a.clone()),
            &NestedList::List(vec![NestedList::Item(*b)]),
        ),
        (NestedList::List(a), NestedList::List(b)) => {
            let (mut iter1, mut iter2) = (a.iter(), b.iter());

            while let Some(item1) = iter1.next() {
                if let Some(item2) = iter2.next() {
                    match compare_lists(item1, item2) {
                        Ordering::Equal => {}
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    }
                } else {
                    return Ordering::Greater;
                }
            }

            if iter2.next().is_none() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        }
    }
}
