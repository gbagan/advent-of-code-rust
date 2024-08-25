// dynamic programming

pub fn parse(input: &str) -> Option<Vec<usize>> {
    Some(input.lines().filter_map(|line| line.parse().ok()).collect())
}

pub fn solve1(numbers: &[usize], target: usize) -> u32 {
    let n = numbers.len();
    let m = target+1;
    let size = (n+1)*m;
    let mut table: Vec<u32> = vec![0; size];

    table[0] = 1;
    for i in 1..=n {
        table[i*m] = 1;
        for j in 1..=target {
            let idx = i*m+j;
            table[idx] = table[idx-m];
            if numbers[i-1] <= j {
                table[idx] += table[idx-m-numbers[i-1]];
            }
        }
    }
    table[size-1]
}

pub fn solve2(numbers: &[usize], target: usize) -> Option<u32> {
    let n = numbers.len();
    let m = target+1;
    let size = (n+1)*m;
    let mut table: Vec<u32> = vec![0; size];
    let mut prev_table: Vec<u32> = vec![0; size];

    for i in 0..=n {
        prev_table[i*m] = 1;
    }
    
    for _ in 0..n {
        for i in 0..=m {
            table[i] = 0;
        }

        for i in 1..=n {
            table[i*m] = 1;
            for j in 1..=target {
                let idx = i*m+j;
                table[idx] = table[idx-m];
                if numbers[i-1] <= j {
                    table[idx] += prev_table[idx-m-numbers[i-1]];
                }
            }
        }
        if table[size-1] != 0 {
            return Some(table[size-1]);
        }
        std::mem::swap(&mut table, &mut prev_table);
    }
    None
}

pub fn part1(numbers: &[usize]) -> Option<u32> {
    Some(solve1(numbers, 150))
}

pub fn part2(numbers: &[usize]) -> Option<u32> {
    solve2(numbers, 150)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let numbers= vec!(20, 15, 10, 5, 5);
        let result = solve1(&numbers, 35);
        assert_eq!(result, 4);
    }
}