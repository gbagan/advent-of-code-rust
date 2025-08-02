use std::sync::Mutex;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use crate::util::{coord::Coord3, parser::*};

pub fn solve(input: &str) -> (usize, i32) {
    let scanners: Vec<_> = input
        .split("\n\n")
        .map(Scanner::parse)
        .collect();

    let n = scanners.len();

    let matrix = vec![None; n * n];
    let mutex = Mutex::new(matrix);

    let mut tasks = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n - 1 {
        for j in i+1..n {
            tasks.push([i, j]);
        }
    }

    let counter = AtomicUsize::new(0);

    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(&scanners, &tasks, &counter, &mutex));
        }
    });

    let matrix = mutex.into_inner().unwrap();
    
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut todo = vec![(0, Transformation::default())];

    let mut located_scanners = Vec::with_capacity(n);
    let mut located_beacons = HashSet::with_capacity(500);

    while let Some((idx, tr)) = todo.pop() {
        located_scanners.push(tr.translation);
        let beacons = &scanners[idx].beacons;
        for &beacon in beacons {
            located_beacons.insert(tr.apply(beacon));
        }

        for i in 0..n {
            if visited[i] {
                continue;
            }
            if let Some(tr2) = matrix[idx*n + i] {
                todo.push((i, tr2.compose(&tr)));
                visited[i] = true;
            }
        }
    }

    let p1 = located_beacons.len();

    let mut p2 = 0;
    for i in 0..located_scanners.len() - 1 {
        for j in i+1..located_scanners.len() {
            p2 = p2.max(located_scanners[i].manhattan(&located_scanners[j]));
        }
    }

    (p1, p2)
}

