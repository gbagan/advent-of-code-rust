use anyhow::*;
use std::cmp::max;
use itertools::Itertools;
use crate::util::{parallel::*, parser::*};

pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut ingredients = vec!();
    for (capacity, durability, flavor, texture, calories) in input.iter_unsigned().tuples() {
        ingredients.push(Ingredient { capacity, durability, flavor, texture, calories });
    }
    let p1 = part1(&ingredients);
    let p2 = part2(&ingredients);
    Ok((p1, p2))
}

fn score(quantities: & Vec<u32>, ingredients: &[Ingredient]) -> Option<i32> {
    let capacity: i32 = ingredients
        .iter()
        .zip(quantities)
        .map(|(ing, q)| ing.capacity * *q as i32)
        .sum();

    let durability: i32 = ingredients
        .iter()
        .zip(quantities)
        .map(|(ing, q)| ing.durability * *q as i32)
        .sum();

    let flavor: i32 = ingredients
        .iter()
        .zip(quantities)
        .map(|(ing, q)| ing.flavor * *q as i32)
        .sum();

    let texture: i32 = ingredients
        .iter()
        .zip(quantities)
        .map(|(ing, q)| ing.texture * *q as i32)
        .sum();

    if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
        None
    } else {
        Some(capacity * durability * flavor * texture)
    }
}

fn calories(quantities: & Vec<u32>, ingredients: &[Ingredient]) -> i32 {
    ingredients
        .iter()
        .zip(quantities)
        .map(|(ing, q)| ing.calories * *q as i32)
        .sum()
}

pub fn part1(ingredients: &[Ingredient]) -> i32 {
    (0usize..101).into_par_iter().map(|i| {
        let i = i as u32;
        let mut best_score = 0;
        for j in 0..100-i+1 {
            for k in 0..100-i-j+1 {
                let quantities = vec!(i, j, k, 100 - i - j - k);
                match score(&quantities, ingredients) {
                    Some(s) if s > best_score => {
                        best_score = s;
                    }
                    _ => ()
                } 
            }
        }
        best_score
    }).reduce(0, max)
}

pub fn part2(ingredients: &[Ingredient]) -> i32 {
    let mut best_score = 0;
    for i in 0..101 {
        for j in 0..101-i {
            for k in 0..101-i-j {
                let quantities = vec!(i, j, k, 100 - i - j - k);
                if calories(&quantities, ingredients) != 500 {
                    break;
                }
                match score(&quantities, ingredients) {
                    Some(s) if s > best_score => {
                        best_score = s;
                    }
                    _ => ()
                } 
            }
        }
    }
    best_score
}