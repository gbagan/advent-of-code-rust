use crate::util::coord::Coord;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<T>,
}

impl<T: Copy> Grid<T> {
    #[inline]
    pub fn contains(&self, c: Coord<i32>) -> bool {
        c.x >= 0 && c.x < (self.width as i32) && c.y >= 0 && c.y < (self.height as i32)
    }

    pub fn new(width: usize, height: usize, init_value: T) -> Grid<T> {
        let vec = vec![init_value; width * height];
        Grid { width, height, vec }
    }

    pub fn map<A, F>(&self, f: F) -> Grid<A> where F: FnMut(&T) -> A{
        let vec = self.vec.iter().map(f).collect();
        Grid {
            width: self.width,
            height: self.height,
            vec
        }
    }

    pub fn map_with_indices<F,U>(&self, mut f: F) -> Grid<U> 
        where F: FnMut(usize, &T) -> U
    {
        let mut vec = Vec::with_capacity(self.width * self.height);
        for (i, v) in self.vec.iter().enumerate() {
            vec.push(f(i, v));
        }

        Grid {
            width: self.width,
            height: self.height,
            vec,
        }
    }

    pub fn map_with_positions<F,U>(&self, mut f: F) -> Grid<U> 
        where F: FnMut(Coord<usize>, &T) -> U
    {
        let mut vec = Vec::with_capacity(self.width * self.height);
        for (i, v) in self.vec.iter().enumerate() {
            let c = Coord::new(i % self.width, i / self.width);
            vec.push(f(c, v));
        }

        Grid {
            width: self.width,
            height: self.height,
            vec,
        }
    }

    pub fn generate<F>(width: usize, height: usize, mut f: F) -> Grid<T> 
        where F: FnMut(usize, usize) -> T
    {
        let mut vec = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                vec.push(f(x, y));
            }
        }

        Grid { width, height: width, vec }
    }

    #[inline]
    pub fn foreach_adjacent_index(&self, idx: usize, mut f: impl FnMut(usize)) {
        let x = idx % self.width;
        let y = idx / self.width;
        if x > 0 {
            f(idx - 1);
        }
        if y > 0 {
            f(idx - self.width);
        }
        if x < self.width - 1 {
            f(idx + 1);
        }
        if y < self.height - 1 {
            f(idx + self.width);
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}


impl<T> Index<Coord<i32>> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, c: Coord<i32>) -> &Self::Output {
        &self.vec[self.width * c.y as usize + c.x as usize]
    }
}

impl<T> Index<Coord<u32>> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, c: Coord<u32>) -> &Self::Output {
        &self.vec[self.width * c.y as usize + c.x as usize]
    }
}

impl<T> Index<Coord<usize>> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, c: Coord<usize>) -> &Self::Output {
        &self.vec[self.width * c.y  + c.x]
    }
}


impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, p: (i32, i32)) -> &Self::Output {
        &self.vec[self.width * p.1 as usize + p.0 as usize]
    }
}

impl<T> Index<(i64, i64)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, p: (i64, i64)) -> &Self::Output {
        &self.vec[self.width * p.1 as usize + p.0 as usize]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, p: (usize, usize)) -> &Self::Output {
        &self.vec[self.width * p.1 + p.0]
    }
}


impl<T> IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<T> IndexMut<Coord<i32>> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, c: Coord<i32>) -> &mut Self::Output {
        &mut self.vec[self.width * c.y as usize + c.x as usize]
    }
}


impl<T> IndexMut<Coord<u32>> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, c: Coord<u32>) -> &mut Self::Output {
        &mut self.vec[self.width * c.y as usize + c.x as usize]
    }
}

impl<T> IndexMut<Coord<usize>> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, c: Coord<usize>) -> &mut Self::Output {
        &mut self.vec[self.width * c.y + c.x]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: (i32, i32)) -> &mut Self::Output {
        &mut self.vec[self.width * p.1 as usize + p.0 as usize]
    }
}

impl<T> IndexMut<(i64, i64)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: (i64, i64)) -> &mut Self::Output {
        &mut self.vec[self.width * p.1 as usize + p.0 as usize]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: (usize, usize)) -> &mut Self::Output {
        &mut self.vec[self.width * p.1 + p.0]
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len();
        let height = raw.len();
        let mut vec = Vec::with_capacity(width * height);
        for slice in raw {
            debug_assert!(slice.len() == width, "Two rows have different lengths ({width} and {})", slice.len());
            vec.extend_from_slice(slice);
        }
        debug_assert!(width > 0, "Grid width must be > 0");
        debug_assert!(height > 0, "Height width must be > 0");
        Grid { width, height, vec }
    }

    pub fn parse_with_padding(input: &str, pad: u8) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len()+2;
        let height = raw.len()+2;
        let mut vec = Vec::with_capacity(width * height);
        for _ in 0..width {
            vec.push(pad)
        }
        
        for slice in raw {
            vec.push(pad);
            debug_assert!(slice.len()+2 == width, "Two rows have different lengths ({width} and {})", slice.len());
            vec.extend_from_slice(slice);
            vec.push(pad);
        }

        for _ in 0..width {
            vec.push(pad);
        }

        Grid { width, height, vec }
    }

    pub fn parse_with_padding2<const W: usize, const H: usize>(input: &str, pad: u8) -> Self {
        let input = input.as_bytes();
        let mut vec = Vec::with_capacity((W * 2) * (H * 2));
        for _ in 0..W+2 {
            vec.push(pad)
        }
        
        for i in 0..H {
            vec.push(pad);
            vec.extend_from_slice(&input[(W+1) * i.. (W+1) * i + W]);
            vec.push(pad);
        }

        for _ in 0..W+2 {
            vec.push(pad);
        }

        Grid { width: W, height: H, vec }
    }
}

impl Grid<char> {
    pub fn draw(&self) -> String {
        let mut output = self.vec
            .chunks_exact(self.width)
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\n");
        output.insert(0, '\n');
        output
    }
}

impl Grid<u8> {
    pub fn draw(&self) -> String {
        self.map(|&c| c as char).draw()
    }
}