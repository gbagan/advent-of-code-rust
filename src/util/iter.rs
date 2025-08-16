use core::hash::Hash;
use ahash::{HashSet, HashMap, HashMapExt, HashSetExt};
use std::{marker::PhantomData, ops::AddAssign};

use num_traits::ConstZero;

pub trait AOCIter: Iterator+Sized {
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

    fn minmax(mut self) -> Option<(Self::Item, Self::Item)> where Self::Item: Ord+Copy {
        let mut min = self.next()?;
        let mut max = min;
        for x in self {
            min = min.min(x);
            max = max.max(x);
        }
        Some((min, max))
    }

    fn with_putback(self) -> WithPutBack<Self::Item, Self> {
        WithPutBack { iter: self, back: Vec::new() }
    }

    #[inline]
    fn next_tuple<T>(&mut self) -> Option<T> where T: Tuple<Item = Self::Item> {
        T::from_iterator(self )
    }
    
    fn tuples<T>(self) -> Tuples<Self::Item, Self, T> where T: Tuple<Item=Self::Item> {
        Tuples { iter: self, phantom: PhantomData }
    }

    fn tuple_windows<T>(self) -> TupleWindows<Self::Item, Self, T> where T: WindowTuple<Item=Self::Item> {
        TupleWindows { iter: self, prev: None, phantom: PhantomData }
    }

}

impl<I: Iterator> AOCIter for I {}



pub trait Tuple: Sized {
    type Item;
    fn from_iterator(it: &mut impl Iterator<Item=Self::Item>) -> Option<Self>;
}

impl<A> Tuple for (A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<(A, A)> {
        let a = it.next()?;
        let b = it.next()?;
        Some((a, b))
    }
}

impl<A> Tuple for (A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<(A, A, A)> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        Some((a, b, c))
    }
}

impl<A> Tuple for (A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<(A, A, A, A)> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        Some((a, b, c, d))
    }
}

impl<A> Tuple for (A, A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<Self> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        let e = it.next()?;
        Some((a, b, c, d, e))
    }
}


// 6
impl<A> Tuple for (A, A, A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<Self> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        let e = it.next()?;
        let f = it.next()?;
        Some((a, b, c, d, e, f))
    }
}


// 7
impl<A> Tuple for (A, A, A, A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<Self> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        let e = it.next()?;
        let f = it.next()?;
        let g = it.next()?;
        Some((a, b, c, d, e, f, g))
    }
}

// 8
impl<A> Tuple for (A, A, A, A, A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<Self> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        let e = it.next()?;
        let f = it.next()?;
        let g = it.next()?;
        let h = it.next()?;
        Some((a, b, c, d, e, f, g, h))
    }
}

// 9
impl<A> Tuple for (A, A, A, A, A, A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<Self> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        let e = it.next()?;
        let f = it.next()?;
        let g = it.next()?;
        let h = it.next()?;
        let i = it.next()?;
        Some((a, b, c, d, e, f, g, h, i))
    }
}

// 10
impl<A> Tuple for (A, A, A, A, A, A, A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<Self> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        let e = it.next()?;
        let f = it.next()?;
        let g = it.next()?;
        let h = it.next()?;
        let i = it.next()?;
        let j = it.next()?;
        Some((a, b, c, d, e, f, g, h, i, j))
    }
}

// 11
impl<A> Tuple for (A, A, A, A, A, A, A, A, A, A, A) {
    type Item = A;

    fn from_iterator(it: &mut impl Iterator<Item=A>) -> Option<Self> {
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        let d = it.next()?;
        let e = it.next()?;
        let f = it.next()?;
        let g = it.next()?;
        let h = it.next()?;
        let i = it.next()?;
        let j = it.next()?;
        let k = it.next()?;
        Some((a, b, c, d, e, f, g, h, i, j, k))
    }
}


pub trait WindowTuple: Tuple {
    type Item;
    type Prev: Clone;

    fn window_from_enumerator(prev: &mut Option<Self::Prev>, it: &mut impl Iterator<Item=<Self as WindowTuple>::Item>) -> Option<Self>;
}

impl<A> WindowTuple for (A, A) where A: Clone {
    type Item = A;
    type Prev = A;

    fn window_from_enumerator(prev: &mut Option<Self::Prev>, it: &mut impl Iterator<Item=<Self as WindowTuple>::Item>) -> Option<Self> {
        if let Some(a) = prev.clone() {
            let b = it.next()?;
            *prev = Some(b.clone());
            Some((a, b))
        } else {
            let (a, b) = Self::from_iterator(it)?;
            *prev = Some(b.clone());
            Some((a, b))
        }
    }
}
pub struct Tuples<A, I: Iterator<Item=A>, T: Tuple<Item=A>> {
    iter: I,
    phantom: PhantomData<T>
}


impl<A, I, T> Iterator for Tuples<A, I, T> where I: Iterator<Item=A>, T: Tuple<Item=A> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        T::from_iterator(&mut self.iter)
    }
}

pub struct TupleWindows<A, I: Iterator<Item=A>, T: WindowTuple<Item=A>> {
    iter: I,
    prev: Option<T::Prev>,
    phantom: PhantomData<T>
}

impl<A, I, T> Iterator for TupleWindows<A, I, T> where I: Iterator<Item=A>, T: WindowTuple<Item=A> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        T::window_from_enumerator(&mut self.prev, &mut self.iter)
    }
}

#[test]
fn tuples_test() {
    let mut it = (0..5).tuples();
    let (a, b) = it.next().unwrap();
    assert_eq!(a, 0);
    assert_eq!(b, 1);
}




pub struct Product<I, J> where I: Iterator {
    iter1: I,
    current_a: Option<Option<I::Item>>,
    iter2: J,
    iter2_orig: J,
}


pub struct WithPutBack<A, I> {
    back: Vec<A>,
    iter: I,
}

impl<A, I> WithPutBack<A, I> where I: Iterator<Item=A> {
    pub fn put_back(&mut self, x: A) {
        self.back.push(x);
    }
}

impl<A, I> Iterator for WithPutBack<A, I> where
    I: Iterator<Item=A>
{
    type Item = A;


    fn next(&mut self) -> Option<A> {
        self.back.pop().or_else(|| self.iter.next())

    }
}




pub fn cartesian_product<I, J>(iter1: I, iter2: J) -> Product<I, J> where
    I: Iterator,
    J: Clone + Iterator,
    I::Item: Clone,
{
    Product { iter1, current_a: None, iter2: iter2.clone(), iter2_orig: iter2, }

}

impl<I, J> Iterator for Product<I, J> where
    I: Iterator,
    J: Clone + Iterator,
    I::Item: Clone 
{
    type Item = (I::Item, J::Item);


    fn next(&mut self) -> Option<Self::Item> {

        let Self { iter1, current_a, iter2, iter2_orig} = self;

        let b = match iter2.next() {
            Some(x) => x,
            None => {
                *iter2 = iter2_orig.clone();
                match iter2.next() {
                    None => return None,
                    Some(x) => {
                        *current_a = Some(iter1.next());
                        x
                    }
                }
            }
        };

        current_a.get_or_insert_with(|| iter1.next()).as_ref().map(|a| (a.clone(), b))
    }
}


pub fn iterate<A, F>(state: A, f: F) -> Iterate<A, F> 
where F: FnMut(&A) -> A
{
    Iterate { state, f }
}

pub struct Iterate<A, F> {
    state: A,
    f: F,
}


impl<A, F> Iterator for Iterate<A, F> where F: FnMut(&A) -> A, {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next_state = (self.f)(&self.state);
        Some(std::mem::replace(&mut self.state, next_state))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }

}