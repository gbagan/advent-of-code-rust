use crate::util::parser::*;

pub fn solve(input: &str) -> (i32, i32) {
    let ingredients: Vec<[i32; 5]> = input.iter_signed().array_chunks().collect();
    let p1 = part1(&ingredients);
    let p2 = part2(&ingredients);
    (p1, p2)
}

pub fn part1(ingredients: &[[i32; 5]]) -> i32 {
    let mut sol = [25, 25, 25, 25];
    let mut prev_score = i32::MIN;

    loop {
        let (sol2, score2) = (0..16).map(|i| {
            let mut sol2 = sol;
            sol2[i>>2] -= 1;
            sol2[i&3] += 1;
            (sol2, score(sol2, ingredients))
        }).max_by_key(|x| x.1).unwrap();
        if score2 == prev_score {
            return score2
        }
        prev_score = score2;
        sol = sol2;
    }
}

#[inline]
fn score(quantities: [i32; 4], ingredients: &[[i32; 5]]) -> i32 {
    let vals: [i32; 4] = std::array::from_fn(|i|
        quantities.iter().zip(ingredients).map(|(&q, ing)| q * ing[i]).sum()
    );

    let v= vals[0].min(0) + vals[1].min(0) + vals[2].min(0) + vals[3].min(0);
    if v == 0 {
        vals[0] * vals[1] * vals[2] * vals[3]
    } else {
        1000 * v
    }
}

#[inline]
fn calories(quantities: [i32; 4], ingredients: &[[i32; 5]]) -> i32 {
    quantities.iter().zip(ingredients).map(|(&q, ing)| q * ing[4]).sum()
}

pub fn part2(ingredients: &[[i32; 5]]) -> i32 {
    let mut best_score = i32::MIN;
    if ingredients[2][4] != ingredients[3][4] {
        let c3 = ingredients[2][4];
        let c4 = ingredients[3][4];
        for i in 0..101 {
            let c1 = i * ingredients[0][4];
            for j in 0..101-i {
                let c2 = j * ingredients[2][4];
                if let Some((k, l)) = solve_equations(c3, c4, 100 - i - j, 500 - c1 - c2) {
                
                    let quantities = [i, j, k, l];
                    best_score = best_score.max(score(quantities, ingredients));
                }
            }
        }
    } else if ingredients[1][4] != ingredients[3][4] {   
        let c2 = ingredients[1][4];
        let c4 = ingredients[3][4];
        for i in 0..101 {
            let c1 = i * ingredients[0][4];
            for k in 0..101-i {
                let c3 = k * ingredients[2][4];
                if let Some((j, l)) = solve_equations(c2, c4, 100 - i - k, 500 - c1 - c3) {
                
                    let quantities = [i, j, k, l];
                    best_score = best_score.max(score(quantities, ingredients));
                }
            }
        }
    } else {
        panic!();
    };
    best_score
}

#[inline]
fn solve_equations(c3: i32, c4: i32, t1: i32, t2: i32) -> Option<(i32, i32)> {
    let det1 = t2 - c4 * t1;
    let det2 = c3 - c4;
    if det2 == 0 || det1 % det2 != 0 {
        return None;
    }
    let x = det1 / det2;
    let y = t1 - x;

    if x < 0 || y < 0 {
        return None;
    }
    Some((x, y))
}

#[test]
fn solve_equations_test() {
    assert_eq!(solve_equations(3, 5, 10, 44), Some((3, 7)));
}