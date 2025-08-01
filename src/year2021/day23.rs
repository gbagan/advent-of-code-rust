use ahash::{HashMap, HashMapExt};
use crate::util::heap::MinHeap;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const ROOM: usize = 4;
const EMPTY: usize = 5;
const COST: [u32; 4] = [1, 10, 100, 1000];

pub fn solve(input: &str) -> (u32, u32) {
    let input = input
        .bytes()
        .filter(u8::is_ascii_uppercase)
        .map(|c| (c - b'A') as usize)
        .next_chunk::<8>()
        .unwrap();

    let p1_start = Burrow::p1_start(&input);
    let p1 = mandatory_cost(p1_start) + dijkstra(p1_start, 100);
    
    let p2_start = Burrow::p2_start(&input);
    let p2 = mandatory_cost(p2_start) + dijkstra(p2_start, 10_000);

    (p1, p2)
}


fn mandatory_cost(burrow: Burrow) -> u32 {
    let mut energy = 0;
    let mut depth = [0; 4];

    for (room_index, &room) in burrow.rooms.iter().enumerate() {
        let pods = room.decode();
        
        for (i, &pod) in pods.iter().rev().enumerate().skip_while(|(_, pod)| **pod == room_index) {
            depth[pod] += 1;
            let up = 4 - i;
            let across = 2 * pod.abs_diff(room_index);
            let down = depth[pod];
            let extra = 2 * (pod == room_index) as usize;
            energy += COST[pod] * (up + across + down + extra) as u32;
        }
    }
    energy
}

fn dijkstra(start: Burrow, capacity: usize) -> u32 {
    let mut queue = MinHeap::with_capacity(capacity);
    let mut seen = HashMap::with_capacity(capacity);
    seen.insert(start, 0);
    queue.push(0, start);

    while let Some((energy, burrow)) = queue.pop() {
        if burrow.is_goal() {
            return energy;
        }
        if seen[&burrow] != energy {
            continue;
        }

        macro_rules! push_nexts2 {
            ($i:expr, $j: expr) => {
                if !burrow.rooms[$i].can_enter($i) && !burrow.rooms[$i].is_full($i) {
                    push_nexts(&mut queue, &mut seen, burrow, energy, $i, (0..$j).rev());
                    push_nexts(&mut queue, &mut seen, burrow, energy, $i, $j+1..11);
                }
            }
        }
        push_nexts2!(A, 2);
        push_nexts2!(B, 4);
        push_nexts2!(C, 6);
        push_nexts2!(D, 8);
    }
    unreachable!()
}

