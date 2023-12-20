use std::{cmp::Ordering, fmt::Debug};

use itertools::Itertools;
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment<T> {
    pub lo: isize,
    pub hi: isize,
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct SegmentSequence<T> {
    inner: Vec<Segment<T>>,
}

pub enum Overlap<'a, 'b, T> {
    New {
        new: &'b Segment<T>,
    },
    Both {
        new: &'b Segment<T>,
        old: &'a Segment<T>,
    },
}

impl<T: Clone> Segment<T> {
    pub fn new(lo: isize, hi: isize, value: T) -> Option<Self> {
        if lo > hi {
            return None;
        }

        Some(Self { lo, hi, value })
    }

    pub fn split(self, mid: isize) -> Option<(Self, Self)> {
        if mid <= self.lo || mid >= self.hi {
            return None;
        }

        let lower = Self {
            lo: self.lo,
            hi: mid,
            value: self.value.clone(),
        };

        let upper = Self {
            lo: mid,
            hi: self.hi,
            value: self.value,
        };

        Some((lower, upper))
    }

    pub fn intersect(&self, other: &Self, value: T) -> Option<Self> {
        if self.lo >= other.hi || self.hi <= other.lo {
            return None;
        }

        Some(Segment {
            lo: self.lo.max(other.lo),
            hi: self.hi.min(other.hi),
            value,
        })
    }
}

impl<T: Clone + PartialEq> SegmentSequence<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn from_iterator(iter: impl Iterator<Item = Segment<T>>) -> Option<Self> {
        let inner = iter.sorted_by_key(|s| s.lo).collect_vec();

        if inner.windows(2).any(|segs| segs[0].hi > segs[1].lo) {
            return None;
        }

        Some(Self { inner })
    }

    pub fn segments(&self) -> impl Iterator<Item = &Segment<T>> {
        self.inner.iter()
    }

    pub fn search(&self, point: isize) -> Option<&Segment<T>> {
        self.binary_search(point).ok().map(|idx| &self.inner[idx])
    }

    pub fn range(&self, mut lo: isize, hi: isize) -> impl Iterator<Item = Segment<Option<T>>> {
        let mut acc = Vec::new();

        let mut idx = match self.binary_search(lo) {
            Err(i) => i,
            Ok(i) => i,
        };

        while lo < hi {
            let Some(seg) = &self.inner.get(idx) else {
                acc.push(Segment::new(lo, hi, None).unwrap());
                break;
            };

            if lo < seg.lo {
                let next = seg.lo.min(hi);
                acc.push(Segment::new(lo, next, None).unwrap());
                lo = next;
            }

            if seg.lo < hi {
                idx += 1;
                let next = seg.hi.min(hi);
                acc.push(Segment::new(lo, next, Some(seg.value.clone())).unwrap());
                lo = next;
            }
        }

        acc.into_iter()
    }

    fn binary_search(&self, point: isize) -> Result<usize, usize> {
        self.inner
            .binary_search_by(|seg| match (point >= seg.lo, point < seg.hi) {
                (true, true) => Ordering::Equal,
                (false, _) => Ordering::Greater,
                (true, _) => Ordering::Less,
            })
    }

    pub fn insert<F>(&mut self, new: Segment<T>, f: F) -> bool
    where
        F: Fn(T, T) -> T + Copy,
    {
        match self.binary_search(new.lo) {
            Err(idx) => {
                if let Some(next) = self.inner.get_mut(idx) {
                    if new.hi > next.lo {
                        let (lower, upper) = new.split(next.lo).unwrap();
                        return self.insert(lower, f) || self.insert(upper, f);
                    }
                };

                self.inner.insert(idx, new);
                false
            }

            Ok(idx) => {
                let seg = &self.inner[idx];
                if new.hi > seg.hi {
                    let (lower, upper) = new.split(seg.hi).unwrap();
                    self.merge(lower, idx, f);
                    self.insert(upper, f);
                } else {
                    self.merge(new, idx, f);
                }

                true
            }
        }
    }

    fn merge<F>(&mut self, inner: Segment<T>, idx: usize, f: F)
    where
        F: Fn(T, T) -> T + Copy,
    {
        let outer = &self.inner[idx];

        let mut replacements: SmallVec<[Segment<T>; 3]> = SmallVec::new();

        if outer.lo < inner.lo {
            replacements.push(Segment {
                lo: outer.lo,
                hi: inner.lo,
                value: outer.value.clone(),
            });
        }

        replacements.push(Segment {
            lo: inner.lo,
            hi: inner.hi,
            value: f(outer.value.clone(), inner.value.clone()),
        });

        if outer.hi > inner.hi {
            replacements.push(Segment {
                lo: inner.hi,
                hi: outer.hi,
                value: outer.value.clone(),
            });
        }

        self.inner.splice(idx..=idx, replacements);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Segment<T>) -> bool,
    {
        self.inner.retain(f)
    }

    pub fn simplify(self) -> Self {
        if self.inner.len() < 2 {
            return self;
        }

        let mut iter = self.inner.into_iter();

        let mut groups = Vec::new();
        groups.push(vec![iter.next().unwrap()]);

        for segment in iter {
            let last = groups.last().unwrap().last().unwrap();
            if segment.value == last.value && segment.lo == last.hi {
                groups.last_mut().unwrap().push(segment);
            } else {
                groups.push(vec![segment]);
            }
        }

        Self {
            inner: groups
                .into_iter()
                .map(|g| {
                    let first = g.first().unwrap();
                    let last = g.last().unwrap();

                    Segment::new(first.lo, last.hi, last.value.clone()).unwrap()
                })
                .collect_vec(),
        }
    }
}

