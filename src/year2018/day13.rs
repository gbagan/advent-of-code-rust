use crate::util::coord::*;

type Point = Coord<i32>;

pub fn solve(input: &str) -> (String, String) {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let mut carts = Vec::new();
    for (position, &c) in input.iter().enumerate() {
        let direction = match c {
            b'^' => Point::NORTH,
            b'v' => Point::SOUTH,
            b'<' => Point::WEST,
            b'>' => Point::EAST,
            _ => continue,
        };
        carts.push(Cart { position, direction, alive: true, turn: 0 });
    }
    let mut occupied = vec![false; input.len()];

    let p1 = loop {
        if let Some(crash) = tick(input, &mut carts, &mut occupied, width as i32) {
            break crash
        }
    };

    while carts.len() > 1 {
        tick(input, &mut carts, &mut occupied, width as i32);
    };
    let p2 = carts[0].position;

    let p1 = format!("{},{}", p1 % width, p1 / width);
    let p2 = format!("{},{}", p2 % width, p2 / width);
    (p1, p2)
}


fn tick(input: &[u8], carts: &mut Vec<Cart>, occupied: &mut [bool], width: i32) -> Option<usize> {
    let mut crash = None;
    
    carts.sort_unstable_by_key(|cart| cart.position);

    for i in 0..carts.len() {
        if carts[i].alive {
            occupied[carts[i].position] = false;
            carts[i].tick(input, width);
            if occupied[carts[i].position] {
                crash = Some(carts[i].position);
                let position = carts[i].position;
                for cart in carts.iter_mut() {
                    if cart.position == position {
                        cart.alive = false;
                    }
                }
                occupied[position] = false;
            } else {
                occupied[carts[i].position] = true;
            }
        }
    }
    if crash.is_some() {
        carts.retain(|cart| cart.alive);
    }
    crash
}

pub struct Cart {
    position: usize,
    direction: Coord::<i32>,
    alive: bool,
    turn: u32,
}

impl Cart {
    fn tick(&mut self, input: &[u8], width: i32) {
        self.position += (width * self.direction.y + self.direction.x) as usize;
        match input[self.position] {
            b'/' => self.direction = if self.direction.x == 0 {
                        self.direction.turn_right()
                    } else {
                        self.direction.turn_left()
                    },
            b'\\' => self.direction = if self.direction.y == 0 {
                        self.direction.turn_right()
                    } else {
                        self.direction.turn_left()
                    },
            b'+' => {
                (self.direction, self.turn) = match self.turn {
                    0 => (self.direction.turn_left(), 1),
                    1 => (self.direction, 2),
                    _ => (self.direction.turn_right(), 0),
                }
            },
            _ => {},
        }
    }
}