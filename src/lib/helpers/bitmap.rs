use std::ops::{BitAnd, BitOr, Shl};

use num::{Integer, PrimInt};

pub struct Bitmap<T> {
    storage: T,
}

impl<T> Bitmap<T>
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

    pub fn count_bits(&self) -> u32
    where
        T: BitAnd<Output = T>,
    {
        self.storage.count_ones()
    }
}

impl<T> Default for Bitmap<T>
where
    T: Copy + Integer + PrimInt,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmap;

    #[test]
    fn create_bitmap() {
        let _: Bitmap<u8> = Bitmap::new();
        let _: Bitmap<u16> = Bitmap::new();
        let _: Bitmap<u32> = Bitmap::new();
        let _: Bitmap<u64> = Bitmap::new();
        let _: Bitmap<u128> = Bitmap::new();
    }

    #[test]
    fn set_bits() {
        let mut b: Bitmap<u128> = Bitmap::new();

        b = b.set(0);
        b = b.set(1);
        b = b.set(3);
        b = b.set(4);

        assert_eq!(b.storage, 27u128);
    }

    #[test]
    fn get_bits() {
        let mut b: Bitmap<u128> = Bitmap::new();

        b = b.set(7);
        b = b.set(80);

        assert_eq!(b.get(80), true);
        assert_eq!(b.get(72), false);
    }

    #[test]
    fn count_bits() {
        let mut b: Bitmap<u128> = Bitmap::new();

        b = b.set(0);
        b = b.set(1);
        b = b.set(3);
        b = b.set(4);

        assert_eq!(b.count_bits(), 4);
    }

    #[test]
    fn union() {
        let mut b1: Bitmap<u128> = Bitmap::new();
        let mut b2: Bitmap<u128> = Bitmap::new();

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
        let mut b1: Bitmap<u128> = Bitmap::new();
        let mut b2: Bitmap<u128> = Bitmap::new();

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
