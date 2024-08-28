use std::ops::{BitAnd, BitOr};

#[derive (PartialEq, Eq, Copy, Clone, Debug)]
pub struct Range {
    pub lower: i64,
    pub upper: i64,
}

impl Range {
    #[inline]
    pub fn new(lower: i64, upper: i64) -> Self {
        Range { lower, upper }
    }
    
    #[inline]
    pub fn contains(&self, v: i64) -> bool {
        v >= self.lower && v <= self.upper
    }

    #[inline]
    pub fn length(&self) -> i64 {
        self.upper + 1 -  self.lower
    }

    #[inline]
    pub fn shift(&self, v: i64) -> Self {
        Range { lower: self.lower + v, upper: self.upper + v }       
    }

    pub fn disjoint_union(ranges: &[Range]) -> Vec<Range> {
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

impl BitAnd for Range {
    type Output = Option<Self>;
    fn bitand(self, other: Self) -> Self::Output {
        if self.upper < other.lower || other.upper < self.lower {
            None
        } else {
            Some(Range::new(self.lower.max(other.lower), self.upper.min(other.upper)))
        }
    }
}

impl BitAnd for &Range {
    type Output = Option<Range>;
    fn bitand(self, other: Self) -> Self::Output {
        if self.upper < other.lower || other.upper < self.lower {
            None
        } else {
            Some(Range::new(self.lower.max(other.lower), self.upper.min(other.upper)))
        }
    }
}

impl BitOr for Range {
    type Output = Option<Range>;
    fn bitor(self, other: Self) -> Self::Output {
        if self.lower.max(other.lower) <= self.upper.min(other.upper) + 1 {
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