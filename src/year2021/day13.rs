use crate::util::parser::*;

pub fn solve(input: &str) -> (usize, String) {
    let (section1, section2) = input.rsplit_once("\n\n").unwrap();

    let mut dots: Vec<_> = section1
        .iter_unsigned::<u32>()
        .array_chunks::<2>()
        .collect();
    
    let mut lines = section2.lines();
        
    fold(&mut dots, lines.next().unwrap());   
    dots.sort_unstable();
    dots.dedup();

    let p1 = dots.len();

    let mut width = 0;
    let mut height = 0;
    for line in lines {
        let (along_x, n) = fold(&mut dots, line);
        if along_x { width = n + 1 } else { height = n };
    }

    let mut p2 = vec![b' '; width * height + 1];
    for c in p2.iter_mut().step_by(width) {
        *c = b'\n';
    }

    for [x, y] in dots {
        p2[1 + y as usize * width + x as usize] = b'#';
    }

    let p2 = String::from_utf8(p2).unwrap();

    (p1, p2)
}

fn fold(dots: &mut[[u32; 2]], line: &str) -> (bool, usize) {
    let line = line.as_bytes();
    let along_x = line[11] == b'x';
    let n = (&line[13..]).try_unsigned::<u32>().unwrap();

    if along_x {
        for dot in dots {
            if dot[0] >= n {
                dot[0] = 2 * n - dot[0];
            }
        }
    } else {
        for dot in dots {
            if dot[1] >= n {
                dot[1] = 2 * n - dot[1];
            }
        }
    }
    (along_x, n as usize)
}