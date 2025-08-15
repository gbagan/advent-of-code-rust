use crate::util::parser::*;

pub fn solve(input: &str) -> (i32, String) {
    let mut values = vec!(1);
    let it = input.split_ascii_whitespace().scan(1, |acc, token| {
        match token {
            "noop" | "addx" => (),
            _ => *acc += token.try_signed::<i32>()?,
        };
        Some(*acc)
    }).take(240);
    values.extend(it);
    
    let p1 = values
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, v)| (i+1) as i32 * v)
        .sum();

    let mut p2 = String::new();
    for chunk in values.chunks_exact(40) {
        p2.push('\n');
        for (i, &c) in chunk.iter().enumerate() {
            p2.push(draw_pixel(i, c));
        }
    }

    (p1, p2)
}

fn draw_pixel (i: usize, c: i32) -> char {
    if (i as i32).abs_diff(c) <= 1 {
        '#'
    } else {
        '.'
    }
}