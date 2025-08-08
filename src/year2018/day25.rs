use crate::util::parser::*;

type Point = [i16; 4];

pub fn solve(input: &str) -> (u32, u32) {
    let mut points: Vec<Point> = input.iter_signed().array_chunks().collect();

    let mut constellations = 0;
    let mut stack = Vec::with_capacity(100);

     while let Some(start) = points.pop() {
        constellations += 1;
        stack.push(start);

        while let Some(point) = stack.pop() {
            let mut i = 0;
            while i < points.len() {
                if adjacent(&point, &points[i]) {
                    stack.push(points.swap_remove(i));
                } else {
                    i += 1;
                }
            }
        }
    }

    (constellations, 0)
}

fn adjacent(p1: &Point, p2: &Point) -> bool {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs() + (p1[2] -p2[2]).abs() + (p1[3] - p2[3]).abs() <= 3
}