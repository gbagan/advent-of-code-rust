use crate::util::grid::*;

const NB_BUCKETS: usize = 10;

pub fn solve(input: &str) -> (u16, u16) {
    let grid = Grid::parse_with_padding(input, b'#').unwrap();
    let p1 = dijkstra(&grid);

    let grid2 = expand_grid(&grid);
    let p2 = dijkstra(&grid2);

    (p1, p2)
}

fn dijkstra(grid: &Grid<u8>) -> u16 {
    let width = grid.width;
    let start = (width + 1) as u32;
    let goal = ((grid.height - 1) * width - 2) as u32;

    let mut distance = vec![u16::MAX; grid.vec.len()];
    let mut queue: [Vec<u32>; NB_BUCKETS] = std::array::from_fn(|_| Vec::with_capacity(200));

    queue[0].push(start);

    let mut index = 0;
    let mut current_dist = 0;

    loop {
        while let Some(node) = queue[index].pop() {
            if node == goal {
                return current_dist;
            }
            let node = node as usize;

            macro_rules! push {
                ($node: expr) => {
                    let c = grid[$node];
                    if c != b'#' {
                        let nbor_distance = current_dist + (c - b'0') as u16;
                        if nbor_distance < distance[$node] {
                            distance[$node] = nbor_distance;
                            queue[nbor_distance as usize % NB_BUCKETS].push($node as u32);
                        }
                    }
                }
            }

            push!(node+1);
            push!(node-1);
            push!(node+width);
            push!(node-width);
        }
        current_dist += 1;
        index = (index + 1) * (index != NB_BUCKETS-1) as usize;
    }
}

fn expand_grid(grid: &Grid<u8>) -> Grid<u8> {
    let inner_width = grid.width - 2;
    let inner_height = grid.height - 2;
    let mut grid2 = Grid::new(5*inner_width+2, 5*inner_height+2, b'#');
    for i in 0..inner_height {
        let src_index = (1+i) * grid.width + 1;
        let src_slice = &grid.vec[src_index..src_index+inner_width];
        for j in 0..5 {
            for k in 0..5 {
                let jk = (j + k) as u8;
                let index = (j * inner_height + i + 1) * grid2.width + k * inner_width + 1;
                for (x, &y) in grid2.vec[index..index+inner_width].iter_mut().zip(src_slice) {
                    *x = y + jk;
                    if *x > b'9' {
                        *x -= 9;
                    }
                }
            }
        }
    }

    grid2
}