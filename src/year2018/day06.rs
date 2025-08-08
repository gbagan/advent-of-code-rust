use crate::util::{coord::*, parser::*};

struct Input {
    points: Vec<Coord<i32>>,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut xmax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    
    let points: Vec<_> = input
        .iter_unsigned()
        .array_chunks()
        .map(|[x, y]| {
            xmax = xmax.max(x);
            ymin = ymin.min(y);
            ymax = ymax.max(y);
            Coord::new(x, y)
        }).collect();

    
    let input = Input {points, xmax, ymin, ymax};

    let p1 = part1(&input);
    let p2 = part2(&input, 10000);

    (p1, p2)
}

fn part1(input: &Input) -> i32 {
    let Input {points, xmax: _, ymin, ymax} = input;
    let mut points = points.clone();
    points.sort_unstable_by_key(|p| p.x);
    let mut finite = vec![true; points.len()];
    let mut areas = vec![0; points.len()];
    let mut segments: Vec<(i32, i32, usize)> = Vec::new();
    
    for row in *ymin..*ymax+1 {
        segments.clear();

        for (i, point) in points.iter().enumerate() {
            let ydiff = (point.y - row).abs();

            while let Some((x2, ydiff2, j)) = segments.pop() {
                let xdiff = point.x - x2;
                let delta_y = ydiff - ydiff2;
                if xdiff < -delta_y {
                    continue;
                } else if xdiff < delta_y {
                    segments.push((x2, ydiff2, j));
                } else if xdiff == -delta_y {
                    segments.push((x2, ydiff2, usize::MAX));
                    segments.push((point.x, ydiff, i));
                } else if xdiff == delta_y {
                    segments.push((x2, ydiff2, j));
                    segments.push((point.x, ydiff, usize::MAX));
                } else {
                    segments.push((x2, ydiff2, j));
                    segments.push((point.x, ydiff, i));
                }
                break;
            }
            if segments.is_empty() {
                segments.push((point.x, ydiff, i));
            }
        }

        if row == *ymin || row == *ymax {
            for &(_, _, i) in &segments {
                if i != usize::MAX {
                    finite[i] = false;
                }
            }
        } else {
            let i = segments[0].2;
            if i != usize::MAX {
                finite[i] = false;
            }
            let i = segments[segments.len()-1].2;
            if i != usize::MAX {
                finite[i] = false;
            }

            for &[(x1, ydiff1, _), (x2, ydiff2, i), (x3, ydiff3, _)] in segments.array_windows() {
                if i != usize::MAX {
                    let start = x2 - (x2 - x1 + ydiff1 - ydiff2 - 1) / 2;
                    let end = x2 + (x3 - x2 + ydiff3 - ydiff2 - 1) / 2;
                    areas[i] += end - start + 1;
                }
            }
        }
    }

    finite.iter().zip(areas).filter_map(|(&f, a)| f.then_some(a)).max().unwrap()
}

fn part2(input: &Input, limit: i32) -> i32 {
    let Input {points, xmax, ymin: _, ymax} = input;

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

    let dist = total_distance(Coord::new(xmid, ymid), points);

    let mut left = xmid;
    let mut dist_left = dist;
    while dist_left <= limit {
        dist_left = go_left(left, dist_left);
        left -= 1;
    }
    dist_left = go_right(left, dist_left);
    left += 1;


    let mut right = xmid;
    let mut dist_right = dist;
    while dist_right <= limit {
        dist_right = go_right(right, dist_right);
        right += 1;
    }
    dist_right = go_left(right, dist_right);
    right -= 1;

    let mut area = right - left + 1;

    let sav = (left, right, dist_left, dist_right);

    let mut up = ymid;
    'outer: loop {
        dist_left = go_above(up, dist_left);
        dist_right = go_above(up, dist_right);
        up += 1;

        while dist_left > limit {
            dist_left = go_right(left, dist_left);
            left += 1;
            if left > right {
                break 'outer;
            }
        }
        while dist_right > limit {
            dist_right = go_left(right, dist_right);
            right -= 1;
        }
        area += right - left + 1;
    }

    (left, right, dist_left, dist_right) = sav;

    let mut down = ymid;
    'outer: loop {
        dist_left = go_below(down, dist_left);
        dist_right = go_below(down, dist_right);
        down -= 1;
        while dist_left > limit {
            dist_left = go_right(left, dist_left);
            left += 1;
            if left > right {
                break 'outer;
            }
        }
        while dist_right > limit {
            dist_right = go_left(right, dist_right);
            right -= 1;
        }
        area += right - left + 1;
    }

    area
}

fn total_distance(point: Coord<i32>, points: &[Coord<i32>]) -> i32 {
    points.iter().map(|p| p.manhattan(point)).sum()
}



