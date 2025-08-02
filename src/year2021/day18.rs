use crate::util::parallel::*;

pub fn solve(input: &str) -> (u32, u32) {
    let snailfishes: Vec<_> = input.lines().map(Snailfish::parse).collect();

    let p1 = snailfishes
        .iter()
        .copied()
        .reduce(|f1, f2| f1.add(&f2))
        .unwrap()
        .magnitude();

    let p2 = snailfishes.into_par_iter()
        .map(|fish| {
            snailfishes
                .iter().map(|fish2| fish.add(fish2).magnitude())
                .max()
                .unwrap()
        }).reduce(0, std::cmp::max);

    (p1, p2)
}

#[inline]
const fn next_sibling(node: usize) -> usize {
    node + 1
}

#[inline]
const fn left_child(node: usize) -> usize {
    2 * node + 1
}

#[inline]
const fn right_child(node: usize) -> usize {
    2 * node + 2
}

#[inline]
const fn parent(node: usize) -> usize {
    (node - 1) / 2
}

const EMPTY: u32 = u32::MAX;

const TRAVERSAL_ORDER: [usize; 31] = {
    let mut table = [0; 31];

    const fn traverse(table: &mut [usize; 31], idx: usize, node: usize, depth: usize) -> usize {
        table[idx] = node; 
        
        if depth < 4 {
            let idx2 = traverse(table, idx+1, left_child(node), depth+1);
            traverse(table, idx2, right_child(node), depth+1)
        } else {
            idx+1
        }
    }

    traverse(&mut table, 0, 0, 0);

    table
};

#[derive(Clone, Copy)]
struct Snailfish {
    tree: [u32; 63]
}

impl Snailfish {
    fn parse(line: &str) -> Self {
        let mut tree = [EMPTY; 63];
        let mut idx = 0;

        for c in line.bytes() {
            match c {
                b'[' => idx = left_child(idx),
                b']' => idx = parent(idx),
                b',' => idx = next_sibling(idx),
                _ => tree[idx] = (c - b'0') as u32,
            }
        }

        Self { tree }
    }

    fn add(&self, other: &Self) -> Self {
        let mut tree = [EMPTY; 63];

        tree[3..5].copy_from_slice(&self.tree[1..3]);
        tree[5..7].copy_from_slice(&other.tree[1..3]);
        
        tree[7..11].copy_from_slice(&self.tree[3..7]);
        tree[11..15].copy_from_slice(&other.tree[3..7]);
        
        tree[15..23].copy_from_slice(&self.tree[7..15]);
        tree[23..31].copy_from_slice(&other.tree[7..15]);
        
        tree[31..47].copy_from_slice(&self.tree[15..31]);
        tree[47..63].copy_from_slice(&other.tree[15..31]);

        let mut res = Self { tree };

        for idx in (31..63).step_by(2) {
            if res.tree[idx] != EMPTY {
                res.explode(idx);
            }
        }
        res.split();
        res
    }

    fn explode(&mut self, left: usize) {
        let right = next_sibling(left);
        if left > 31 {
            let mut idx = left - 1;
            while self.tree[idx] == EMPTY {
                idx = parent(idx);
            }
            self.tree[idx] += self.tree[left];
        }

        if right < 62 {
            let mut idx = right + 1;
            while self.tree[idx] == EMPTY {
                idx = parent(idx);
            }
            self.tree[idx] += self.tree[right];
        }
        self.tree[left] = EMPTY;
        self.tree[right] = EMPTY;
        self.tree[parent(left)] = 0;
    }

    fn split(&mut self) {
        'outer: loop {
            for &idx in &TRAVERSAL_ORDER {
                let v = self.tree[idx];
                if v != EMPTY && v >= 10 {
                    self.tree[left_child(idx)] = v / 2;
                    self.tree[right_child(idx)] = v.div_ceil(2);
                    self.tree[idx] = EMPTY;
                    if idx >= 15 {
                        self.explode(left_child(idx));
                    }
                    continue 'outer;
                }
            }
            return;
        }
    }

    fn magnitude(&mut self) -> u32 {
        for i in (0..31).rev() {
            if self.tree[i] == EMPTY {
                self.tree[i] = 3 * self.tree[left_child(i)] + 2 * self.tree[right_child(i)];
            }
        }
        self.tree[0]
    }
}
