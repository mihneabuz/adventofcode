mod bitset;
mod map;
mod segment;
mod trie;

pub use bitset::*;
pub use map::*;
pub use segment::*;
pub use trie::*;

use std::str::FromStr;

pub fn unchecked_parse<T>(s: &str) -> T
where
    T: FromStr,
{
    s.parse::<T>().ok().unwrap()
}

pub fn join<I, T>(iter: I, sep: &str) -> String
where
    I: Iterator<Item = T>,
    T: ToString,
{
    itertools::intersperse(iter.map(|item| item.to_string()), sep.to_string()).collect()
}
