use ahash::{HashMap, HashMapExt};
use crate::util::{coord::*, iter::*, parser::*};

type V3 = Coord3<i32>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Particle {
    p: V3,
    v: V3,
    a: V3
}

impl Particle {
    fn tick(&mut self) {
        let v = self.v + self.a;
        self.p += v;
        self.v = v; 
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let particles: Vec<_> = input
        .iter_signed()
        .tuples()
        .map(|(px, py, pz, vx, vy, vz, ax, ay, az)| 
            Particle { p: Coord3::new(px, py, pz),
                                  v: Coord3::new(vx, vy, vz),
                                  a: Coord3::new(ax, ay, az)
            })
        .collect();

    let p1 = part1(&particles);
    let p2 = part2(&particles);
    (p1, p2)
}

fn part1(particles: &[Particle]) -> usize {
    particles
    .iter()
    .enumerate()
    .min_by_key(|(_, p)|  p.a.manhattan(V3::ORIGIN))
    .unwrap()
    .0
}

fn part2(particles: &[Particle]) -> usize {
    let mut particles = particles.to_vec();
    let mut exploded = vec![u32::MAX; particles.len()];
    let mut positions = HashMap::with_capacity(particles.len());
    for i in 1..40 {
        for (j, particle) in particles.iter_mut().enumerate() {
            if exploded[j] >= i {
                particle.tick();
                if let Some(j2) = positions.insert(particle.p, j) {
                    exploded[j] = i;
                    exploded[j2] = i;
                }
            }
        }
        positions.clear();
    }
    exploded.iter().filter(|&&t| t == u32::MAX).count()
}
