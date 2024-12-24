use anyhow::*;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let input = input.as_bytes();
    let n = input.len();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut enabled = true;
    let mut i = 0;

    while i < n {
        if input[i] != b'd' && input[i] != b'm' {
            i += 1;
        } else if input[i..].starts_with(b"do()") {
            i += 4;
            enabled = true;
        } else if input[i..].starts_with(b"don't()") {
            i += 7;
            enabled = false;
        } else if input[i..].starts_with(b"mul(") {
            i += 4;
            if i >= n || input[i].wrapping_sub(b'0') > 9 {
                continue;
            }
            let mut first = input[i].wrapping_sub(b'0') as u32;
            i += 1;
            while i < n && input[i].wrapping_sub(b'0') <= 9 {
                first = 10 * first + (input[i].wrapping_sub(b'0') - b'0') as u32;
                i += 1;
            }

            if i >= n || input[i] != b',' {
                continue;
            }        
            i += 1;
            
            if i >= n || input[i].wrapping_sub(b'0') > 9 {
                continue;
            }
            let mut second = input[i].wrapping_sub(b'0') as u32;
            i += 1;
            while i < n && input[i].wrapping_sub(b'0') <= 9 {
                second = 10 * second + input[i].wrapping_sub(b'0') as u32;
                i += 1;
            }
            if i >= n || input[i] != b')' {
                continue;
            }

            p1 += first * second;
            if enabled {
                p2 += first * second
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    Ok((p1, p2))
}