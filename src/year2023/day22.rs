// dominators
// https://en.wikipedia.org/wiki/Dominator_(graph_theory)

use std::collections::HashMap;

use itertools::Itertools;
use crate::util::{coord::Coord3, grid::Grid, iter::AOCIter};

type Dominators = Vec<(usize, u32)>;

fn parse_line(input: &str) -> Option<(Coord3, Coord3)> {
    let (left, right) = input.split_once('~')?;
    let (x1, y1, z1) = left.split(',').next_tuple()?;
    let (x2, y2, z2) = right.split(',').next_tuple()?;
    let x1 = x1.parse().ok()?;
    let y1 = y1.parse().ok()?;
    let z1 = z1.parse().ok()?;
    let x2 = x2.parse().ok()?;
    let y2 = y2.parse().ok()?;
    let z2 = z2.parse().ok()?;
    let p1 = Coord3::new(x1, y1, z1);
    let p2 = Coord3::new(x2, y2, z2);
    Some((p1.min(p2), p1.max(p2)))
}

pub fn parse(input: &str) -> Option<Dominators> {
    let mut bricks: Vec<_> = input.lines().filter_map(parse_line).collect();
    bricks.sort_unstable_by_key(|b| b.0.z);
    
    let xmax = bricks.iter().map(|&p| p.1.x).max()? as usize;
    let ymax = bricks.iter().map(|&p| p.1.y).max()? as usize;
    let mut heights = Grid::new(xmax+1, ymax+1, -1);
    let mut cube_owners = HashMap::new();
    let mut dominator = vec![(0, 0); bricks.len()+1];

    for i in 0..bricks.len() {
        let cubes = cubes_of(&bricks[i]);
        let height = cubes
            .iter()
            .map(|p| heights[(p.x, p.y)])
            .max()?;
        let fall = bricks[i].0.z - height - 1;
        let mut supported_by = vec!();
        for cube in &cubes {
            heights[(cube.x, cube.y)] = cube.z - fall;
            cube_owners.insert(Coord3::new(cube.x, cube.y, cube.z - fall), i);
            if let Some(&j) = cube_owners.get(&Coord3::new(cube.x, cube.y, cube.z - fall - 1)) {
                if i == j {
                    continue;
                }
                supported_by.push(j+1);
            }
        }
        supported_by.dedup();
        dominator[i+1] = match supported_by.len() {
            0 => (0, 0),
            1 => (supported_by[0], dominator[supported_by[0]].1 + 1),
            _ => lowest_common_ancestor(&dominator, &supported_by)
        }
    }
    Some(dominator)
}   

fn cubes_of((pmin, pmax): &(Coord3, Coord3)) -> Vec<Coord3> {
    let mut output = vec!();
    for x in pmin.x..=pmax.x {
        for y in pmin.y..=pmax.y {
            for z in pmin.z..=pmax.z {
                output.push(Coord3::new(x, y, z));
            }
        }
    }
    output
}

fn lowest_common_ancestor(ancestor: &[(usize, u32)], nodes: &[usize]) -> (usize, u32) {
    nodes.iter().map(|v| ancestor[*v]).reduce(|(n1, h1), (n2, h2)| {
        let (mut n1, mut h1) = (n1, h1);
        let (mut n2, mut h2) = (n2, h2);
        if h1 < h2 {
            while h1 != h2 {
                (n2, h2) = ancestor[n2];
            }
        }
        if h1 > h2 {
            while h1 != h2 {
                (n1, h1) = ancestor[n1];
            }
        }
        while n1 != n2 {
            (n1, h1) = ancestor[n1];
            (n2, _) = ancestor[n2];
        }
        (n1, h1)
    }).unwrap()
}


pub fn part1(dominator: &Dominators) -> Option<usize> {
    let mut safe = vec![true; dominator.len()];
    for (v, _) in dominator {
        safe[*v] = false;
    }
    Some(safe.iter().count_by(|&n| n))
}

pub fn part2(dominator: &Dominators) -> Option<u32> {
    Some(dominator.iter().map(|(_, h)| *h).sum())
}