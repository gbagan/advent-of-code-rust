#[derive (PartialEq, Eq, Clone, Debug)]
pub struct Range {
    pub lower: i64,
    pub upper: i64,
}

impl Range {
    pub fn new(lower: i64, upper: i64) -> Self {
        Range { lower, upper }
    }
    
    pub fn contains(&self, v: i64) -> bool {
        v >= self.lower && v <= self.upper
    }

    pub fn translate(&self, v: i64) -> Self {
        Range { lower: self.lower + v, upper: self.upper + v }       
    }

    pub fn intersection(&self, other: &Range) -> Option<Range> {
        if self.upper < other.lower || other.upper < self.lower {
            None
        } else {
            Some(Range::new(self.lower.max(other.lower), self.upper.min(other.upper)))
        }
    }
}