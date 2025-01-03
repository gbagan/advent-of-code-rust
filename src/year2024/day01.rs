// https://lemire.me/blog/2022/01/21/swar-explained-parsing-eight-digits/
// radix sort

const N: usize = 1000;

pub fn solve(input: &str) ->(u32, u32) {
    let mut list1 = [0; N];
    let mut list2 = [0; N];
    let mut line = input.as_bytes();

    for i in 0..N {
        unsafe {
            list1[i] = parse_5first(to_u64(line.get_unchecked(0..8)));
            list2[i] = parse_5last(to_u64(line.get_unchecked(5..13)));
            line = line.get_unchecked(14..);
        }
    }

    unsafe {
        radix_sort(&mut list1);
        radix_sort(&mut list2);
    }

    let p1 = list1.iter().zip(list2.iter())
        .map(|(x, &y)| x.abs_diff(y))
        .sum();
 
    let mut p2 = 0;
    let mut i = 0;

    for x in list1 {
        while i < N && list2[i] < x {
            i += 1;
        }
        let mut count = 0;
        while i < N && list2[i] == x {
            count += 1;
            i += 1;
        }
        p2 += x * count;
    }

    (p1, p2)
}

#[inline]
fn parse_5first(val: u64) -> u32 {
    parse_8digits((val << 24) - 0x3030303030000000)
}

#[inline]
fn parse_5last(val: u64) -> u32 {
    parse_8digits(val - 0x3030303030202020)
}

#[inline]
fn to_u64(s: &[u8]) -> u64 {
    u64::from_le_bytes(s.try_into().unwrap())
}

#[inline]
fn parse_8digits(mut val: u64) -> u32 {
    const MASK: u64 = 0xFF | (0xFF << 32);
    const MUL1: u64 = 100 + (1000000 << 32);
    const MUL2: u64 = 1 + (10000 << 32);

    val = val * 10 + (val >> 8);
    val = ((val & MASK).wrapping_mul(MUL1) + ((val >> 16) & MASK).wrapping_mul(MUL2)) >> 32;
    val as u32
}

unsafe fn radix_sort<const N: usize>(arr: &mut [u32; N]) {
    let mut lowbits_count: [u16; 256] = [0; 256];
    let mut highbits_count: [u16; 512] = [0; 512];

    for &x in arr.iter() {
        *lowbits_count.get_unchecked_mut((x & 0xff) as usize) += 1;
        *highbits_count.get_unchecked_mut((x >> 8) as usize) += 1;
    }

    let mut prev = 0;
    for x in &mut lowbits_count {
        prev += *x;
        *x = prev;
    }

    let mut prev = 0;
    for x in &mut highbits_count {
        prev += *x;
        *x = prev;
    }

    let mut tmp = [0u32; N];

    for &x in arr.iter() {
        let y = lowbits_count.get_unchecked_mut((x & 0xff) as usize);
        *y -= 1;
        *tmp.get_unchecked_mut(*y as usize) = x;
    }

    for &x in tmp.iter().rev() {
        let y = highbits_count.get_unchecked_mut((x >> 8) as usize);
        *y -= 1;
        *arr.get_unchecked_mut(*y as usize) = x;
    }
}

#[test]
fn radix_sort_test() {
    let mut list = [0, 1, 256, 258];
    unsafe {
        radix_sort(&mut list);
    }
    assert_eq!(list, [0, 1, 256, 258]);

    let mut list = [258, 256, 1, 0];
    unsafe {
        radix_sort(&mut list);
    }
    assert_eq!(list, [0, 1, 256, 258]);
}