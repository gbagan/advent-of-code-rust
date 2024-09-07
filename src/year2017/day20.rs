use anyhow::*;
use std::collections::HashMap;
use itertools::Itertools;
use crate::util::{coord::Coord3, iter::*, parser::*};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Particle {
    p: Coord3,
    v: Coord3,
    a: Coord3
}

impl Particle {
    fn tick(&mut self) {
        let v = self.v + self.a;
        self.p += v;
        self.v = v; 
    }
}

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut particles = Vec::new();
    for (px, py, pz, vx, vy, vz, ax, ay, az) in input.iter_signed().tuples() {
        particles.push(Particle { p: Coord3::new(px, py, pz),
                                  v: Coord3::new(vx, vy, vz),
                                  a: Coord3::new(ax, ay, az)
                                });
    }

    let p1 = part1(&particles);
    let p2 = part2(&particles);
    Ok((p1, p2))
}

fn part1(particles: &[Particle]) -> usize {
    particles
    .iter()
    .enumerate()
    .min_by_key(|(_, p)|  p.a.manhattan(&Coord3::ORIGIN))
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
    exploded.iter().count_if(|&t| t == u32::MAX)
}
