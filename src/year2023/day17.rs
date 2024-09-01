use crate::util::{coord::Coord, grid::Grid};

type Point = Coord<i32>;

pub fn solve(input: &str) -> Option<(u16, u16)> {
    let grid= Grid::parse(input);
    let p1 = astar(&grid, 1, 3)?;
    let p2 = astar(&grid, 4, 10)?;
    Some((p1, p2))
}

const VERTICAL: usize = 0;
const HORIZONTAL: usize = 1;

fn astar(grid: &Grid<u8>, min_dist: u16, max_dist: u16) -> Option<u16> {
    let start= Point::ORIGIN;
    let goal = Point::new(grid.width as i32 - 1, grid.height as i32 - 1);
    let heuristic = grid.map_with_indices(|p, _| {
        let diff = goal - p;
        let dist = p.manhattan(goal);
        let penalty = diff.x.abs_diff(diff.y) as u16 * min_dist / max_dist;
        dist as usize + penalty as usize
    });
    let max_heuristic = heuristic.vec.iter().max()?;
    let mut queue = BucketQueue::new(*max_heuristic + 100);
    let mut costs = Grid::new(grid.width, grid.height, [u16::MAX; 2]);
    queue.insert(heuristic[start], (start, HORIZONTAL, 0));
    queue.insert(heuristic[start], (start, VERTICAL, 0));
    while let Some((_, (node, direction, cost))) = queue.pop() {
        if node == goal {
            return Some(cost);
        }
        if cost > costs[node][direction] {
            continue;
        }

        let dirs = if direction == VERTICAL {
            [Point::NORTH, Point::SOUTH]}
        else {
            [Point::WEST, Point::EAST]
        };
        
        for dir2 in dirs {
            let mut psum = 0;
            let mut next = node;
            let next_dir = 1 - direction;
            for i in 1..=max_dist {
                next += dir2;
                if !grid.contains(next) {
                    break;
                }
                psum += grid[next] - b'0';
                if i < min_dist {
                    continue;
                }
                let h = heuristic[next];
                let next_cost = cost + psum as u16;
                if next_cost < costs[next][next_dir] {
                    costs[next][next_dir] = next_cost;
                    queue.insert(next_cost as usize + h, (next, next_dir, next_cost));
                }
            }
        }
    } 

    None
}


struct BucketQueue<A> {
    min: Option<usize>,
    buckets: Vec<Vec<A>>,
}

impl <A> BucketQueue<A> {
    fn new(n: usize) -> BucketQueue<A> {
        let buckets = (0..n).map(|_| vec!()).collect();
        BucketQueue { buckets, min: None }
    }

    fn insert(&mut self, key: usize, val: A) {
        if key >= self.buckets.len() {
            self.buckets
                .resize_with(key + 1, || Vec::new());
        }
        
        self.buckets[key].push(val);
        if self.min.map(|m| m > key).unwrap_or(true) {
            self.min = Some(key);
        }
    }

    fn pop(&mut self) -> Option<(usize, A)> {
        if let Some(min) = self.min {
            let v = self.buckets[min].pop()?;
            self.min = (min..self.buckets.len()).find(|&i| !self.buckets[i].is_empty());
            Some((min, v))
        } else {
            None
        }
    }
}