use rayon::prelude::*;
use itertools::Itertools;

pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredient(line: &str) -> Option<Ingredient> {
    let (capacity, _, durability, _, flavor, _, texture, _, calories) = line.split(' ').skip(2).next_tuple()?;
    let capacity = capacity.trim_end_matches(',').parse().ok()?;
    let durability = durability.trim_end_matches(',').parse().ok()?;
    let flavor = flavor.trim_end_matches(',').parse().ok()?;
    let texture = texture.trim_end_matches(',').parse().ok()?;
    let calories = calories.parse().ok()?;
    Some(Ingredient { capacity, durability, flavor, texture, calories})
}

pub fn parse(input: &str) -> Option<Vec<Ingredient>> {
    Some(input.lines().filter_map(parse_ingredient).collect())
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

pub fn part1(ingredients: &[Ingredient]) -> Option<i32> {
    (0..100).into_par_iter().filter_map(|i| {
        let mut best_score = 0;
        for j in 0..=100-i {
            for k in 0..=100-i-j {
                let quantities = vec!(i, j, k, 100 - i - j - k);
                match score(&quantities, ingredients) {
                    Some(s) if s > best_score => {
                        best_score = s;
                    }
                    _ => ()
                } 
            }
        }
        Some(best_score)
    }).max()
}

pub fn part2(ingredients: &[Ingredient]) -> Option<i32> {
    let mut best_score = 0;
    for i in 0..=100 {
        for j in 0..=100-i {
            for k in 0..=100-i-j {
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
    Some(best_score)
}