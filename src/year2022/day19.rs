use std::ops::{Add, Sub};
use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> (u32, u32) {
    let blueprints: Vec<_> = input.iter_unsigned().tuples().map(|(a, b, c, d, e, f, g)| {
        let id = a;
        let ore_robot_cost = Resource::new(b, 0, 0, 0);
        let clay_robot_cost = Resource::new(c, 0, 0, 0);
        let obsidian_robot_cost = Resource::new(d, e, 0, 0);
        let geode_robot_cost = Resource::new(f, 0, g, 0);
        let max_ore_cost = b.max(c).max(f);
        Blueprint { id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost, max_ore_cost }
    }).collect();
    
    let p1 = blueprints.iter().map(|bp| {
        let mut score = 0;
        explore(bp, Resource::new(0, 0, 0, 0),ONE_ORE_ROBOT, 24, &mut score);
        score * bp.id
    }).sum();

    let p2 = blueprints[0..3].iter().map(|bp| {
        let mut score = 0;
        explore(bp, Resource::new(0, 0, 0, 0),ONE_ORE_ROBOT, 32, &mut score);
        score
    }).product();

    (p1, p2)
}

fn explore(bp: &Blueprint, resources: Resource, robots: Resource, time: u32, best: &mut u32) {    
    *best = (*best).max(resources.geode + robots.geode * time);

    if upper_bound(bp, resources, robots, time) <= *best {
        return;
    }

    let mut build = |mut resources: Resource, cost: Resource, to_build: Resource| {
        for time2 in (1..time).rev() {
            if resources.geq(cost) {
                explore(bp, resources + robots - cost, robots + to_build,  time2, best);
                break;
            }
            resources = resources + robots;
        }
    };

    if time > 1 {
        build(resources, bp.geode_robot_cost, ONE_GEODE_ROBOT);
    }
    if time > 3 {
        build(resources, bp.obsidian_robot_cost, ONE_OBSIDIAN_ROBOT);
    }
    if time > 3 && robots.ore < bp.max_ore_cost {
        build(resources, bp.ore_robot_cost, ONE_ORE_ROBOT);
    }
    if time > 5 {
        build(resources, bp.clay_robot_cost, ONE_CLAY_ROBOT);
    }

}

#[inline]
fn upper_bound(bp: &Blueprint, mut resources: Resource, mut robots: Resource, time: u32) -> u32 {
    for _ in 0..time {
        resources.ore = bp.max_ore_cost;
        if resources.geq(bp.geode_robot_cost) {
            resources = resources + robots - bp.geode_robot_cost;
            robots.geode += 1;
        } else if resources.geq(bp.obsidian_robot_cost) {
            resources = resources + robots - bp.obsidian_robot_cost;
            robots.obsidian += 1;
        } else {
            resources = resources + robots;
            robots.clay += 1;
        }
    }
    resources.geode
}


#[derive(Clone, Copy)]
struct Resource {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Resource {
    const fn new(ore: u32, clay: u32, obsidian: u32, geode: u32) -> Self {
        Self { ore, clay, obsidian, geode }
    }
    
    #[inline]
    fn geq(self, other: Self) -> bool {
        self.ore >= other.ore
        && self.clay >= other.clay
        && self.obsidian >= other.obsidian
        // && self.geode >= other.geode
    }
}

impl Add for Resource {
    type Output = Resource;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(
            self.ore + other.ore,
            self.clay + other.clay,
            self.obsidian + other.obsidian,
            self.geode + other.geode,
        )
    }
}

impl Sub for Resource {
    type Output = Resource;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.ore - other.ore,
            self.clay - other.clay,
            self.obsidian - other.obsidian,
            self.geode - other.geode,
        )
    }
}

const ONE_ORE_ROBOT: Resource = Resource::new(1, 0, 0, 0);
const ONE_CLAY_ROBOT: Resource = Resource::new(0, 1, 0, 0);
const ONE_OBSIDIAN_ROBOT: Resource = Resource::new(0, 0, 1, 0);
const ONE_GEODE_ROBOT: Resource = Resource::new(0, 0, 0, 1);

struct Blueprint {
    id: u32,
    ore_robot_cost: Resource,
    clay_robot_cost: Resource,
    obsidian_robot_cost: Resource,
    geode_robot_cost: Resource,
    max_ore_cost: u32,
}
