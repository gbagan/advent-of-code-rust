use std::ops::{Add,AddAssign,Mul,Neg,Sub};
use std::iter::Sum;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub y: i64,
    pub x: i64,
}

impl Coord {
    pub fn origin() -> Coord {
        Coord { x: 0, y: 0 }
    }

    pub fn new(x: i64, y: i64) -> Coord {
        Coord { x: x, y: y }
    }

    pub fn north() -> Coord {
        Coord { x: 0, y: -1 }
    }

    pub fn south() -> Coord {
        Coord { x: 0, y: 1 }
    }

    pub fn west() -> Coord {
        Coord { x: -1, y: 0 }
    }

    pub fn east() -> Coord {
        Coord { x: 1, y: 0 }
    }

    pub fn left(&self) -> Coord {
        Coord { x: self.x-1, y: self.y }
    }

    pub fn right(&self) -> Coord {
        Coord { x: self.x+1, y: self.y }
    }

    pub fn above(&self) -> Coord {
        Coord { x: self.x, y: self.y-1 }
    }

    pub fn below(&self) -> Coord {
        Coord { x: self.x, y: self.y+1 }
    }

    pub fn turn_left(&self) -> Coord {
        Coord { x: self.y, y: -self.x }
    }

    pub fn turn_right(&self) -> Coord {
        Coord { x: -self.y, y: self.x }
    }

    pub fn adjacent(&self) -> Vec<Coord> {
        vec!( Coord { x: self.x, y: self.y-1 },
              Coord { x: self.x, y: self.y+1 },
              Coord { x: self.x-1, y: self.y },
              Coord { x: self.x+1, y: self.y },
            )
    }

    pub fn surrounding(&self) -> Vec<Coord> {
        vec!( 
              Coord { x: self.x, y: self.y-1 },
              Coord { x: self.x, y: self.y+1 },
              Coord { x: self.x-1, y: self.y },
              Coord { x: self.x+1, y: self.y },
              Coord { x: self.x+1, y: self.y+1 },
              Coord { x: self.x+1, y: self.y-1 },
              Coord { x: self.x-1, y: self.y+1 },
              Coord { x: self.x-1, y: self.y-1 },
            )
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Neg for Coord {
    type Output = Self;

    fn neg (self) -> Self::Output {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i64> for Coord {
    type Output = Self;

    fn mul(self, n: i64) -> Self::Output {
        Coord {
            x: n * self.x,
            y: n * self.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sum for Coord {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::origin(), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for Coord {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = &'a Self>,
     {
        iter.fold(Coord::origin(), |a, b| a + *b)
    }
}