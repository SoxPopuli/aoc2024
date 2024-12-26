use std::{
    collections::HashSet,
    hash::Hash,
    iter::{from_fn, FromFn},
};

#[derive(Debug)]
pub struct Unique<I: Iterator> {
    seen: HashSet<I::Item>,
    iter: I,
}
impl<T> Iterator for Unique<T>
where
    T: Iterator,
    T::Item: Hash + Eq + Clone,
{
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(next) => {
                    if self.seen.contains(&next) {
                        continue;
                    } else {
                        self.seen.insert(next.clone());
                        return Some(next);
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}
pub trait UniqueIter: Iterator + Sized {
    fn unique(self) -> Unique<Self> {
        Unique {
            iter: self,
            seen: HashSet::new(),
        }
    }
}
impl<T> UniqueIter for T
where
    T: Iterator,
    T::Item: Hash + Eq + Clone,
{
}

pub fn pairs_iter<T>(
    mut iter: impl Iterator<Item = T>,
) -> FromFn<impl FnMut() -> Option<(T, Option<T>)>> {
    from_fn(move || iter.next().map(|a| (a, iter.next())))
}

pub trait PairsIter: Iterator + Sized {
    fn pairs(self) -> FromFn<impl FnMut() -> Option<(Self::Item, Option<Self::Item>)>> {
        pairs_iter(self)
    }
}
impl<T: Iterator> PairsIter for T {}
