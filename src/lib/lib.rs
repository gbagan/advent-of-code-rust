use std::time::Instant;
use std::fmt::Debug;

pub mod iter;
pub mod coord;
pub mod graph;
pub mod knothash;
pub mod number;
pub mod permutation;

pub fn aoc<A,B, F>(f: F)
    where
        A: Debug,
        B: Debug,
        F: Fn() -> (A, B), 
{
    let start = Instant::now();
    let (p1, p2) = f();

            
    let end = start.elapsed().as_micros();

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
    println!("Time: {} μs", end);
}