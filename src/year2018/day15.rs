// todo SIMD?

use std::thread;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

pub fn solve(input: &str) -> (i32, i32) {
    let mut walls = [0u32; 32];
    let mut units = Vec::new();
    for (i, c) in input.bytes().filter(|&c| c != b'\n').enumerate() {
        match c {
            b'#' => set_bit(&mut walls, i),
            b'G' => units.push(Unit{ position: i, race: Race::Goblin, health: 200 }),
            b'E' => units.push(Unit{ position: i, race: Race::Elf, health: 200 }),
            _ => {},
        }
    }

    let p1 = Game::new(&units, 3, walls).simulate::<false>().unwrap();

    let n = thread::available_parallelism().unwrap().get();
    let done = AtomicBool::new(false);
    let counter = AtomicI32::new(4);
    let mut results = vec![(i32::MAX, 0); n];

    thread::scope(|scope| {
        for res in &mut results {
            scope.spawn(|| worker(&walls, &units, &done, &counter, res));
        }
    });

    let p2 = results.iter().min_by_key(|p| p.0).unwrap().1;

    (p1, p2)
}

fn worker(walls: &[u32; 32], units: &[Unit], done: &AtomicBool, counter: &AtomicI32, res: &mut (i32, i32)) {
    while !done.load(Ordering::Relaxed) {
        let power = counter.fetch_add(1, Ordering::Relaxed);
        if let Some(score) = Game::new(&units, power, *walls).simulate::<true>() {
            *res = (power, score);
            done.store(true, Ordering::Relaxed);
            return;
        }
    }
}


#[derive(PartialEq, Eq, Clone, Copy)]
enum Race { Elf, Goblin }
const DIRECTIONS: [usize; 4] = [0usize.wrapping_sub(32), usize::MAX, 1, 32];

#[derive(Clone)]
struct Unit {
    position: usize,
    race: Race,
    health: i32,
}

struct Game {
    grid: Vec<Option<usize>>,
    units: Vec<Unit>,
    elf_power: i32,
    walls: [u32; 32],
}

impl Game {
    fn new<'a>(units: &[Unit], elf_power: i32, walls: [u32; 32]) -> Self {
        let grid = vec![None; 32*32];
        let units = units.to_vec();
        Self { grid, units, elf_power, walls }
    }

    fn can_attack(&self, position: usize, race: Race) -> Option<usize> {
        let mut enemy_health = i32::MAX;
        let mut enemy_index = None;

        for &dir in DIRECTIONS.iter() {
            if let Some(nbor)  = self.grid[position.wrapping_add(dir)] 
                && self.units[nbor].race != race && self.units[nbor].health < enemy_health {
                    enemy_health = self.units[nbor].health;
                    enemy_index = Some(nbor);
                }
        }

        enemy_index
    }

    fn move_and_attack(&mut self, index: usize) -> Option<Race> {
        let Unit { position, race, health: _ } = self.units[index];
        let adversary = self.can_attack(position, race).or_else(|| {
            let next = self.move_(position, race);
            self.grid[position] = None;
            self.grid[next] = Some(index);
            self.units[index].position = next;
            self.can_attack(next, race)
        });
        if let Some(adv) = adversary {
            self.units[adv].health -= self.get_power(race);
            if self.units[adv].health <= 0 {
                self.grid[self.units[adv].position] = None;
                return Some(self.units[adv].race);
            }
        }
        None
    }

    fn simulate<const P2: bool>(&mut self) -> Option<i32> {
        let mut elves = self.units.iter().filter(|unit| unit.race == Race::Elf).count();
        let mut goblins = self.units.len() - elves;

        for turn in 0.. {
            self.units.retain(|u| u.health > 0);
            self.units.sort_unstable_by_key(|u| 32 * u.position);
            for (i, unit) in self.units.iter().enumerate() {
                self.grid[unit.position] = Some(i);
            }

            for index in 0..self.units.len() {
                if self.units[index].health <= 0 {
                    continue;
                }

                if let Some(dead) = self.move_and_attack(index) {
                    match dead {
                        Race::Elf if P2 => return None,
                        Race::Elf => elves -= 1,
                        Race::Goblin => goblins -= 1,
                    }
                }
                if elves == 0 || goblins == 0 {
                    return Some(turn * self.units.iter().map(|u| u.health.max(0)).sum::<i32>());
                }
            }
        }

        unreachable!()
    }

    fn move_(&self, position: usize, race: Race) -> usize {
        let mut obstacles = self.walls;
        let mut targets = [0; 32];
        for unit in &self.units {
            if unit.health <= 0 {
                continue
            } else if unit.race == race {
                set_bit(&mut obstacles, unit.position);
            } else {
                set_bit(&mut targets, unit.position);
            }
        }

        if let Some(adversary) = bfs(position, &obstacles, &targets) {
            let mut targets = [0; 32];
            set_bit(&mut targets, position - 1);
            set_bit(&mut targets, position + 1);
            set_bit(&mut targets, position - 32);
            set_bit(&mut targets, position + 32);
            bfs(adversary, &obstacles, &targets).unwrap()
        } else {
            position
        }
    }


    fn get_power(&self, race: Race) -> i32 {
        match race {
            Race::Elf => self.elf_power,
            Race::Goblin => 3,
        }
    }
}

#[inline]
fn bfs(start: usize, obstacles: &[u32; 32], targets: &[u32; 32]) -> Option<usize> {
    let mut grid = [0; 32];
    set_bit(&mut grid, start);

    while propagate(&mut grid, obstacles) {
        for i in 1..31 {
            let intersection = targets[i] & grid[i];
            if intersection != 0 {
                let x = intersection.trailing_zeros() as usize;
                return Some(i << 5 | x);
            }
        }
    }
    None
} 

#[inline]
fn propagate(grid: &mut [u32; 32], obstacles: &[u32; 32]) -> bool {
    let mut changed = 0;
    let mut previous = grid[0];

    for i in 1..31 {
        let current = grid[i];
        grid[i] = (previous | current | current << 1 | current >> 1 | grid[i+1]) & !obstacles[i];
        previous = current;
        changed |= current ^ grid[i];
    }
    return changed != 0
}

#[inline]
fn set_bit(grid: &mut [u32], position: usize) {
    grid[position >> 5] |= 1 << (position & 31);
}

