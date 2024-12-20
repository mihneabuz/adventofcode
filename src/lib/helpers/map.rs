use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use ndarray::{Array, Array2, Axis};
use num::Num;

#[derive(Clone)]
pub struct Map<T> {
    inner: Array2<T>,
}

type Dirs = &'static [(i32, i32)];

impl<T> Map<T>
where
    T: Default + Clone + Num,
{
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            inner: Array2::zeros((n, m)),
        }
    }
}

impl Map<u8> {
    pub fn from_text(input: &str) -> Self {
        let mut m = 0;
        let mut acc = Vec::new();

        for b in input.bytes() {
            if b == b'\n' {
                m += 1;
                continue;
            }
            acc.push(b);
        }

        if !input.ends_with('\n') {
            m += 1;
        }

        let n = (input.len() - m + 1) / m;

        Self {
            inner: Array::from_vec(acc).into_shape((n, m)).unwrap(),
        }
    }
}

impl<T> Map<T>
where
    T: Copy,
{
    pub fn from_slices(map: &[&[T]]) -> Self {
        let (n, m) = (map.len(), map[0].len());
        Self {
            inner: Array::from_iter(map.iter().flat_map(|row| row.iter()).copied())
                .into_shape((n, m))
                .unwrap(),
        }
    }

    pub fn from_vecs(map: Vec<Vec<T>>) -> Self {
        let (n, m) = (map.len(), map[0].len());
        Self {
            inner: Array::from_iter(map.iter().flat_map(|row| row.iter()).copied())
                .into_shape((n, m))
                .unwrap(),
        }
    }
}

impl<T> Map<T> {
    pub const D4: Dirs = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
    pub const D8: Dirs = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    pub fn dims(&self) -> (usize, usize) {
        self.inner.dim()
    }

    pub fn height(&self) -> usize {
        self.inner.dim().0
    }

    pub fn width(&self) -> usize {
        self.inner.dim().1
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.inner[(i, j)]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.inner[(i, j)]
    }

    pub fn get_checked(&self, i: usize, j: usize) -> Option<&T> {
        self.inner.get((i, j))
    }

    pub fn geti<I>(&self, pos: (I, I)) -> Option<&T>
    where
        I: num::NumCast,
    {
        let i = num::NumCast::from(pos.0)?;
        let j = num::NumCast::from(pos.1)?;
        self.inner.get((i, j))
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        *self.get_mut(i, j) = value;
    }

    pub fn replace(&mut self, i: usize, j: usize, value: T) -> T {
        std::mem::replace(self.get_mut(i, j), value)
    }

    pub fn fill(&mut self, elem: T)
    where
        T: Clone,
    {
        self.inner.fill(elem);
    }

    pub fn row(&self, i: usize) -> &[T] {
        self.row_checked(i).unwrap()
    }

    pub fn row_checked(&self, i: usize) -> Option<&[T]> {
        self.inner.row(i).to_slice()
    }

    pub fn rows(&self) -> impl Iterator<Item = (usize, &[T])> {
        self.inner
            .axis_iter(Axis(0))
            .map(|row| row.to_slice().unwrap())
            .enumerate()
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.inner.indexed_iter().map(|(pos, _)| pos)
    }

    pub fn cells(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.inner.indexed_iter()
    }

    pub fn cellsi(&self) -> impl Iterator<Item = ((i32, i32), &T)> {
        self.inner
            .indexed_iter()
            .map(|(pos, val)| ((pos.0 as i32, pos.1 as i32), val))
    }

    pub fn neighs4(&self, i: usize, j: usize) -> Neighbours<T> {
        Neighbours {
            map: self,
            pos: (i, j),
            dirs: Self::D4,
            k: 0,
        }
    }

    pub fn neighs4i(&self, pos: (i32, i32)) -> NeighboursI<T> {
        NeighboursI {
            map: self,
            pos,
            dirs: Self::D4,
            k: 0,
        }
    }

    pub fn neighs8(&self, i: usize, j: usize) -> Neighbours<T> {
        Neighbours {
            map: self,
            pos: (i, j),
            dirs: Self::D8,
            k: 0,
        }
    }

    pub fn is_valid(&self, i: isize, j: isize) -> bool {
        if i < 0 || i >= self.height() as isize {
            return false;
        }

        if j < 0 || j >= self.width() as isize {
            return false;
        }

        true
    }

    pub fn valid<I>(&self, pos: (I, I)) -> bool
    where
        I: num::Integer + num::Zero + num::NumCast,
    {
        if pos.0 < I::zero() || pos.0 >= I::from(self.height()).unwrap() {
            return false;
        }

        if pos.1 < I::zero() || pos.1 >= I::from(self.width()).unwrap() {
            return false;
        }

        true
    }

    pub fn on_edge(&self, i: usize, j: usize) -> bool {
        if i == 0 || i == self.height() - 1 {
            return true;
        }

        if j == 0 || j == self.width() - 1 {
            return true;
        }

        false
    }

    pub fn rel_move(&self, pos: (usize, usize), d: (isize, isize)) -> Option<(usize, usize)> {
        let (i, j) = (pos.0 as isize + d.0, pos.1 as isize + d.1);

        if !self.is_valid(i, j) {
            return None;
        }

        Some((i as usize, j as usize))
    }

    pub fn find<F>(&self, predicate: F) -> Option<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        self.cells()
            .find_map(|(pos, cell)| predicate(cell).then_some(pos))
    }

    pub fn findi<F>(&self, predicate: F) -> Option<(i32, i32)>
    where
        F: Fn(&T) -> bool,
    {
        self.cellsi()
            .find_map(|(pos, cell)| predicate(cell).then_some(pos))
    }

    pub fn print(&self)
    where
        T: Debug,
    {
        for (_, row) in self.rows() {
            println!("{:?}", row);
        }
    }
}

impl Map<u8> {
    pub fn print_text(&self) {
        for (_, row) in self.rows() {
            println!("{}", String::from_utf8_lossy(row));
        }
    }
}

impl<T> Index<(i32, i32)> for Map<T> {
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        self.get(index.0.try_into().unwrap(), index.1.try_into().unwrap())
    }
}

impl<T> IndexMut<(i32, i32)> for Map<T> {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        self.get_mut(index.0.try_into().unwrap(), index.1.try_into().unwrap())
    }
}

impl<T> Index<(usize, usize)> for Map<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<T> IndexMut<(usize, usize)> for Map<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
    }
}

pub struct Neighbours<'a, T> {
    map: &'a Map<T>,
    pos: (usize, usize),
    dirs: Dirs,
    k: usize,
}

impl<T> Iterator for Neighbours<'_, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (di, dj) = self.dirs.get(self.k)?;
        self.k += 1;

        let (i, j) = (self.pos.0 as i32 + di, self.pos.1 as i32 + dj);

        if !self.map.valid((i, j)) {
            return self.next();
        }

        Some((i as usize, j as usize))
    }
}

#[derive(Clone)]
pub struct NeighboursI<'a, T> {
    map: &'a Map<T>,
    pos: (i32, i32),
    dirs: Dirs,
    k: usize,
}

impl<T> Iterator for NeighboursI<'_, T> {
    type Item = ((i32, i32), (i32, i32));

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.dirs.get(self.k)?;
        self.k += 1;

        let next = (self.pos.0 + dir.0, self.pos.1 + dir.1);

        if !self.map.valid(next) {
            return self.next();
        }

        Some((*dir, next))
    }
}
