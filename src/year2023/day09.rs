fn pascal_triangle(n: usize) -> Vec<i64> {
    let mut c = 1;
    let mut triangle = Vec::with_capacity(n+1);
    triangle.push(1);
    for i in 0..n {
        let n = n as i64;
        let i = i as i64;
        c = (c * (i - n)) / (i + 1);
        triangle.push(c);
    }
    triangle
}

pub fn parse(input: &str) -> Option<(i64, i64)> {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut prev_n = 0;
    let mut triangle = vec!();
    for line in input.lines() {
        let values: Vec<i64> = line.split(' ').filter_map(|x| x.parse().ok()).collect();
        let n = values.len();
        if n != prev_n {
            triangle = pascal_triangle(n);
        }
        let m = n-1;
        for (i, v) in values.iter().enumerate() {
            p1 += v * triangle[i];
            p2 += v * triangle[m-i];
        }
        prev_n = n;
    }

    Some((p1, p2))
}

pub fn part1(input: &(i64, i64)) -> Option<i64> {
    Some(input.0)
}

pub fn part2(input: &(i64, i64)) -> Option<i64> {
    Some(input.1)
}