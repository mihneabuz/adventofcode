use itertools::Itertools;

pub struct Map<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Map<T>
where
    T: Default + Clone,
{
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            inner: vec![vec![T::default(); m]; n],
        }
    }
}

impl<T> Map<T>
where
    T: Clone,
{
    pub fn from_slices(map: &[&[T]]) -> Self {
        Self {
            inner: Vec::from_iter(map.iter().map(|s| s.to_vec())),
        }
    }
}

impl<T> Map<T> {
    const D4: Dirs = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
    const D8: Dirs = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    pub fn from_vecs(map: Vec<Vec<T>>) -> Self {
        Self { inner: map }
    }

    pub fn from_iterator<R, C>(iter: R) -> Self
    where
        R: Iterator<Item = C>,
        C: Iterator<Item = T>,
    {
        Self {
            inner: iter.map(|r| r.collect_vec()).collect_vec(),
        }
    }

    pub fn height(&self) -> usize {
        self.inner.len()
    }

    pub fn width(&self) -> usize {
        self.inner[0].len()
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.inner[i][j]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &T {
        &mut self.inner[i][j]
    }

    pub fn get_checked(&self, i: usize, j: usize) -> Option<&T> {
        self.inner.get(i)?.get(j)
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.inner[i][j] = value;
    }

    pub fn row(&self, i: usize) -> &[T] {
        &self.inner[i]
    }

    pub fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self.inner[i]
    }

    pub fn row_checked(&self, i: usize) -> Option<&[T]> {
        self.inner.get(i).map(|v| v.as_slice())
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        let (n, m) = (self.height(), self.width());
        (0..n).flat_map(move |i| (0..m).map(move |j| (i, j)))
    }

    pub fn cells(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        let (n, m) = (self.height(), self.width());
        (0..n).flat_map(move |i| (0..m).map(move |j| ((i, j), self.get(i, j))))
    }

    pub fn neighs4(&self, i: usize, j: usize) -> Neighbours<T> {
        Neighbours {
            map: self,
            pos: (i, j),
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
}

type Dirs = &'static [(isize, isize)];

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

        let (i, j) = (self.pos.0 as isize + di, self.pos.1 as isize + dj);

        if i < 0 || i >= self.map.height() as isize {
            return self.next();
        }

        if j < 0 || j >= self.map.width() as isize {
            return self.next();
        }

        Some((i as usize, j as usize))
    }
}
