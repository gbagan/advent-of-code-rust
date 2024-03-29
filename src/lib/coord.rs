use std::ops::{Add,AddAssign,Mul,Neg,Sub};
use std::iter::Sum;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub y: i64,
    pub x: i64,
}

impl Coord {
    #[inline]
    pub fn origin() -> Coord {
        Coord { x: 0, y: 0 }
    }

    #[inline]
    pub fn new(x: i64, y: i64) -> Coord {
        Coord { x: x, y: y }
    }

    pub fn manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() +  (self.y - other.y).abs()
    }

    #[inline]
    pub fn north() -> Coord {
        Coord { x: 0, y: -1 }
    }

    #[inline]
    pub fn south() -> Coord {
        Coord { x: 0, y: 1 }
    }

    #[inline]
    pub fn west() -> Coord {
        Coord { x: -1, y: 0 }
    }

    #[inline]
    pub fn east() -> Coord {
        Coord { x: 1, y: 0 }
    }

    #[inline]
    pub fn left(&self) -> Coord {
        Coord { x: self.x-1, y: self.y }
    }

    #[inline]
    pub fn right(&self) -> Coord {
        Coord { x: self.x+1, y: self.y }
    }

    #[inline]
    pub fn above(&self) -> Coord {
        Coord { x: self.x, y: self.y-1 }
    }

    #[inline]
    pub fn below(&self) -> Coord {
        Coord { x: self.x, y: self.y+1 }
    }

    #[inline]
    pub fn turn_left(&self) -> Coord {
        Coord { x: self.y, y: -self.x }
    }

    #[inline]
    pub fn turn_right(&self) -> Coord {
        Coord { x: -self.y, y: self.x }
    }

    #[inline]
    pub fn adjacent(&self) -> Vec<Coord> {
        vec!( Coord { x: self.x, y: self.y-1 },
              Coord { x: self.x, y: self.y+1 },
              Coord { x: self.x-1, y: self.y },
              Coord { x: self.x+1, y: self.y },
            )
    }

    #[inline]
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

    #[inline]
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Neg for Coord {
    type Output = Self;

    #[inline]
    fn neg (self) -> Self::Output {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i64> for Coord {
    type Output = Self;

    #[inline]
    fn mul(self, n: i64) -> Self::Output {
        Coord {
            x: n * self.x,
            y: n * self.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    #[inline]
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
        iter.fold(Self::origin(), |a, b| a + *b)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Coord3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coord3 {
    #[inline]
    pub fn origin() -> Self {
        Self { x: 0, y: 0, z: 0, }
    }

    #[inline]
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() +  (self.y - other.y).abs() +  (self.z - other.z).abs()
    }
}

impl Add for Coord3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Coord3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Coord3 {
    type Output = Self;

    #[inline]
    fn neg (self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<i64> for Coord3 {
    type Output = Self;

    #[inline]
    fn mul(self, n: i64) -> Self::Output {
        Self {
            x: n * self.x,
            y: n * self.y,
            z: n * self.z,
        }
    }
}

impl Sub for Coord3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - self.z,
        }
    }
}

impl Sum for Coord3 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::origin(), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for Coord3 {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = &'a Self>,
     {
        iter.fold(Self::origin(), |a, b| a + *b)
    }
}