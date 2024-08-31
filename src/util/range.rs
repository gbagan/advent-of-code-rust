use std::ops::{BitAnd, BitOr};

use num_integer::Integer;
use num_traits::ConstOne;

#[derive (PartialEq, Eq, Copy, Clone, Debug)]
pub struct Range<A> {
    pub lower: A,
    pub upper: A,
}

impl<A: Integer + Copy + Ord + ConstOne> Range<A> {
    #[inline]
    pub fn new(lower: A, upper: A) -> Self {
        Range { lower, upper }
    }
    
    #[inline]
    pub fn contains(&self, v: A) -> bool {
        v >= self.lower && v <= self.upper
    }

    #[inline]
    pub fn fully_contains(&self, other: &Self) -> bool {
        self.lower <= other.lower && other.upper <= self.upper
    }

    #[inline]
    pub fn length(&self) -> A {
        self.upper - self.lower + A::ONE
    }

    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.upper >= other.lower && other.upper >= self.lower
    }

    #[inline]
    pub fn shift(&self, v: A) -> Self {
        Range { lower: self.lower + v, upper: self.upper + v }       
    }

    pub fn disjoint_union(ranges: &[Range<A>]) -> Vec<Range<A>> {
        let mut ranges = ranges.to_vec();
        ranges.sort_by_key(|r| r.lower);
        let mut it = ranges.iter();
        let mut res = vec!();
        if let Some(&first) = it.next() {
            let mut prev = first;
            for &current in it {
                if let Some(union) = prev | current {
                    prev = union;
                } else {
                    res.push(prev);
                    prev = current;
                }
            }
            res.push(prev);
        }
        res
    }
}

impl<A: Integer + Ord + Copy + ConstOne> BitAnd for Range<A> {
    type Output = Option<Self>;
    fn bitand(self, other: Self) -> Self::Output {
        if self.upper < other.lower || other.upper < self.lower {
            None
        } else {
            Some(Range::new(self.lower.max(other.lower), self.upper.min(other.upper)))
        }
    }
}

impl<A: Integer + Ord + Copy + ConstOne> BitAnd for &Range<A> {
    type Output = Option<Range<A>>;
    fn bitand(self, other: Self) -> Self::Output {
        if self.upper < other.lower || other.upper < self.lower {
            None
        } else {
            Some(Range::new(self.lower.max(other.lower), self.upper.min(other.upper)))
        }
    }
}

impl<A: Integer + Ord + Copy + ConstOne> BitOr for Range<A> {
    type Output = Option<Range<A>>;
    fn bitor(self, other: Self) -> Self::Output {
        if self.lower.max(other.lower) <= self.upper.min(other.upper) + A::ONE {
            Some(Range::new(self.lower.min(other.lower), self.upper.max(other.upper)))
        } else {
            None
        }
    }
}

#[test]
fn length_test () {
    let range = Range::new(3, 4);
    assert_eq!(range.length(), 2);
}

#[test]
fn disjoint_union_test () {
    let ranges = vec!(Range::new(3, 4), Range::new(0, 1), Range::new(6, 7), Range::new(1, 2));
    let res = vec!(Range::new(0, 4), Range::new(6, 7));
    assert_eq!(Range::disjoint_union(&ranges), res);
}