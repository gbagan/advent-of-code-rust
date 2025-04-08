use anyhow::*;
use crate::util::parser::*;
use itertools::Itertools;
use std::array::from_fn;

struct Contents {
    amount: u32,
    bag_index: usize
}

type Bag = Vec<Contents>;

const ATTRIBUTE1: [&str; 18] = [
    "bright",
    "clear",
    "dark",
    "dim",
    "dotted",
    "drab",
    "dull",
    "faded",
    "light",
    "mirrored",
    "muted",
    "pale",
    "plaid",
    "posh",
    "shiny",
    "striped",
    "vibrant",
    "wavy"
];

static ATTRIBUTE2: [&str; 33] = [
    "aqua",
    "beige",
    "black",
    "blue",
    "bronze",
    "brown",
    "chartreuse",
    "coral",
    "crimson",
    "cyan",
    "fuchsia",
    "gold",
    "gray",
    "green",
    "indigo",
    "lavender",
    "lime",
    "magenta",
    "maroon",
    "olive",
    "orange",
    "plum",
    "purple",
    "red",
    "salmon",
    "silver",
    "tan",
    "teal",
    "tomato",
    "turquoise",
    "violet",
    "white",
    "yellow"
];

#[inline]
fn first_hash(attr: &str) -> usize {
    let attr = attr.as_bytes();
    let a = attr[0] as usize;
    let b = attr[1] as usize;
    26 * a + b - 2619
}

#[inline]
fn second_hash(attr: &str) -> usize {
    let attr = attr.as_bytes();
    let a = attr[0] as usize;
    let b = attr[1] as usize + attr.len() % 2;
    26 * a + b - 2619
}

pub fn solve(input: &str) -> Result<(usize, u32)> {
    let mut idx1 = [0; 676];
    let mut idx2 = [0; 676];
    
    let mut bags: [Bag; 594] = from_fn(|_| vec!());
    
    for (i, attr) in ATTRIBUTE1.iter().enumerate() {
        idx1[first_hash(attr)] = i;
    }

    for (i, attr) in ATTRIBUTE2.iter().enumerate() {
        idx2[second_hash(attr)] = i;
    }

    let perfect_hash = |attr1: &str, attr2: &str| {
        idx1[first_hash(attr1)] + 18 * idx2[second_hash(attr2)]
    };

    for line in input.lines() {
        let mut it = line.split_ascii_whitespace().tuples();
        if let Some((attr1, attr2, _, _)) = it.next() {
            let index = perfect_hash(attr1, attr2);
            for (amount, attr1, attr2, _) in it {
                let amount: u32 = amount.try_unsigned()?;
                let index2 = perfect_hash(attr1, attr2);
                bags[index].push(Contents {amount, bag_index: index2})
            }
        }
    }
    let shinygold = perfect_hash("shiny", "gold");

    let p1 = part1(&bags, shinygold);
    let p2 = part2(&bags, shinygold);

    Ok((p1, p2))
}


fn part1(bags: &[Bag], shinygold: usize) -> usize {
    let mut cache = [None; 594];
    (0..594).filter(|&index| contains_shinygold(index, bags, shinygold, &mut cache)).count()
}

fn contains_shinygold(index: usize, bags: &[Bag], shinygold: usize, cache: &mut [Option<bool>]) -> bool {
    if let Some(res) = cache[index] {
        res
    } else {
        let res = bags[index].iter().any(|contents|
            contents.bag_index == shinygold || contains_shinygold(contents.bag_index, bags, shinygold, cache)
        );
        cache[index] = Some(res);
        res 
    }
}

fn part2(bags: &[Bag], shinygold: usize) -> u32 {
    let mut cache = [None; 594];
    contained(1, shinygold, bags, &mut cache) - 1
}

fn contained(amount: u32, index: usize, bags: &[Bag], cache: &mut [Option<u32>]) -> u32 {
    if let Some(res) = cache[index] {
        amount * res
    } else {
        let res = 1 + bags[index].iter().map(|contents|
            contained(contents.amount, contents.bag_index, bags, cache)
        ).sum::<u32>();
        cache[index] = Some(res);
        amount * res
    }
}
