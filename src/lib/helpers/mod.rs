mod bitmap;
mod map;
mod segment;
mod trie;

pub use bitmap::*;
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
