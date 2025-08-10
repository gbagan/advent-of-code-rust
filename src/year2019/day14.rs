use ahash::{HashMap, HashMapExt};
use arrayvec::ArrayVec;

use crate::util::parser::*;

#[derive(Clone, Default)]
struct Ingredient {
    amount: u64,
    chemical: usize,
}

#[derive(Clone, Default)]
struct Reaction {
    amount: u64,
    ingredients: ArrayVec<Ingredient, 10>,
}

pub fn solve(input: &str) -> (u64, u64) {
    let reactions = parse(input);
    let order = topological_sort(&reactions);

    let p1 = ore_amount(&reactions, &order, 1);

    const N: u64 = 1_000_000_000_000;

    let mut lo = N / p1;
    let mut hi = 2 * lo;
    while ore_amount(&reactions, &order, hi) <= N {
        hi *= 2;
    }

    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if ore_amount(&reactions, &order, mid) <= N {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    (p1, lo)
}

fn parse(input: &str) -> Vec<Reaction> {
    let lines: Vec<_> = input.trim().as_bytes().split(|&c| c==b'\n').collect();

    let mut indices = HashMap::new();   
    let mut tokens = Vec::new();
    let mut reactions = vec![Reaction::default(); lines.len() + 1];
    
    indices.insert(b"FUEL".as_slice(), 0);
    indices.insert(b"ORE".as_slice(), 1);

    for line in lines {
        tokens.extend(line
            .split(|&c| !c.is_ascii_alphanumeric())
            .filter(|s| !s.is_empty())
            .array_chunks()
            .map(|[a, c]| (a.to_unsigned(), c))
        );
        let (amount, chemical) = tokens[tokens.len() - 1];

        let size = indices.len();
        let chemical = *indices.entry(chemical).or_insert(size);

        let reaction = Reaction {
            amount,
            ingredients: tokens[..tokens.len()-1].iter().map(|&(a, c)| {
                let size = indices.len();
                Ingredient { amount: a, chemical: *indices.entry(c).or_insert(size) }
            }).collect()
        };

        reactions[chemical] = reaction;
        tokens.clear();
    }
    reactions[1].amount = 1;

    reactions
}

fn topological_sort(reactions: &[Reaction]) -> Vec<usize> {
    let mut seen = vec![false; reactions.len()];
    let mut order = Vec::new();
    
    fn dfs(reactions: &[Reaction], seen: &mut [bool], order: &mut Vec<usize>, i: usize) {
        seen[i] = true;
        for ingredient in &reactions[i].ingredients {
            if !seen[ingredient.chemical] {
                dfs(reactions, seen, order, ingredient.chemical);
            }
        }
        order.push(i);
    }
    
    for i in 0..reactions.len() {
        if !seen[i] {
            dfs(reactions, &mut seen, &mut order, i);
        }
    }
    order.reverse();
    order
}

fn ore_amount(reactions: &[Reaction], order: &[usize], fuel_amount: u64) -> u64 {
    let mut amounts = vec![0; reactions.len()];
    amounts[0] = fuel_amount;

    for &i in order {
        let reaction = &reactions[i];
        let n = amounts[i].div_ceil(reaction.amount);

        for ingredient in &reaction.ingredients {
            amounts[ingredient.chemical] += n * ingredient.amount;
        }
    }

    amounts[1]
}
