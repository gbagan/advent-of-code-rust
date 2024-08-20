use crate::util::coord::Coord;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    vec: Vec<T>,
}

impl<T> Grid<T> {
    #[inline]
    pub fn contains(&self, c: Coord) -> bool {
        c.x >= 0 && c.x < (self.width as i64) && c.y >= 0 && c.y < (self.height as i64)
    }
    pub fn map_with_indices<U, F>(&self, mut f: F) -> Grid<U> 
        where F: FnMut(Coord, &T) -> U
    {
        let mut vec = Vec::with_capacity(self.width * self.height);
        for (i, v) in self.vec.iter().enumerate() {
            let c = Coord::new((i % self.width) as i64, (i / self.width) as i64);
            vec.push(f(c, v));
        }

        Grid {
            width: self.width,
            height: self.height,
            vec,
        }
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, c: Coord) -> &Self::Output {
        &self.vec[self.width * c.y as usize + c.x as usize]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, c: Coord) -> &mut Self::Output {
        &mut self.vec[self.width * c.y as usize + c.x as usize]
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len();
        let height = raw.len();
        let mut vec = Vec::with_capacity(width * height);
        raw.iter().for_each(|slice| vec.extend_from_slice(slice));
        Grid { width, height, vec }
    }
}
