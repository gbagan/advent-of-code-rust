use crate::util::parser::*;

// assume
// - coordinates between 0 and 999
// - a point is the intersection of at most 255 lines


pub fn solve(input: &str) -> (u32, u32) {
    let (orthogonal, diagonal): (Vec<_>, Vec<_>) = input
        .iter_unsigned::<usize>()
        .array_chunks::<4>()
        .partition(|[x1, y1, x2, y2]| x1 == x2 || y1 == y2);
    let mut points = vec!(0u8; 1_000_000);

    let mut count = 0;

    macro_rules! plot_line {
        ($x1: expr, $y1: expr, $x2: expr, $y2: expr, $step: expr) => {
            for i in (1000*$y1+$x1..1000*$y2+$x2+1).step_by($step) {
                points[i] += 1;
                if points[i] == 2 {
                    count += 1;
                }
            };
        }
    }

    for [x1, y1, x2, y2] in orthogonal {
        if x1 == x2 {
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            plot_line!(x1, ymin, x1, ymax, 1000);
        } else {
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            plot_line!(xmin, y1, xmax, y1, 1);
        }
    }

    let p1 = count;

    for [x1, y1, x2, y2] in diagonal {
        let (x1, y1, x2, y2) = if y1 < y2 {
            (x1, y1, x2, y2)
        } else {
            (x2, y2, x1, y1)
        };

        if x1 < x2 {
            plot_line!(x1, y1, x1, y2, 1001);
        } else {
            plot_line!(x1, y1, x1, y2, 999);
        }
    }

    (p1, count)
}