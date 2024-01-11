use std::time::Instant;
use std::fmt::Debug;
use nom::IResult;

pub mod iter;
pub mod coord;
pub mod graph;
pub mod grid;
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

pub fn aoc_with_parser<A,B,I,F,P>(input: &str, parser: P, f: F)
    where
        A: Debug,
        B: Debug,
        F: Fn(I) -> (A, B),
        P: Fn(&str) -> IResult<&str, I>
{
    match parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, data)) => {
            let start = Instant::now();
            let (p1, p2) = f(data);
            let end = start.elapsed().as_micros();

            println!("Part 1: {:?}", p1);
            println!("Part 2: {:?}", p2);
            println!("Time: {} μs", end);
        }
    }
}