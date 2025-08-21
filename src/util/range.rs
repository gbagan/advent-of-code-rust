use num_traits::{ConstOne, Num};

#[derive (PartialEq, Eq, Copy, Clone, Debug)]
pub struct Range<A> {
    pub start: A,
    pub end: A,
}

impl<A: Num + Copy + Ord + ConstOne> Range<A> {
    #[inline]
    pub fn new(lower: A, upper: A) -> Self {
        Range { start: lower, end: upper }
    }
    
    #[inline]
    pub fn contains(&self, v: A) -> bool {
        v >= self.start && v <= self.end
    }

    #[inline]
    pub fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    #[inline]
    pub fn length(&self) -> A {
        self.end - self.start + A::ONE
    }

    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.end >= other.start && other.end >= self.start
    }

    #[inline]
    pub fn shift(&self, v: A) -> Self {
        Range { start: self.start + v, end: self.end + v }       
    }

    pub fn union(&self, other: &Self) -> Option<Self> {
        if self.start.max(other.start) <= self.end.min(other.end) + A::ONE {
            Some(Range::new(self.start.min(other.start), self.end.max(other.end)))
        } else {
            None
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(Range::new(self.start.max(other.start), self.end.min(other.end)))
        }
    }

    pub fn disjoint_union<I>(it: I) -> Vec<Range<A>> 
    where 
        I: IntoIterator<Item=Range<A>>,
        //A: Integer + Ord + Copy + ConstOne
    {
        let mut ranges: Vec<_> = it.into_iter().collect();
        ranges.sort_unstable_by_key(|r| r.start); 
        let mut it = ranges.iter();
        let mut output = vec!();
        if let Some(&first) = it.next() {
            let mut previous = first;
            for current in it {
                if let Some(union) = previous.union(current) {
                    previous = union;
                } else {
                    output.push(previous);
                    previous = *current;
                }
            }
            output.push(previous);
        }
        output
    }
}



pub trait RangeIter<A>: Iterator<Item=Range<A>> 
    where
        Self: Sized,
        A: Num + Ord + Copy + ConstOne {
    fn disjoint_union(&mut self) -> Vec<Range<A>> {
        let mut ranges: Vec<_> = self.collect();
        ranges.sort_unstable_by_key(|r| r.start); 
        let mut it = ranges.iter();
        let mut output = vec!();
        if let Some(&first) = it.next() {
            let mut previous = first;
            for current in it {
                if let Some(union) = previous.union(current) {
                    previous = union;
                } else {
                    output.push(previous);
                    previous = *current;
                }
            }
            output.push(previous);
        }
        output
    }
}

impl<A: Ord+Num+Copy+ConstOne, I: Iterator<Item=Range<A>>> RangeIter<A> for I {}

#[test]
fn length_test () {
    let range = Range::new(3, 4);
    assert_eq!(range.length(), 2);
}

#[test]
fn disjoint_union_test () {
    let ranges = vec!(Range::new(3, 4), Range::new(0, 1), Range::new(6, 7), Range::new(1, 2));
    let expected = vec!(Range::new(0, 4), Range::new(6, 7));
    assert_eq!(Range::disjoint_union(ranges), expected);
}