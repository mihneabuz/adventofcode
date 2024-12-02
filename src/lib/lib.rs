pub mod challenge;
pub mod executor;
pub mod helpers;
pub mod inputs;
pub mod util;

pub mod prelude {
    pub use crate::challenge::{Challenge, ThreadedChallenge};
    pub use crate::{aoc, example, day, year};
}
