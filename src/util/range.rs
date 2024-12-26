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

    pub fn union(&self, other: &Self) -> Option<Self> {
        if self.lower.max(other.lower) <= self.upper.min(other.upper) + A::ONE {
            Some(Range::new(self.lower.min(other.lower), self.upper.max(other.upper)))
        } else {
            None
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.upper < other.lower || other.upper < self.lower {
            None
        } else {
            Some(Range::new(self.lower.max(other.lower), self.upper.min(other.upper)))
        }
    }

    pub fn disjoint_union<I>(it: I) -> Vec<Range<A>> 
    where 
        I: IntoIterator<Item=Range<A>>,
        //A: Integer + Ord + Copy + ConstOne
    {
        let mut ranges: Vec<_> = it.into_iter().collect();
        ranges.sort_unstable_by_key(|r| r.lower); 
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
        A: Integer + Ord + Copy + ConstOne {
    fn disjoint_union(&mut self) -> Vec<Range<A>> {
        let mut ranges: Vec<_> = self.collect();
        ranges.sort_unstable_by_key(|r| r.lower); 
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

impl<A: Ord+Integer+Copy+ConstOne, I: Iterator<Item=Range<A>>> RangeIter<A> for I {}

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