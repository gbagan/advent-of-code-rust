use aoc::aoc_with_parser;
use nom::{
    bytes::complete::{take_till, tag},
    character::complete::{line_ending,space1,i64},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};

fn input_parser() {
    
}