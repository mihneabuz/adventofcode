use std::ops::{BitAnd, BitOr, Shl};

use num::{Integer, PrimInt};

#[derive(Clone, Copy)]
pub struct Bitset<T> {
    storage: T,
}

impl<T> Bitset<T>
where
    T: Copy + Integer + PrimInt,
{
    pub fn new() -> Self {
        Self { storage: T::zero() }
    }

    pub fn set<B>(self, bit: B) -> Self
    where
        B: Ord,
        T: Shl<B, Output = T> + BitOr<Output = T>,
    {
        Self {
            storage: self.storage | (T::one() << bit),
        }
    }

    pub fn get<B>(&self, bit: B) -> bool
    where
        B: Ord,
        T: Shl<B, Output = T> + BitAnd<Output = T>,
    {
        self.storage & (T::one() << bit) > T::zero()
    }

    pub fn union(self, other: Self) -> Self
    where
        T: BitOr<Output = T>,
    {
        Self {
            storage: self.storage | other.storage,
        }
    }

    pub fn intersect(self, other: Self) -> Self
    where
        T: BitAnd<Output = T>,
    {
        Self {
            storage: self.storage & other.storage,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.storage == T::zero()
    }

    pub fn count_bits(&self) -> u32
    where
        T: BitAnd<Output = T>,
    {
        self.storage.count_ones()
    }

    pub fn clear(&mut self) {
        self.storage = T::zero()
    }
}

impl<T> Default for Bitset<T>
where
    T: Copy + Integer + PrimInt,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Bitset;

    #[test]
    fn create_bitmap() {
        let _: Bitset<u8> = Bitset::new();
        let _: Bitset<u16> = Bitset::new();
        let _: Bitset<u32> = Bitset::new();
        let _: Bitset<u64> = Bitset::new();
        let _: Bitset<u128> = Bitset::new();
    }

    #[test]
    fn set_bits() {
        let mut b: Bitset<u128> = Bitset::new();

        b = b.set(0);
        b = b.set(1);
        b = b.set(3);
        b = b.set(4);

        assert_eq!(b.storage, 27u128);
    }

    #[test]
    fn get_bits() {
        let mut b: Bitset<u128> = Bitset::new();

        b = b.set(7);
        b = b.set(80);

        assert_eq!(b.get(80), true);
        assert_eq!(b.get(72), false);
    }

    #[test]
    fn count_bits() {
        let mut b: Bitset<u128> = Bitset::new();

        b = b.set(0);
        b = b.set(1);
        b = b.set(3);
        b = b.set(4);

        assert_eq!(b.count_bits(), 4);
    }

    #[test]
    fn union() {
        let mut b1: Bitset<u128> = Bitset::new();
        let mut b2: Bitset<u128> = Bitset::new();

        b1 = b1.set(3);
        b2 = b2.set(7);

        let b = b1.union(b2);

        assert_eq!(b.get(3), true);
        assert_eq!(b.get(7), true);
        assert_eq!(b.get(6), false);
        assert_eq!(b.count_bits(), 2);
    }

    #[test]
    fn intersect() {
        let mut b1: Bitset<u128> = Bitset::new();
        let mut b2: Bitset<u128> = Bitset::new();

        b1 = b1.set(3);
        b1 = b1.set(7);
        b2 = b2.set(7);

        let b = b1.intersect(b2);

        assert_eq!(b.get(3), false);
        assert_eq!(b.get(7), true);
        assert_eq!(b.get(6), false);
        assert_eq!(b.count_bits(), 1);
    }
}
