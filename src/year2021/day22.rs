use crate::util::parser::*;

pub fn solve(input: &str) -> (i64, i64) {
    let cubes: Vec<_> = input.lines().map(Cube::parse).collect();

    let region = Cube {
        on: true,
        xmin: -50,
        xmax: 50,
        ymin: -50,
        ymax: 50,
        zmin: -50,
        zmax: 50
    };

    let p1_cubes: Vec<_> = cubes.iter().filter_map(|c| c.intersect(&region)).collect();

    let p1 = reboot(&p1_cubes);
    let p2 = reboot(&cubes);
    (p1, p2)
}


fn reboot(cubes: &[Cube]) -> i64 {
    let mut total = 0;
    let mut intersected = Vec::new();

    for (i, cube) in cubes.iter().enumerate() {
        if !cube.on {
            continue;
        }

        intersected.extend(cubes[i+1..]
            .iter()
            .filter(|cube2| !cube2.disjoint(cube))
        );

        total += include_exclude(cube, &intersected);
        intersected.clear();
    }

    total
}

fn include_exclude(cube: &Cube, others: &[Cube]) -> i64 {
    let mut total = cube.volume();

    for (i, other) in others.iter().enumerate() {
        if let Some(cube2) = cube.intersect(other) {
            total -= include_exclude(&cube2, &others[(i + 1)..]);
        }
    }

    total
}


#[derive(Clone, Copy)]
struct Cube {
    on: bool,
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

impl Cube {
    fn parse(line: &str) -> Self {
        let on = line.starts_with("on");
        let [xmin, xmax, ymin, ymax, zmin, zmax] = line.iter_signed().next_chunk().unwrap();
        Cube { on, xmin, xmax, ymin, ymax, zmin, zmax }
    }

    fn volume(&self) -> i64 {
        (self.xmax - self.xmin + 1)
        * (self.ymax - self.ymin + 1)
        * (self.zmax - self.zmin + 1)
    }

    fn disjoint(&self, other: &Self) -> bool {
        self.xmax < other.xmin || self.ymax < other.ymin || self.zmax < other.zmin 
        || other.xmax < self.xmin || other.ymax < self.ymin || other.zmax < self.zmin
    }

    fn intersect(&self, other: &Self) -> Option<Cube> {
        if self.disjoint(other) {
            None
        } else {
            Some(Cube {
                on: self.on,
                xmin: self.xmin.max(other.xmin),
                xmax: self.xmax.min(other.xmax),
                ymin: self.ymin.max(other.ymin),
                ymax: self.ymax.min(other.ymax),
                zmin: self.zmin.max(other.zmin),
                zmax: self.zmax.min(other.zmax),
            })
        }
    }
}