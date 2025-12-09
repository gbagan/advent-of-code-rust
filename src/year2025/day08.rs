use arrayvec::ArrayVec;
use crate::util::{coord::*, iter::*, parser::*};

type Point = Coord3<i64>;

pub fn solve(input: &str) -> (usize, i64) {
    let points: Vec<_> = input
        .iter_unsigned()
        .tuples()
        .map(|(x, y, z)| Point::new(x, y, z))
        .collect();

    let xmax = 99_999;
    let ymax = 99_999;
    let zmax = 99_999;

    let mut partitions: [ArrayVec<usize, 10>; 8*8*8] = std::array::from_fn(|_| ArrayVec::new());
    for (i, point) in points.iter().enumerate() {
        let x_idx = (8 * point.x / (xmax + 1)) as usize;
        let y_idx = (8 * point.y / (ymax + 1)) as usize;
        let z_idx = (8 * point.z / (zmax + 1)) as usize;
        let idx = 64 * x_idx + 8 * y_idx + z_idx;
        partitions[idx].push(i);
    }

    let mut edges = Vec::new();

    for i in 0usize..8 {
        for j in 0usize..8 {
            for k in 0usize..8 {
                let ijk = 64*i+8*j+k;
                for i2 in i.saturating_sub(1)..(i+2).min(8) {
                    for j2 in j.saturating_sub(1)..(j+2).min(8) {
                        for k2 in k.saturating_sub(1)..(k+2).min(8) {
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




/* 
pub fn run(input: &str) -> usize {
    let points: Vec<_> = input
        .iter_unsigned()
        .tuples()
        .map(|(x, y, z)| Point::new(x, y, z))
        .collect();

    let xmax = 99_999;
    let ymax = 99_999;
    let zmax = 99_999;

    let mut partitions: [ArrayVec<usize, 10>; 8*8*8] = std::array::from_fn(|_| ArrayVec::new());
    for (i, point) in points.iter().enumerate() {
        let x_idx = (8 * point.x / (xmax + 1)) as usize;
        let y_idx = (8 * point.y / (ymax + 1)) as usize;
        let z_idx = (8 * point.z / (zmax + 1)) as usize;
        let idx = 64 * x_idx + 8 * y_idx + z_idx;
        partitions[idx].push(i);
    }

    let mut heap = BinaryHeap { arr: [(0, 0, i64::MAX); 1000] };

    for i in 0usize..8 {
        for j in 0usize..8 {
            for k in 0usize..8 {
                let ijk = 64*i+8*j+k;
                for i2 in i.saturating_sub(1)..(i+2).min(8) {
                    for j2 in j.saturating_sub(1)..(j+2).min(8) {
                        for k2 in k.saturating_sub(1)..(k+2).min(8) {
                            let ijk2 = 64*i2+8*j2+k2;
                            for &idx1 in &partitions[ijk] {
                                for &idx2 in &partitions[ijk2] {
                                    if idx1 < idx2 {
                                        let dist = points[idx1].euclidean(&points[idx2]);
                                        heap.insert(idx1, idx2, dist);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    //println!("{:?}", &heap.arr);

    //edges.sort_unstable_by_key(|p| p.2);
    //edges.select_nth_unstable_by_key(1000, |p| p.2);

    let mut uf = UnionFind::new(points.len());

    for &(v1, v2, _) in &heap.arr {
        uf.union(v1, v2) as usize;
    }

    let mut sizes: Vec<_> = (0..points.len())
        .filter(|&i| uf.parent[i] == i)
        .map(|i| uf.size[i])
        .collect();

    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}


struct BinaryHeap<const N: usize> {
    arr: [(usize, usize, i64); N]
}

impl<const N: usize> BinaryHeap<N> {
    fn insert(&mut self, u: usize, v: usize, w: i64) {
        if w > self.arr[0].2 {
            return;
        }
        self.arr[0] = (u, v, w);
        let mut current = 0;
        loop {
            let left = 2 * current + 1;
            let right = 2 * current + 2;
            let mut next = current;
            if left < N && self.arr[left].2 > self.arr[next].2 {
                next = left;
            }
            if right < N && self.arr[right].2 > self.arr[next].2 {
                next = right;
            }
            if next == current {
                break;
            }
            self.arr.swap(current, next);
            current = next;
        }
    }
}
*/