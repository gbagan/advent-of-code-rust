// breadth first search with two vectors

use crate::util::{coord::Coord, parser::*};

type Point = Coord::<usize>;

const SIZE: usize = 64;

fn is_wall(p: Point, n: usize) -> bool {
    let Point {x, y} = p;
    (x*x + 3*x + 2*x*y + y + y*y + n).count_ones() & 1 == 1
}


pub fn solve(input: &str) -> (u32, u32) {
    let mut p2 = 0;
    let n = input.try_unsigned().unwrap();
    let end = Point::new(31, 39);
    let mut seen = vec![false; SIZE*SIZE];
    seen[SIZE+1] = true;
    let mut queue1 = Vec::new();
    let mut queue2 = Vec::new();
    queue1.push(Point::new(1, 1));
    let mut dist = 0;

    while !queue1.is_empty() {
        for &node in &queue1 {
            if node == end {
                return (dist, p2);
            } else if dist <= 50 {
                p2 += 1;
            }
            
            {
                let next = node.right();
                let index = SIZE*next.y+next.x;
                if !seen[index] && !is_wall(next, n) {
                    queue2.push(next);
                    seen[index] = true;
                }
            }

            {
                let next = node.below();
                let index = SIZE*next.y+next.x;
                if !seen[index] && !is_wall(next, n) {
                    queue2.push(next);
                    seen[index] = true;
                }
            }
            
            if node.x > 0 {
                let next = node.left();
                let index = SIZE*next.y+next.x;
                if !seen[index] && !is_wall(next, n) {
                    queue2.push(next);
                    seen[index] = true;
                }
            }
            if node.y > 0 {
                let next = node.above();
                let index = SIZE*next.y+next.x;
                if !seen[index] && !is_wall(next, n) {
                    queue2.push(next);
                    seen[index] = true;
                }
            }
        }
        dist += 1;
        std::mem::swap(&mut queue1, &mut queue2);
        queue2.clear();

    };
    
    /*
    while !queue1.is_empty() {
        if helper(&queue1, &mut queue2, dist) {
            return (dist, p2);
        }
        queue1.clear();
        dist += 1;
        if helper(&queue2, &mut queue1, dist) {
            return (dist, p2);
        }
        queue2.clear();
        dist += 1;
    }
    */
    unreachable!();
}