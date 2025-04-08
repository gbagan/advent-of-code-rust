use core::hash::Hash;
use ahash::{HashSet, HashMap, HashMapExt, HashSetExt};
use std::ops::AddAssign;

use num_traits::ConstZero;

pub trait AOCIter: Iterator {
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

    fn partial_sums(self) -> Vec<Self::Item>
    where 
        Self: Sized,
        Self::Item: AddAssign + ConstZero + Copy
    {   
        let mut sum = Self::Item::ZERO;
        let mut psums = vec!(Self::Item::ZERO);
        for x in self {
            sum += x;
            psums.push(sum);
        }
        psums
    }


    fn find_duplicate(self) -> Option<(usize, usize, Self::Item)>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut seen = HashMap::new();
        for (i, x) in self.enumerate() {
            match seen.get(&x) {
                None => { let _ = seen.insert(x, i); }
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