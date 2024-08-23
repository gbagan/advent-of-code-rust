use core::hash::Hash;
use std::collections::HashSet;
use std::collections::HashMap;

pub trait AOCIter: Iterator {
    #[inline]
    fn count_by<P>(self, mut predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        let mut counter = 0;
        for x in self {
            if predicate(x) {
                counter += 1;
            }
        }
        counter
    }

    fn all_distinct(self) -> bool 
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        let mut seen = HashSet::new();
        for x in self {
            if seen.contains(&x) {
                return false
            }
            seen.insert(x);
        }
        true
    }

    fn find_duplicate(self) -> Option<(usize, usize, Self::Item)>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut seen = HashMap::new();
        for (i, x) in self.enumerate() {
            match seen.get(&x) {
                None => { seen.insert(x, i); () }
                Some(&j) => return Some((j, i, x)),
            }
        }
        None
    }

    fn find_duplicate_on<A,F>(self, f: F) -> Option<(usize, usize, Self::Item)>
    where
        Self: Sized,
        A: Eq + Hash,
        F: Fn(&Self::Item) -> A
    {
        let mut seen = HashMap::new();
        for (i, x) in self.enumerate() {
            if let Some(j) = seen.insert(f(&x), i) {
                return Some((j, i, x));
            }
        }
        None
    }
}

impl<I: Iterator> AOCIter for I {}