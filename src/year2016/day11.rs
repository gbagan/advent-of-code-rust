use ahash::{HashSet, HashSetExt};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
struct State {
    elevator: u32,
    floor: [Floor; 4],
}

impl State {
    #[inline]
    fn is_complete(&self) -> bool {
        self.floor[0].is_empty() && self.floor[1].is_empty() && self.floor[2].is_empty()
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
struct Floor {
    mask: u8
}

impl Floor {
    #[inline]
    fn new(generators: usize, microchips: usize) -> Self {
        Floor { mask:  ((generators << 4) + microchips) as u8 }
    }

    #[inline]
    fn generators(self) -> u8 {
        self.mask >> 4
    }

    #[inline]
    fn microchips(self) -> u8 {
        self.mask & 15
    }

    #[inline]
    fn is_empty(self) -> bool {
        self.mask == 0
    }

    #[inline]
    fn is_valid(self) -> bool {
        self.generators() == 0 || self.microchips() <= self.generators()
    }

    #[inline]
    fn leq(self, other: Self) -> bool {
        self.generators() <= other.generators() && self.microchips() <= other.microchips()
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        Self { mask: self.mask + other.mask }
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self { mask: self.mask - other.mask }
    }

}

pub fn solve(input: &str) -> (u32, u32) {
    let mut state = State::default();

    for (i, line) in input.lines().enumerate() {
        let generators = line.matches("generator").count();
        let microchips = line.matches("microchip").count();
        state.floor[i] = Floor::new(generators, microchips);
    }
    state.elevator = 0;
    let p1 = bfs(state, 256, 2500);
    state.floor[0] = state.floor[0].add(Floor::new(2, 2));
    let p2 = bfs(state, 1024, 10_000);

    (p1, p2)
}


fn bfs(start: State, queue_capacity: usize, seen_capacity: usize) -> u32 {       
    let moves = [Floor::new(1, 1), Floor::new(2, 0), Floor::new(0, 2), Floor::new(1, 0), Floor::new(0, 1)];
    let mut queue1 = Vec::with_capacity(queue_capacity);
    let mut queue2 = Vec::with_capacity(queue_capacity);
    let mut seen = HashSet::with_capacity(seen_capacity);
    queue1.push(start);
    let mut dist = 0;
    while !queue1.is_empty() {
        for &state in &queue1 {
            if state.is_complete() {
                return dist;
            }
            if !seen.insert(state) {
                continue;
            }

            let elevator = state.elevator as usize;
            let current_floor = state.floor[elevator];
            let go_down = elevator > 0 &&
                            state.floor[0..elevator].iter().any(|floor| !floor.is_empty());

            if state.elevator < 3 {
                let mut found = false;

                for (i, &mov) in moves.iter().enumerate() {
                    if i == 3 && found {
                        break
                    }
                    if !mov.leq(current_floor) {
                        continue
                    }
                    let new_floor = current_floor.sub(mov);
                    if !new_floor.is_valid() {
                        continue;
                    }
                    let above_floor = state.floor[elevator+1].add(mov);
                    if above_floor.is_valid() {
                        let mut new_state = state;
                        new_state.floor[elevator] = new_floor;
                        new_state.floor[elevator+1] = above_floor;
                        new_state.elevator += 1;
                        queue2.push(new_state);
                        found = true;
                    }
                }
            }
            if go_down {
                let mut found = false;
                for (i, &mov) in moves.iter().rev().enumerate() {
                    if i == 2 && found {
                        break
                    }
                    if !mov.leq(current_floor) {
                        continue
                    }
                    let new_floor = current_floor.sub(mov);
                    if !new_floor.is_valid() {
                        continue;
                    }
                    let below_floor = state.floor[elevator-1].add(mov);
                    if below_floor.is_valid() {
                        let mut new_state = state;
                        new_state.floor[elevator] = new_floor;
                        new_state.floor[elevator-1] = below_floor;
                        new_state.elevator -= 1;
                        queue2.push(new_state);
                        found = true;
                    }
                }
            }
        }
        dist += 1;
        std::mem::swap(&mut queue1, &mut queue2);
        queue2.clear();
    }
    unreachable!();
}




#[test]
fn bfs_test() {
    let state = State {
        elevator: 0,
        floor: [
            Floor::new(0, 2),
            Floor::new(1, 0),
            Floor::new(1, 0),
            Floor::new(0, 0),
        ]
    };
    let res = bfs(state, 256, 2500);
    assert_eq!(res, 11);
}