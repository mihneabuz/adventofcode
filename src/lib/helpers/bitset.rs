use std::{
    mem::size_of,
    ops::{BitAnd, BitOr, Shl},
};

use num::{Integer, PrimInt};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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

    pub fn unset<B>(self, bit: B) -> Self
    where
        B: Ord,
        T: Shl<B, Output = T> + BitOr<Output = T>,
    {
        Self {
            storage: self.storage & !(T::one() << bit),
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BitArray {
    inner: Vec<Bitset<usize>>,
    size: usize,
}

impl BitArray {
    pub fn new(size: usize) -> Self {
        let buckets = size / size_of::<usize>() + 1;

        Self {
            inner: vec![Bitset::new(); buckets],
            size,
        }
    }

    pub fn get(&self, idx: usize) -> Option<bool> {
        if idx >= self.size {
            return None;
        }

        let bucket = idx / size_of::<usize>();
        Some(self.inner[bucket].get(idx % size_of::<usize>()))
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        if idx >= self.size {
            return;
        }

        let bucket = &mut self.inner[idx / size_of::<usize>()];
        if value {
            *bucket = bucket.set(idx % size_of::<usize>());
        } else {
            *bucket = bucket.unset(idx % size_of::<usize>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bitset() {
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
    fn unset_bits() {
        let mut b: Bitset<u128> = Bitset::new();

        b = b.set(1);
        b = b.set(3);
        b = b.set(4);

        assert_eq!(b.storage, 26u128);

        b = b.unset(3);
        b = b.unset(4);

        assert_eq!(b.storage, 2u128);
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

    #[test]
    fn create_bitarray() {
        let _ = BitArray::new(1);
        let _ = BitArray::new(10);
        let _ = BitArray::new(100);
        let _ = BitArray::new(1000);
    }

    #[test]
    fn get_set_bitarray() {
        let mut array = BitArray::new(100);

        array.set(0, true);
        array.set(64, true);
        array.set(80, true);
        array.set(99, true);

        assert_eq!(array.get(0), Some(true));
        assert_eq!(array.get(1), Some(false));

        assert_eq!(array.get(63), Some(false));
        assert_eq!(array.get(64), Some(true));
        assert_eq!(array.get(65), Some(false));

        assert_eq!(array.get(80), Some(true));
        assert_eq!(array.get(99), Some(true));

        assert_eq!(array.get(100), None);

        array.set(0, false);
        array.set(80, false);

        assert_eq!(array.get(0), Some(false));
        assert_eq!(array.get(80), Some(false));
    }
}