fn push_nexts(
    queue: &mut MinHeap<u32, Burrow>,
    seen: &mut HashMap<Burrow, u32>,
    mut burrow: Burrow,
    energy: u32,
    room_index: usize,
    iter: impl Iterator<Item = usize>,
) {
    let pod = burrow.rooms[room_index].pop();

    for i in iter {
        match burrow.hallway.get(i) {
            ROOM => {},
            EMPTY => {
                let room_position = 2 + 2 * room_index;
                let pod_position = 2 + 2 * pod;

                let min = room_position.min(pod_position);
                let max = room_position.max(pod_position);
                let extra_move =
                    (min.saturating_sub(i) + i.saturating_sub(max)) as u32
                    - (room_position == pod_position) as u32;

                let extra = 2 * extra_move * COST[pod];

                if pod != room_index && extra == 0 {
                    continue;
                }

                let mut next = burrow;
                next.hallway.set(i, pod);
                let next_energy = energy + extra;
                let min = *seen.get(&next).unwrap_or(&u32::MAX);

                if next_energy < min {
                    next.enter_rooms();
                    queue.push(next_energy, next);
                    seen.insert(next, next_energy);
                }
            }
            _ => break,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Room {
    bits: u16,
}

impl Room {
    fn new(x: usize, y: usize, z: usize, t: usize) -> Self {
        Room {bits: 1 << x | 1 << (y+4) | 1 << (z+8) | 1 << (t+12) }
    }

    #[inline]
    fn pop(&mut self) -> usize {
        let pod = self.bits.trailing_zeros() as usize;
        self.bits >>= 4;
        pod
    }

    #[inline]
    fn peek(&self) -> Option<usize> {
        if self.bits == 0 {
            None
        } else {
            Some(self.bits.trailing_zeros() as usize)
        }
    }

    #[inline]
    fn push(&mut self, pod: usize) {
        self.bits = self.bits << 4 | 1 << pod;
    }

    #[inline]
    fn can_enter(self, pod: usize) -> bool {
        let mask = 1 << pod;
        self.bits == 0
            || self.bits == mask
            || self.bits == 0b10001 * mask
            || self.bits == 0b100010001 * mask
    }

    #[inline]
    fn is_full(self, pod: usize) -> bool {
        self.bits == 0b1000100010001 * (1 << pod)
    }

    fn decode(self) -> [usize; 4] {
        let mut room = self;
        std::array::from_fn(|_| {
            if room.bits == 0 { EMPTY } else { room.pop() }
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Hallway {
    bits: u64,
}


impl Hallway {
    fn new() -> Self {
        Self { bits: 0x55454545455 }
    }

    #[inline]
    fn get(&self, index: usize) -> usize {
        (self.bits >> (index * 4) & 15) as usize
    }

    #[inline]
    fn set(&mut self, index: usize, value: usize) {
        let mask: u64 = !(15 << (index * 4));
        let value = (value as u64) << (index * 4);
        self.bits = (self.bits & mask) | value;
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Burrow {
    hallway: Hallway,
    rooms: [Room; 4],
}

impl Burrow {
    fn p1_start(input: &[usize; 8]) -> Self {
        Self {
            hallway: Hallway::new(),
            rooms: [
                Room::new(input[0], input[4], A, A),
                Room::new(input[1], input[5], B, B),
                Room::new(input[2], input[6], C, C),
                Room::new(input[3], input[7], D, D),
            ]
        }
    }

    fn p2_start(input: &[usize; 8]) -> Self {
        Self {
            hallway: Hallway::new(),
            rooms: [
                Room::new(input[0],  D, D, input[4]),
                Room::new(input[1], C, B, input[5]),
                Room::new(input[2], B, A, input[6],),
                Room::new(input[3], A, C, input[7]),
            ]
        }
    }


    fn is_goal(&self) -> bool {
        self.rooms[A].is_full(A)
        && self.rooms[B].is_full(B)
        && self.rooms[C].is_full(C)
        && self.rooms[D].is_full(D)
    }

    fn enter_rooms(&mut self) {
        let mut changed;

        macro_rules! enter_room {
            ($pod:expr, $it:expr) => {
                for j in $it {
                    match self.hallway.get(j) {
                        EMPTY => {},
                        ROOM => {
                            let k = (j-2) / 2;
                            while let Some(pod2) = self.rooms[k].peek() && pod2 == $pod {
                                changed = true;
                                self.rooms[k].pop();
                                self.rooms[$pod].push($pod);
                            }
                        },
                        pod2 if pod2 == $pod => {
                            changed = true;
                            self.hallway.set(j, EMPTY);
                            self.rooms[$pod].push($pod);
                        }
                        _ => break,
                    }
                }
            }
        }
        macro_rules! enter_room2 {
            ($pod:expr, $position:expr) => {
                if self.rooms[$pod].can_enter($pod) {
                    enter_room!($pod, (0..$position).rev());
                    enter_room!($pod, $position+1..11);
                }
            }
        }

        loop {
            changed = false;
            enter_room2!(A, 2);
            enter_room2!(B, 4);
            enter_room2!(C, 6);
            enter_room2!(D, 8);
            if !changed {
                break;
            }
        }
    }
}