impl<T: Clone + PartialEq> Default for SegmentSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment() {
        let seg = Segment::new(0, 10, 100);
        let bad = Segment::new(10, 5, ());

        assert!(seg.is_some());
        assert!(bad.is_none());

        let seg = seg.unwrap();

        assert_eq!(seg.value, 100);
        assert!(seg.clone().split(20).is_none());

        let (lower, upper) = seg.split(5).unwrap();
        assert_eq!(
            lower,
            Segment {
                lo: 0,
                hi: 5,
                value: 100
            }
        );
        assert_eq!(
            upper,
            Segment {
                lo: 5,
                hi: 10,
                value: 100
            }
        );
    }

    #[test]
    fn sequence_search() {
        let seq = SegmentSequence::from_iterator(
            [
                Segment::new(0, 3, 200).unwrap(),
                Segment::new(3, 6, 400).unwrap(),
                Segment::new(7, 9, 100).unwrap(),
            ]
            .into_iter(),
        )
        .unwrap();

        assert_eq!(seq.search(0).map(|s| s.value), Some(200));
        assert_eq!(seq.search(1).map(|s| s.value), Some(200));

        assert_eq!(seq.search(4).map(|s| s.value), Some(400));
        assert_eq!(seq.search(5).map(|s| s.value), Some(400));

        assert_eq!(seq.search(6).map(|s| s.value), None);

        assert_eq!(seq.search(7).map(|s| s.value), Some(100));
        assert_eq!(seq.search(8).map(|s| s.value), Some(100));
    }

    #[test]
    fn sequence_insert() {
        let mut seq = SegmentSequence::from_iterator(
            [
                Segment::new(0, 3, 200).unwrap(),
                Segment::new(3, 6, 400).unwrap(),
                Segment::new(7, 9, 100).unwrap(),
            ]
            .into_iter(),
        )
        .unwrap();

        seq.insert(Segment::new(1, 5, 50).unwrap(), |old, new| old + new);
        seq.insert(Segment::new(3, 5, 1).unwrap(), |old, new| old + new);
        seq.insert(Segment::new(5, 8, 20).unwrap(), |old, new| old + new);

        let segs = seq.segments().cloned().collect_vec();
        assert_eq!(
            segs,
            vec![
                Segment::new(0, 1, 200).unwrap(),
                Segment::new(1, 3, 250).unwrap(),
                Segment::new(3, 5, 451).unwrap(),
                Segment::new(5, 6, 420).unwrap(),
                Segment::new(6, 7, 20).unwrap(),
                Segment::new(7, 8, 120).unwrap(),
                Segment::new(8, 9, 100).unwrap(),
            ]
        );
    }

    #[test]
    fn sequence_range() {
        let seq = SegmentSequence::from_iterator(
            [
                Segment::new(0, 3, 200).unwrap(),
                Segment::new(3, 6, 400).unwrap(),
                Segment::new(7, 9, 100).unwrap(),
            ]
            .into_iter(),
        )
        .unwrap();

        let segs = seq.range(1, 8).collect_vec();
        assert_eq!(
            segs,
            vec![
                Segment::new(1, 3, Some(200)).unwrap(),
                Segment::new(3, 6, Some(400)).unwrap(),
                Segment::new(6, 7, None).unwrap(),
                Segment::new(7, 8, Some(100)).unwrap(),
            ]
        );
    }
}
