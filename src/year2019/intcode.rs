use crate::util::parser::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status { Run, Halt }

#[derive(Clone)]
pub struct IntCode {
    data: Vec<i32>,
    offset: usize,
    status: Status
}

impl IntCode {
    pub fn new(input: &str) -> Self {
        let data = input.iter_unsigned().collect();
        Self { data, offset: 0, status: Status::Run }
    }

    pub fn set(&mut self, index: usize, val: i32) {
        self.data[index] = val;
    }

    pub fn get(&self, index: usize) -> i32 {
        self.data[index]
    }

    pub fn step(&mut self) {
        let instr = self.data[self.offset];
        match instr % 100 {
            1 => {
                let a = self.address(1, (instr / 100) % 10);
                let b = self.address(2, (instr / 1000) % 10);
                let c = self.address(3, instr / 10000);
                self.data[c] = self.data[a] + self.data[b];
                self.offset += 4;
            }
            2 => {
                let a = self.address(1, (instr / 100) % 10);
                let b = self.address(2, (instr / 1000) % 10);
                let c = self.address(3, instr / 10000);
                self.data[c] = self.data[a] * self.data[b];
                self.offset += 4;
            }
            99 => self.status = Status::Halt,
            c => panic!("invalid instruction {c}")
        }
    }

    pub fn run(&mut self) {
        while self.status == Status::Run {
            self.step();
        }
    }

    fn address(&self, offset: usize, mode: i32) -> usize {
        match mode {
            0 => self.data[self.offset + offset] as usize,
            1 => self.offset + offset,
            _ => panic!("invalid mode")
        }
    }

}