fn worker(scanners: &[Scanner], tasks: &[[usize; 2]], counter: &AtomicUsize, mutex: &Mutex<Vec<Option<Transformation>>>) {
    loop {
        let start = counter.fetch_add(16, Ordering::Relaxed);
        if start >= tasks.len() {
            break;
        }
        for i in start..(start+16).min(tasks.len()) {
            let [idx1, idx2] = tasks[i];
            if let Some(transformation) = scanners[idx1].overlaps(&scanners[idx2]) {
                let mut matrix = mutex.lock().unwrap();
                matrix[idx2 * scanners.len() + idx1] = Some(transformation.inverse());
                matrix[idx1 * scanners.len() + idx2] = Some(transformation);
            }

        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Rotation {
    idx: u32,
}

impl Rotation {
    const fn apply(self, point: Coord3) -> Coord3 {
        let Coord3 {x, y, z} = point;
        match self.idx {
            0 => Coord3::new(x, y, z),
            1 => Coord3::new(x, -y, -z),
            2 => Coord3::new(-x, y, -z),
            3 => Coord3::new(-x, -y, z),

            4 => Coord3::new(x, z, -y),
            5 => Coord3::new(x, -z, y),
            6 => Coord3::new(-x, z, y),
            7 => Coord3::new(-x, -z, -y),

            8 => Coord3::new(y, x, -z),
            9 => Coord3::new(y, -x, z),
            10 => Coord3::new(-y, x, z),
            11 => Coord3::new(-y, -x, -z),

            12 => Coord3::new(y, z, x),
            13 => Coord3::new(y, -z, -x),
            14 => Coord3::new(-y, z, -x),
            15 => Coord3::new(-y, -z, x),
            
            16 => Coord3::new(z, y, -x),
            17 => Coord3::new(z, -y, x),
            18 => Coord3::new(-z, y, x),
            19 => Coord3::new(-z, -y, -x),

            20 => Coord3::new(z, x, y),
            21 => Coord3::new(z, -x, -y),
            22 => Coord3::new(-z, x, -y),
            23 => Coord3::new(-z, -x, y),

            _ => unreachable!(),
        }
    }

    #[inline]
    fn compose(self, other: Self) -> Self {
        Self { idx: COMPOSITION[(self.idx * 24 + other.idx) as usize] }
    }

    #[inline]
    fn inverse(self) -> Self {
        Self { idx: INVERSE[self.idx as usize] }
    }

    #[inline]
    fn default() -> Self {
        Self { idx: 0 }
    }
}


const COMPOSITION: [u32; 24*24] = {
    let mut results = [Coord3::new(0, 0, 0); 24];
    let mut table = [0; 24*24];

    let mut i = 0;
    while i < 24 {
        results[i] = (Rotation { idx: i as u32 }).apply(Coord3::new(1, 2, 3));
        i += 1;
    }

    let mut i = 0usize;
    while i < 24*24 {
        let res = (Rotation { idx: i as u32 / 24 }).apply(Coord3::new(1, 2, 3));
        let res = (Rotation { idx: i as u32 % 24 }).apply(res);
        let mut j = 0;
        while j < 24 {
            if res.x == results[j].x && res.y == results[j].y && res.z == results[j].z {
                table[i] = j as u32;
                break;
            }
            j += 1;
        }
        i += 1;
    }

    table
};

const INVERSE: [u32; 24] = {
    let mut table = [0; 24];

    let mut i = 0usize;
    while i < 24 {
        let res = (Rotation { idx: i as u32 }).apply(Coord3::new(1, 2, 3));
        let mut j = 0;
        while j < 24 {
            let res2 = (Rotation { idx: j as u32 }).apply(res);
            if res2.x == 1 && res2.y == 2 && res2.z == 3 {
                table[i] = j as u32;
                break;
            }
            j += 1;
        }
        i += 1;
    }

    table
};

#[derive(Clone, Copy, Debug)]
struct Transformation {
    rotation: Rotation,
    translation: Coord3,
}

impl Transformation {
    #[inline]
    fn apply(&self, point: Coord3) -> Coord3 {
        self.rotation.apply(point) + self.translation
    }

    fn compose(&self, other: &Self) -> Self {
        Transformation {
            rotation: self.rotation.compose(other.rotation),
            translation: other.rotation.apply(self.translation) + other.translation,
        }
    }

    fn inverse(&self) -> Self {
        Transformation {
            rotation: self.rotation.inverse(),
            translation: -self.rotation.inverse().apply(self.translation)
        }
    }

    fn default() -> Self {
        Transformation {
            rotation: Rotation::default(),
            translation: Coord3::new(0, 0, 0),
        }
    }
}


struct Scanner {
    beacons: Vec<Coord3>,
    distances: HashMap<i32, [usize; 2]>,
}

impl Scanner {
    fn parse(input: &str) -> Scanner {
        let beacons: Vec<_> = (&input[18..])
            .iter_signed()
            .array_chunks().map(|[x, y, z]| Coord3::new(x, y, z))
            .collect();
        let mut distances = HashMap::with_capacity(beacons.len()*beacons.len()/2);
        for i in 0..beacons.len() - 1 {
            for j in i+1..beacons.len() {
                distances.insert(beacons[i].euclidean(&beacons[j]), [i, j]);
            }
        }

        Scanner { beacons, distances }
    }

    fn overlaps(&self, other: &Self) -> Option<Transformation> {
        fn overlaps_helper(scan1: &Scanner, scan2: &Scanner, points: [Coord3; 4]) -> Option<Transformation> {
            let [p11, p12, p21, p22] = points;

            for idx in 0..24 {
                let rotation = Rotation { idx };
                let rp21 = rotation.apply(p21);
                let rp22 = rotation.apply(p22);

                let translation = if rp21 - rp22 == p11 - p12 {
                    p12 - rp22
                } else if rp22 - rp21 == p11 - p12 {
                    p12 - rp21
                } else {
                    continue;
                };

                let transformation = Transformation { translation, rotation };
                let mut count = 0;

                for &candidate in &scan2.beacons {
                    let point = transformation.apply(candidate);

                    if scan1.beacons.contains(&point) {
                        count += 1;
                        if count == 12 {
                            return Some(transformation);
                        }
                    }
                }
            }
            None
        }
        
        let mut nb_matchings: u32 = 0;

        for dist in self.distances.keys() {
            if other.distances.contains_key(dist) {
                nb_matchings += 1;
                if nb_matchings == 66 {
                    let [idx1, idx2] = self.distances[dist];
                    let [idx3, idx4] = other.distances[dist];
                    let points =
                        [self.beacons[idx1], self.beacons[idx2], other.beacons[idx3], other.beacons[idx4]];
                    return overlaps_helper(self, other, points);
                }
            }
        }

        None
    }
}

