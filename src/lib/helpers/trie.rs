use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Trie<T, const L: usize> {
    root: TrieInner<T, L>,
}

#[derive(Debug, Clone)]
pub struct TrieInner<T, const L: usize> {
    value: Option<T>,
    next: [Option<Box<TrieInner<T, L>>>; L],
}

impl<T, const L: usize> Trie<T, L> {
    pub fn new() -> Self {
        Self {
            root: TrieInner::new(),
        }
    }

    pub fn add<I, V>(&mut self, path: I, item: T)
    where
        I: IntoIterator<Item = V>,
        V: Into<usize>,
    {
        self.root.add(path.into_iter().map(|idx| idx.into()), item);
    }

    pub fn get<I, V>(&self, path: I) -> Option<&T>
    where
        I: IntoIterator<Item = V>,
        V: Into<usize>,
    {
        self.root.get(path.into_iter().map(|idx| idx.into()))
    }

    pub fn first_match<I, V>(&self, path: I) -> Option<&T>
    where
        I: IntoIterator<Item = V>,
        V: Into<usize>,
    {
        self.root
            .first_match(path.into_iter().map(|idx| idx.into()))
    }
}

impl<T, const L: usize> Default for Trie<T, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const L: usize> TrieInner<T, L> {
    pub fn new() -> Self {
        TrieInner {
            value: None,
            next: std::array::from_fn(|_| None),
        }
    }

    pub fn add(&mut self, mut path: impl Iterator<Item = usize>, item: T) {
        match path.next() {
            Some(i) => {
                self.next[i]
                    .get_or_insert(Box::new(TrieInner::new()))
                    .add(path, item);
            }

            None => {
                self.value = Some(item);
            }
        }
    }

    pub fn get(&self, mut path: impl Iterator<Item = usize>) -> Option<&T> {
        match path.next() {
            Some(i) => self.next[i].as_ref().and_then(|next| next.get(path)),
            None => self.value.as_ref(),
        }
    }

    pub fn first_match(&self, mut path: impl Iterator<Item = usize>) -> Option<&T> {
        if self.value.is_some() {
            return self.value.as_ref();
        }

        match path.next() {
            Some(i) => self.next[i]
                .as_ref()
                .and_then(|next| next.first_match(path)),
            None => self.value.as_ref(),
        }
    }
}

impl<T, const L: usize> Default for TrieInner<T, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const L: usize> Trie<T, L>
where
    T: Debug,
{
    pub fn debug(&self) {
        self.root.debug(0);
    }
}

impl<T, const L: usize> TrieInner<T, L>
where
    T: Debug,
{
    pub fn debug(&self, indent: usize) {
        let prefix = String::from(" ").repeat(indent);
        println!("{}{:?}", prefix, self.value);
        for (i, next) in self.next.iter().enumerate() {
            if let Some(next) = next {
                println!("{}> {}", prefix, i);
                next.debug(indent + 2);
            }
        }
    }
}
