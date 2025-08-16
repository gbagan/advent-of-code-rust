// assume that all coordinates are between 0 and 19

use crate::util::{iter::*, parser::*};

const SIZE: usize = 24;

pub fn solve(input: &str) -> (u32, u32) {
    let mut points = Vec::new();
    let mut empty = [true; SIZE*SIZE*SIZE];
    let mut xmin = usize::MAX;
    let mut xmax = usize::MIN;
    let mut ymin = usize::MAX;
    let mut ymax = usize::MIN;
    let mut zmin = usize::MAX;
    let mut zmax = usize::MIN;

    for (mut x, mut y, mut z) in input.iter_unsigned::<usize>().tuples() {
        x += 2;
        y += 2;
        z += 2;
        xmin = xmin.min(x);
        xmax = xmax.max(x);
        ymin = ymin.min(y);
        ymax = ymax.max(y);
        zmin = zmin.min(z);
        zmax = zmax.max(z);
        points.push((x, y, z));
        empty[x*SIZE*SIZE+y*SIZE+z] = false;
    }
    xmin -= 2;
    xmax += 2;
    ymin -= 2;
    ymax += 2;
    zmin -= 2;
    zmax += 2;

    // part 1

    let mut p1 = 0;
    for &(x, y, z) in &points {
        let index = x*SIZE*SIZE+y*SIZE+z;
        p1 += empty[index-1] as u32;
        p1 += empty[index+1] as u32;
        p1 += empty[index-SIZE] as u32;
        p1 += empty[index+SIZE] as u32;
        p1 += empty[index-SIZE*SIZE] as u32;
        p1 += empty[index+SIZE*SIZE] as u32;
    }


    // part 2

    let mut seen = [false; SIZE*SIZE*SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            seen[xmin * SIZE * SIZE + i * SIZE + j] = true;
            seen[xmax * SIZE * SIZE + i * SIZE + j] = true;
            seen[i * SIZE * SIZE + ymin * SIZE + j] = true;
            seen[i * SIZE * SIZE + ymax * SIZE + j] = true;
            seen[i * SIZE * SIZE + j * SIZE + zmin] = true;
            seen[j * SIZE * SIZE + j * SIZE + zmax] = true;
        }
    }
    let start = (xmin+1) * SIZE * SIZE + (ymin+1) * SIZE + zmin+1;
    seen[start] = true;
    let mut p2 = 0;
    
    let mut stack = Vec::with_capacity(50);
    stack.push(start);
    while let Some(index) = stack.pop() {
        let mut do_next = |next: usize| {
            if !empty[next] {
                p2 += 1;
            } else if !seen[next] {
                seen[next] = true;
                stack.push(next);
            }
        };
        do_next(index-1);
        do_next(index+1);
        do_next(index-SIZE);
        do_next(index+SIZE);
        do_next(index-SIZE*SIZE);
        do_next(index+SIZE*SIZE);
    }

    (p1, p2)
}