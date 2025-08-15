use crate::util::{heap::*, parser::*};

struct Bot {
    x: i32,
    y: i32,
    z: i32,
    radius: i32,
}

impl Bot {
    fn in_range(&self, bot: &Self) -> bool {
        (self.x - bot.x).abs() + (self.y - bot.y).abs() + (self.z - bot.z).abs() <= self.radius
    }
}

struct Box {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Box {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32, z1: i32, z2: i32) -> Box {
        Box { x1, x2, y1, y2, z1, z2 }
    }
    fn split(&self) -> [Box; 8] {
        let Box { x1, x2, y1, y2, z1, z2 } = *self;
        let mx = (self.x1 + self.x2) / 2;
        let my = (self.y1 + self.y2) / 2;
        let mz = (self.z1 + self.z2) / 2;
        [
            Box::new(x1, mx, y1, my, z1, mz),
            Box::new(x1, mx, y1, my, mz+1, z2),
            Box::new(x1, mx, my+1, y2, z1, mz),
            Box::new(x1, mx, my+1, y2, mz+1, z2),
            Box::new(mx+1, x2, y1, my, z1, mz),
            Box::new(mx+1, x2, y1, my, mz+1, z2),
            Box::new(mx+1, x2, my+1, y2, z1, mz),
            Box::new(mx+1, x2, my+1, y2, mz+1, z2),
        ]
    }

    fn in_range(&self, bot: &Bot) -> bool {
        let x = (self.x1 - bot.x).max(0) + (bot.x - self.x2).max(0);
        let y = (self.y1 - bot.y).max(0) + (bot.y - self.y2).max(0);
        let z = (self.z1 - bot.z).max(0) + (bot.z - self.z2).max(0);
        x + y + z <= bot.radius
    }

    fn distance_to_origin(&self) -> i32 {
        let x = self.x1.abs().min(self.x2.abs());
        let y = self.y1.abs().min(self.y2.abs());
        let z = self.z1.abs().min(self.z2.abs());
        x + y + z
    }

    fn size(&self) -> i32 {
        self.x2 - self.x1 + 1
    }

}


pub fn solve(input: &str) -> (usize, i32) {
    let mut xmin= i32::MAX;
    let mut xmax  = i32::MIN;
    let mut ymin= i32::MAX;
    let mut ymax  = i32::MIN;
    let mut zmin= i32::MAX;
    let mut zmax  = i32::MIN;
    let bots: Vec<_> = input
        .iter_signed()
        .array_chunks()
        .map(|[x, y, z, radius]| {
            xmin = xmin.min(x);
            xmax = xmax.min(x);
            ymin = ymin.min(y);
            ymax = ymax.max(y);
            zmin = zmin.min(z);
            zmax = xmax.max(z);
            Bot { x, y, z, radius }
        }).collect();

    // part 1

    let largest = bots.iter().max_by_key(|p| p.radius).unwrap();
    let p1 = bots.iter().filter(|bot| largest.in_range(bot)).count();

    // part 2

    let size = (xmax-xmin+1).max(ymax-ymin+1).max(zmax-zmin+1);
    let size = (size as u32).next_power_of_two() as i32;

    let mut heap = MinHeap::with_capacity(1_000);
    heap.push((0, 0, 0), Box::new(xmin, xmin+size-1,ymin, ymin+size-1, zmin, zmin+size-1));

    let p2 = loop {
        let Some((_, cube)) = heap.pop() else { panic!("no solution found")}; 
        if cube.size() == 1 {
            break cube.distance_to_origin();
        }
        for next in cube.split() {
            let in_range = bots.iter().filter(|nb| next.in_range(nb)).count();
            let priority = (bots.len() - in_range, next.distance_to_origin(), next.size());
            heap.push(priority, next);
        }
    };

    (p1, p2)
}