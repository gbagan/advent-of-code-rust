use crate::util::parser::*;

// part 1 assumes that all non empty nodes have > 50% use

pub struct Node {
    x: u32,
    y: u32,
    used: u32,
    avail: u32,
}

pub fn solve(input: &str) -> (usize, u32) {
    let nodes = input.as_bytes()[28+48..]
        .array_chunks::<48>()
        .map(|line| {
            let [x, y] = (&line[16..22]).iter_unsigned().next_chunk().unwrap();
            let used = (&line[30..33]).try_unsigned().unwrap();
            let avail = (&line[37..40]).try_unsigned().unwrap();
            Node { x, y, used, avail }
        })
        .collect::<Vec<_>>();
    
    let p1 = part1(&nodes);
    let p2 = part2(&nodes);
    (p1, p2)
}

fn part1(nodes: &[Node]) -> usize {
    let mut used: Vec<_> = nodes.iter().map(|n| n.used).filter(|&u| u > 0).collect();
    let mut avail: Vec<_> = nodes.iter().map(|n| n.avail).collect();
    used.sort_unstable();
    avail.sort_unstable();
    let mut viable = 0;
    let mut idx = 0;

    for a in avail {
        while idx < used.len() && used[idx] <= a {
            idx += 1;
        }
        viable += idx;
    }

    viable
}

fn part2(nodes: &[Node]) -> u32 {
    let mut width = 0;
    let mut left_wall_x = u32::MAX;
    let mut empty = None;

    for node in nodes {
        width = width.max(node.x+1);
        if node.used >= 100 {
            left_wall_x = left_wall_x.min(node.x);
        } else if node.used == 0 {
            empty = Some((node.x, node.y));
        }
    }

    let (empty_x, empty_y) = empty.unwrap();

    empty_x + empty_y + 6 * width - 2 * left_wall_x - 9
}