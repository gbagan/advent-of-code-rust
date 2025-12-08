use arrayvec::ArrayVec;
use crate::util::{coord::*, iter::*, parser::*};

type Point = Coord3<i64>;

pub fn solve(input: &str) -> (usize, i64) {
    let points: Vec<_> = input
        .iter_unsigned()
        .tuples()
        .map(|(x, y, z)| Point::new(x, y, z))
        .collect();

    let xmin = points.iter().map(|p| p.x).min().unwrap();
    let xmax =  points.iter().map(|p| p.x).max().unwrap();
    let ymin =  points.iter().map(|p| p.y).min().unwrap();
    let ymax =  points.iter().map(|p| p.y).max().unwrap();
    let zmin =  points.iter().map(|p| p.z).min().unwrap();
    let zmax =  points.iter().map(|p| p.z).max().unwrap();

    let mut partitions: [ArrayVec<usize, 10>; 8*8*8] = std::array::from_fn(|_| ArrayVec::new());
    for (i, point) in points.iter().enumerate() {
        let x_idx = (8 * (point.x - xmin) / (xmax - xmin + 1)) as usize;
        let y_idx = (8 * (point.y - ymin) / (ymax - ymin + 1)) as usize;
        let z_idx = (8 * (point.z - zmin) / (zmax - zmin + 1)) as usize;
        let idx = 64 * x_idx + 8 * y_idx + z_idx;
        partitions[idx].push(i);
    }

    let mut edges = Vec::new();

    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                let ijk = 64*i+8*j+k;
                for i2 in i.max(1)-1..(i+2).min(8) {
                    for j2 in j.max(1)-1..(j+2).min(8) {
                        for k2 in k.max(1)-1..(k+2).min(8) {
                            let ijk2 = 64*i2+8*j2+k2;
                            for &idx1 in &partitions[ijk] {
                                for &idx2 in &partitions[ijk2] {
                                    if idx1 < idx2 {
                                        let dist = points[idx1].euclidean(&points[idx2]);
                                        edges.push((idx1, idx2, dist));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    edges.sort_unstable_by_key(|p| p.2);
    //edges.select_nth_unstable_by_key(1000, |p| p.2);

    let mut uf = UnionFind::new(points.len());
    let mut component_count = points.len();

    for &(v1, v2, _) in &edges[..1000] {
        component_count -= uf.union(v1, v2) as usize;
    }

    let mut sizes: Vec<_> = (0..points.len())
        .filter(|&i| uf.parent[i] == i)
        .map(|i| uf.size[i])
        .collect();

    sizes.sort_unstable();
    let p1 = sizes.iter().rev().take(3).product();

    let mut p2 = 0;

    for &(v1, v2, _) in &edges {
        component_count -= uf.union(v1, v2) as usize;
        if component_count == 1 {
            p2 = points[v1].x * points[v2].x;
            break;
        }
    }

    (p1, p2)
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }
}