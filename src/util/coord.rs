use std::ops::{Add,AddAssign,Mul,Neg,Sub, SubAssign};
use std::iter::Sum;
use num_integer::Integer;
use num_traits::{ConstOne, Signed};

fn distance<A: Integer + Ord>(a: A, b: A) -> A {
    if a >= b { a - b } else { b - a} 
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Coord<A> {
    pub y: A,
    pub x: A,
}

impl<A: Copy + Integer + ConstOne> Coord<A> {
    #[inline]
    pub fn new(x: A, y: A) -> Coord<A> {
        Coord { x, y }
    }

    pub fn manhattan(&self, other: Self) -> A {
        distance(self.x, other.x) +  distance(self.y, other.y)
    }

    #[inline]
    pub fn left(&self) -> Self {
        Coord { x: self.x-A::ONE, y: self.y }
    }

    #[inline]
    pub fn right(&self) -> Self {
        Coord { x: self.x+A::ONE, y: self.y }
    }

    #[inline]
    pub fn above(&self) -> Self {
        Coord { x: self.x, y: self.y-A::ONE }
    }

    #[inline]
    pub fn below(&self) -> Self {
        Coord { x: self.x, y: self.y+A::ONE }
    }

    #[inline]
    pub fn adjacent4(&self) -> [Self; 4] {
        [self.left(), self.right(), self.above(), self.below()]
    }
}

impl<A: Copy + Ord + Integer + Signed + ConstOne> Coord<A> {
    #[inline]
    pub fn turn_left(&self) -> Self {
        Coord { x: self.y, y: -self.x }
    }

    #[inline]
    pub fn turn_right(&self) -> Self {
        Coord { x: -self.y, y: self.x }
    }

    #[inline]
    pub fn adjacent8(&self) -> [Self; 8] {
        let left = self.left();
        let right = self.right();
        [left, right, self.above(), self.below(), left.above(), left.below(), right.above(), right.below()]
    }
}

impl<A: Integer> Add for Coord<A> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<A: Integer + Copy> AddAssign for Coord<A> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<A: Integer + Copy> SubAssign for Coord<A> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<A: Signed> Neg for Coord<A> {
    type Output = Self;

    #[inline]
    fn neg (self) -> Self::Output {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<A: Integer + Copy> Mul<A> for Coord<A> {
    type Output = Self;

    #[inline]
    fn mul(self, n: A) -> Self::Output {
        Coord {
            x: n * self.x,
            y: n * self.y,
        }
    }
}

impl<A: Integer>Sub for Coord<A> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/* 
impl<A> Sum for Coord<A> {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::ORIGIN, |a, b| a + b)
    }
}

impl<'a, A> Sum<&'a Self> for Coord<A> {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = &'a Self>,
     {
        iter.fold(Self::ORIGIN, |a, b| a + *b)
    }
}
*/
 
macro_rules! constant {
    ($($t:ty)*) => ($(
        impl Coord<$t> {
            pub const ORIGIN: Coord<$t> = Coord { x: 0, y: 0 };
            pub const NORTH: Coord<$t> = Coord { x: 0, y: -1 };
            pub const SOUTH: Coord<$t> = Coord { x: 0, y: 1 };
            pub const WEST: Coord<$t> = Coord { x: -1, y: 0 };
            pub const EAST: Coord<$t> = Coord { x: 1, y: 0 };
        }
    )*)
}

constant!(i16 i32 i64 i128);




#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Coord3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord3 {
    pub const ORIGIN: Coord3 = Self { x: 0, y: 0, z: 0 };

    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn manhattan(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() +  (self.y - other.y).abs() +  (self.z - other.z).abs()
    }

    #[inline]
    pub fn euclidean(&self, other: &Self) -> i32 {
        let Self { x, y, z } = *self - *other;
        x * x + y * y + z * z
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

impl Mul<i32> for Coord3 {
    type Output = Self;

    #[inline]
    fn mul(self, n: i32) -> Self::Output {
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
            z: self.z - other.z,
        }
    }
}

impl Sum for Coord3 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self::ORIGIN, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for Coord3 {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = &'a Self>,
     {
        iter.fold(Self::ORIGIN, |a, b| a + *b)
    }
}