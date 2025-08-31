const HL: (u32, usize) = (0b00000000_00000000_00000000_00111100, 1);
const PLUS: (u32, usize) = (0b00000000_00010000_00111000_00010000, 3);
const IL: (u32, usize) = (0b_00000000_00001000_00001000__00111000, 3);
const I: (u32, usize) = (0b00100000_00100000_00100000_00100000, 4);
const SQUARE: (u32, usize) = (0b00000000_00000000_00110000_00110000, 2);
const ROCKS: [(u32, usize); 5] = [HL, PLUS, IL, I, SQUARE];
const WALLS: u32 = 0x01010101;

pub fn solve(input: &str) -> (usize, u32) {
    let input = input.trim().as_bytes();
    
    let p1 = part1(input);

    (p1, 0)
}

fn part1(input: &[u8]) -> usize {
    let mut cave = [0u8; 5_000];
    cave[0] = 0xff;
    let mut height = 0;
    let mut rocks = ROCKS.iter().copied().cycle();
    let mut jets = input.iter().copied().cycle();

    for _ in 0..2022 {
        let (mut rock, size) = rocks.next().unwrap();
        let mut index = height + 3;
        let mut blocks = WALLS;

        loop {
            let jet = jets.next().unwrap();
            let next = if jet == b'<' { rock << 1 } else { rock >> 1 };
            if next & blocks == 0 {
                rock = next;
            }

            blocks = blocks << 8 | WALLS | cave[index] as u32;
            if rock & blocks == 0 {
                index -= 1;
            } else {
                let [row1, row2, row3, row4] = rock.to_le_bytes();
                cave[index+1] |= row1;
                cave[index+2] |= row2;
                cave[index+3] |= row3;
                cave[index+4] |= row4;
                height = height.max(index + size);
                break;
            }
        }
    }
    height
}