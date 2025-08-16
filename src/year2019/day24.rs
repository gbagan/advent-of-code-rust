use ahash::{HashSet, HashSetExt};

pub fn solve(input: &str) -> (u32, u32) {
    let grid = input
        .bytes()
        .filter(|&c| c != b'\n')
        .enumerate()
		.fold(0, |acc, (i, b)| acc | ((b == b'#') as u64) << (i * 2));

    let p1 = part1(grid);
	let p2 = part2(grid);

    (p1, p2)

}

const fn mk_mask(bits: &[usize]) -> u64 {
	let mut mask = 0;
	let mut i = 0;
	while i < bits.len() {
		mask |= 1 << (2 * bits[i]);
		i += 1;
	}
	mask
}

fn part1(mut grid: u64) -> u32 {
	let mut seen = HashSet::new();
    while seen.insert(grid) {
        grid = step(grid, neighbors(grid)) & GRID_MASK
    }
    to_u32(grid)
}

const GRID_MASK: u64 = mk_mask(&[
	0, 1, 2, 3, 4,
	5, 6, 7, 8, 9,
	10, 11, 12, 13, 14,
	15, 16, 17, 18, 19,
	20, 21, 22, 23, 24
]);


fn saturating_add(x: u64, y: u64) -> u64 {
	((x & !GRID_MASK) | (x & GRID_MASK) + y) ^ (y & x & (x >> 1))
}

fn neighbors(grid: u64) -> u64 {
	const RIGHT_MASK: u64 = mk_mask(&[0, 1, 2, 3,
										5, 6, 7, 8,
										10, 11, 12, 13, 
										15, 16, 17, 18,
										20, 21, 22, 23]);
	const LEFT_MASK: u64 = mk_mask(&[1, 2, 3, 4,
										6, 7, 8, 9,
										11, 12, 13, 14,
										16, 17, 18, 19,
										21, 22, 23, 24]);
	let mut nbor = saturating_add(grid << 10, grid >> 10);
	nbor = saturating_add(nbor, (grid & RIGHT_MASK) << 2);
	saturating_add(nbor, (grid & LEFT_MASK) >> 2)
}

fn step(grid: u64, nbor: u64) -> u64 {
	let survived =  grid & nbor & !(nbor >> 1);
	let born = !grid & (nbor ^ (nbor >> 1));
	survived | born
}

fn to_u32(grid: u64) -> u32 {
	let mut b = grid | (grid >> 31);
	b = (b & 0x99999999) | (b & 0x22222222) << 1 | (b & 0x44444444) >> 1;
	b = (b & 0xc3c3c3c3) | (b & 0x0c0c0c0c) << 2 | (b & 0x30303030) >> 2;
	b = (b & 0xf00ff00f) | (b & 0x00f000f0) << 4 | (b & 0x0f000f00) >> 4;
	b = (b & 0xff0000ff) | (b & 0x0000ff00) << 8 | (b & 0x00ff0000) >> 8;
	b as u32
}



fn part2(grid: u64) -> u32 {
	let mut grids = [0; 403];
	grids[201] = grid;
	for i in 1..201 {
		let mut prev = grids[200-i];
		for j in 201-i..202+i {
			let next = part2_step(prev, grids[j], grids[j+1]);
			prev = grids[j];
			grids[j] = next;
		}
	}

	grids.iter().map(|g| g.count_ones()).sum()
}

fn part2_step(inner: u64, grid: u64, outer: u64) -> u64 {
	const UP_MASK: u64 = mk_mask(&[0, 1, 2, 3, 4]);
	const DOWN_MASK: u64 = mk_mask(&[20, 21, 22, 23, 24]);
	const LEFT_MASK: u64 = mk_mask(&[0, 5, 10, 15, 20]);
	const RIGHT_MASK: u64 = mk_mask(&[4, 9, 14, 19, 24]);
	const INNER_MASK: u64 = mk_mask(&[7, 11, 13, 17]);
	const UP_DOWN_MASK: u64 = mk_mask(&[7, 17]);
	const NOT_CENTER_MASK: u64 = mk_mask(&[
		0, 1, 2, 3, 4,
		5, 6, 7, 8, 9,
		10, 11, 13, 14,
		15, 16, 17, 18, 19,
		20, 21, 22, 23, 24
	]);

	let mut nbor = neighbors(grid);

	let mut out_ud = ((outer >> 14) & 1) * UP_MASK;
	out_ud |= ((outer >> 34) & 1) * DOWN_MASK;
	let mut out_lr = ((outer >> 22) & 1) * LEFT_MASK;
	out_lr |= ((outer >> 26) & 1) * RIGHT_MASK;

	nbor = saturating_add(nbor, out_ud);
	nbor = saturating_add(nbor, out_lr);

	let in_ud = (inner & UP_MASK) << 10 | (inner & DOWN_MASK) >> 10;
	let in_lr = (inner & LEFT_MASK) << 2 | (inner & RIGHT_MASK) >> 2;

	nbor = saturating_add(nbor, (in_ud | in_lr) & INNER_MASK);
	nbor = saturating_add(nbor, (in_ud >> 2 | in_lr >> 10) & INNER_MASK);
	nbor = saturating_add(nbor, (in_ud << 2 | in_lr << 10) & INNER_MASK);
	nbor = saturating_add(nbor, ((in_ud >> 4 & UP_DOWN_MASK) | in_lr >> 20) & INNER_MASK);
	nbor = saturating_add(nbor, ((in_ud << 4 & UP_DOWN_MASK) | in_lr << 20) & INNER_MASK);

	step(grid, nbor) & NOT_CENTER_MASK
}