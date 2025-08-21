pub fn solve(input: &str) -> (u32, u32) {
    let mut solver1 = Solver1::new();
    let mut solver2 = Solver2::new();

    let mut y = 0;
    let mut row = 0;
    for c in input.bytes() {
        if c == b'\n' {
            solver1.grid[0][solver1.ymin+y] = row << 24;
            solver2.grid[0][0][solver1.ymin+y] = row << 24;
            y += 1;
            row = 0;
        } else {
            row = (row << 4) | (c == b'#') as u128;
        }
    }

    for _ in 0..6 {
        solver1.step();
        solver2.step();
    }
    
    let p1 = solver1.count();
    let p2 = solver2.count();

    (p1, p2)
}

const MASK1: u128 = u128::MAX / 15;
const MASK7: u128 = MASK1 * 7;
const MASK8: u128 = MASK1 * 8;
const MASKC: u128 = MASK1 * 12;

struct Solver1 {
    grid: [[u128; 20]; 8],
    grid2: [[u128; 20]; 8],
    grid3: [[u128; 20]; 8],
    ymin: usize,
    ymax: usize,
    zmax: usize,
}

impl Solver1 {
    fn new() -> Self {
        Self {
            grid: [[0; 20]; 8],
            grid2: [[0; 20]; 8],
            grid3: [[0; 20]; 8],
            ymin: 6,
            ymax: 14,
            zmax: 1
        }
    }

    fn step(&mut self) {
		// x axis
		for z in 0..self.zmax {
			for y in self.ymin..self.ymax {
                let current = self.grid[z][y];
				self.grid2[z][y] = sadd3(current, current<<4, current>>4);
			}
		}

		// y axis
		for z in 0..self.zmax {
			let mut prev = 0;
			self.grid3[z][self.ymin-1] = self.grid2[z][self.ymin];
			for y in self.ymin..self.ymax {
				let current = self.grid2[z][y];
				self.grid3[z][y] = sadd3(current, prev, self.grid2[z][y+1]);
				prev = current;
			}
			self.grid3[z][self.ymax] = prev;
		}
		self.ymin -= 1;
        self.ymax += 1;

		// z axis

        for y in self.ymin..self.ymax {
		    let mut prev = self.grid3[1][y];
            for z in 0..self.zmax {
				let current = self.grid3[z][y];
                self.grid2[z][y] = sadd3(
                    current,
                    prev,
                    self.grid3[z+1][y]
                );
                prev = current;
			}
            self.grid2[self.zmax][y] = prev;
		}
        self.zmax += 1;

		// transition rule
		for z in 0..self.zmax {
			for y in self.ymin..self.ymax {
				self.grid[z][y] = transition(self.grid[z][y], self.grid2[z][y])
			}
		}
	}

    fn count(&self) -> u32 {
        let mut counter = 0;
        for y in self.ymin..self.ymax {
			counter += self.grid[0][y].count_ones();
		}
        for z in 1..self.zmax {
            for y in self.ymin..self.ymax {
				counter += 2 * self.grid[z][y].count_ones();
			}
		}

        counter
    }
}


struct Solver2 {
    grid: [[[u128; 20]; 8]; 8],
    grid2: [[[u128; 20]; 8]; 8],
    grid3: [[[u128; 20]; 8]; 8],
    ymin: usize,
    ymax: usize,
    zmax: usize,
    tmax: usize,
}

impl Solver2 {
    fn new() -> Self {
        Self {
            grid: [[[0; 20]; 8]; 8],
            grid2: [[[0; 20]; 8]; 8],
            grid3: [[[0; 20]; 8]; 8],
            ymin: 6,
            ymax: 14,
            zmax: 1,
            tmax: 1,
        }
    }

    fn step(&mut self) {
		// x axis
        for t in 0..self.tmax {
		    for z in 0..self.zmax {
			    for y in self.ymin..self.ymax {
				    let current = self.grid[t][z][y];
				    self.grid2[t][z][y] = sadd3(current, current<<4, current>>4);
			    }
		    }
        }

		// y axis
		for t in 0..self.tmax {
			for z in 0..self.zmax {
				let mut prev = 0;
				self.grid3[t][z][self.ymin-1] = self.grid2[t][z][self.ymin];
				for y in self.ymin..self.ymax {
					let current = self.grid2[t][z][y];
					self.grid3[t][z][y] = sadd3(
                        current,
                        prev,
                        self.grid2[t][z][y+1]
                    );
					prev = current;
				}
				self.grid3[t][z][self.ymax] = prev;
			}
		}

		self.ymin -= 1;
        self.ymax += 1;

		// z axis
        for t in 0..self.tmax {
            for y in self.ymin..self.ymax {
				let mut prev = if t == 0 { self.grid3[0][1][y] } else { self.grid3[t-1][t][y] };
				for z in t..self.zmax {
					let current = self.grid3[t][z][y];
					self.grid2[t][z][y] = sadd3(
                        current,
                        prev,
                        self.grid3[t][z+1][y]
                    );
					prev = current;
				}
				self.grid2[t][self.zmax][y] = prev;
				if t == 1 {
                    self.grid2[1][0][y] = saturating_add(
                        self.grid3[0][1][y],
                        self.grid3[1][1][y] << 1,
                    );
                } else if t > 1 {
                    self.grid2[t][t-1][y] = sadd3(
                        self.grid3[t-1][t][y],
                        self.grid3[t-2][t][y],
                        self.grid3[t][t][y]
                    );
                }
			}
		}
        self.zmax += 1;

		// t axis
		for z in 0..self.zmax {
			for y in self.ymin..self.ymax {
				let mut prev = self.grid2[1][z][y];
				for t in 0..z+1 {
					let current = self.grid2[t][z][y];
					self.grid3[t][z][y] = sadd3(
                        current,
                        prev,
                        self.grid2[t+1][z][y]
                    );
					prev = current;
				}
			}
		}
		self.tmax += 1;

		// transition rule
        for t in 0..self.tmax {
		    for z in 0..self.zmax {
			    for y in self.ymin..self.ymax {
				    self.grid[t][z][y] = transition(
                        self.grid[t][z][y],
                        self.grid3[t][z][y],
                    );
			    }
		    }
	    }
    }

    fn count(&self) -> u32 {
        let mut counter = 0;

        for t in 0..self.tmax {
            for z in 0..self.zmax {
                for y in self.ymin..self.ymax {
				    let coeff = (z != 0) as u32 + (t != 0) as u32 + (z != t) as u32;
                    counter += self.grid[t][z][y].count_ones() << coeff;
			    }
		    }
        }

        counter
    }
}


#[inline]
fn saturating_add(x: u128, y: u128) -> u128 {
	let s = (x + (y & MASK7)) | (y & MASK8);
	let h = MASK8 | ((s >> 3) & MASK1);
	s & (h - MASK1)
}

#[inline]
fn sadd3(x: u128, y: u128, z: u128) -> u128 {
    saturating_add(x, saturating_add(y, z))
}


#[inline]
fn transition(row: u128, neighbors: u128) -> u128 {
    let mut r = (neighbors - row) | row;
	r ^= MASKC;
    r &= r >> 1;
    r &= r >> 2;
    r & MASK1
}