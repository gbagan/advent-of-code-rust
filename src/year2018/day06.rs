use crate::util::{coord::*, parser::*};

struct Input {
    points: Vec<Coord<i32>>,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

pub fn solve(input: &str) -> (usize, i32) {
    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    
    let points: Vec<_> = input
        .iter_unsigned()
        .array_chunks()
        .map(|[x, y]| {
            xmin = xmin.min(x);
            xmax = xmax.max(x);
            ymin = ymin.min(y);
            ymax = ymax.max(y);
            Coord::new(x, y)
        }).collect();

    let input = Input {points, xmin, xmax, ymin, ymax};

    let p2 = part2(&input, 10000);

    (0, p2)
}

fn part2(input: &Input, limit: i32) -> i32 {
    let Input {points, xmin: _, xmax, ymin: _, ymax} = input;

    let mut xs: Vec<_> = points.iter().map(|&p| p.x).collect();
    let mut ys: Vec<_> = points.iter().map(|&p| p.y).collect();
    let mid = xs.len() / 2;
    let xmid = *xs.select_nth_unstable(mid).1;
    let ymid = *ys.select_nth_unstable(mid).1;
    let mut xcount = vec![0; *xmax as usize + 1];
    for &x in &xs {
        xcount[x as usize] += 1;
    }
    let mut ycount = vec![0; *ymax as usize + 1];
    for &y in &ys {
        ycount[y as usize] += 1;
    }

    let mut delta_x = vec![0; *xmax as usize + 1];
    let mut acc = 0;
    for (i, &m) in xcount.iter().enumerate() {
        delta_x[i] += acc;
        acc += m;
    }
    acc = 0;
    for (i, m) in xcount.iter().enumerate().rev() {
        acc += m;
        delta_x[i] -= acc;
    }

    let mut delta_y = vec![0; *ymax as usize + 1];
    acc = 0;
    for (i, &m) in ycount.iter().enumerate() {
        delta_y[i] += acc;
        acc += m;
    }
    acc = 0;
    for (i, m) in ycount.iter().enumerate().rev() {
        acc += m;
        delta_y[i] -= acc;
    }


    let go_left = |x: i32, tdist: i32| tdist - delta_x[x as usize];
    let go_right = |x: i32, tdist: i32| tdist + delta_x[x as usize+1];
    let go_below = |y: i32, tdist: i32| tdist - delta_y[y as usize];
    let go_above = |y: i32, tdist: i32| tdist + delta_y[y as usize+1];

    let tdist = total_distance(Coord::new(xmid, ymid), &points);


    let mut left = xmid;
    let mut tdist_left = tdist;
    while tdist_left <= limit {
        tdist_left = go_left(left, tdist_left);
        left -= 1;
    }
    tdist_left = go_right(left, tdist_left);
    left += 1;


    let mut right = xmid;
    let mut tdist_right = tdist;
    while tdist_right <= limit {
        tdist_right = go_right(right, tdist_right);
        right += 1;
    }
    tdist_right = go_left(right, tdist_right);
    right -= 1;

    let mut region = right - left + 1;

    let sav = (left, right, tdist_left, tdist_right);

    let mut up = ymid;
    'outer: loop {
        tdist_left = go_above(up, tdist_left);
        tdist_right = go_above(up, tdist_right);
        up += 1;

        while tdist_left > limit {
            tdist_left = go_right(left, tdist_left);
            left += 1;
            if left > right {
                break 'outer;
            }
        }
        while tdist_right > limit {
            tdist_right = go_left(right, tdist_right);
            right -= 1;
        }
        region += right - left + 1;
    }

    (left, right, tdist_left, tdist_right) = sav;

    let mut down = ymid;
    'outer: loop {
        tdist_left = go_below(down, tdist_left);
        tdist_right = go_below(down, tdist_right);
        down -= 1;
        while tdist_left > limit {
            tdist_left = go_right(left, tdist_left);
            left += 1;
            if left > right {
                break 'outer;
            }
        }
        while tdist_right > limit {
            tdist_right = go_left(right, tdist_right);
            right -= 1;
        }
        region += right - left + 1;
    }

    region
}

fn total_distance(point: Coord<i32>, points: &[Coord<i32>]) -> i32 {
    points.iter().map(|p| p.manhattan(point)).sum()
}



