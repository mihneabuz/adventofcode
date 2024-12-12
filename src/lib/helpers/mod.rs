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
