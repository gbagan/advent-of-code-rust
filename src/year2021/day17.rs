use crate::util::parser::*;

pub fn solve(input: &str) -> (i32, usize) {
    let [xmin, xmax, ymin, ymax] = input.iter_signed::<i32>().next_chunk().unwrap();
    let p1 = -ymin * (-ymin-1) / 2;

    let mut vxmin = 0;
    while vxmin * (vxmin + 1) / 2 < xmin {
        vxmin += 1;
    }
    let vxmax = xmax;
    let vymin = ymin;
    let vymax = -ymin;

    let tmax = 2 * -ymin as usize;

    let mut first = vec![0; tmax+1];
    let mut non_first = vec![0; tmax+1];

    let mut p2 = 0;

    for vx in vxmin..vxmax+1 {
        let mut vx = vx;
        let mut x = 0;
        let mut is_first = true;

        for t in 0..tmax+1 {
            if x >= xmin {
                if is_first {
                    first[t] += 1;
                    is_first = false;
                } else {
                    non_first[t] += 1;
                }
            }
            x += vx;
            vx = (vx - 1).max(0);
            if x > xmax {
                break;
            }
        }
    }

    for vy in vymin..vymax+1 {
        let mut vy = vy;
        let mut y = 0;
        let mut is_first = true;

        for t in 0..tmax+1 {
            if y <= ymax {
                if is_first {
                    p2 += first[t] + non_first[t];
                    is_first = false;
                } else {
                    p2 += first[t];
                }
            }
            y += vy;
            vy -= 1;
            if y < ymin {
                break;
            }
        }
    }

    (p1, p2)
}