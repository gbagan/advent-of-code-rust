use anyhow::*;
use itertools::Itertools;
use crate::util::parser::*;

struct Room<'a> {
    encrypted: &'a str,
    sector_id: u32,
    checksum: &'a str,
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let rooms: Vec<_> = input.lines().map(parse_room).collect();
    
    let p1 = rooms
        .iter()
        .filter_map(|room| is_real_room(room).then_some(room.sector_id))
        .sum();
    
    let p2 = rooms
        .iter()
        .find_map(|room| is_northpole_room(room).then_some(room.sector_id))
        .context("Part 2: No solution found")?;

    Ok((p1, p2))
}



fn parse_room(line: &str) -> Room {
    let len = line.len();
    let encrypted = &line[..len-11]; 
    let sector_id = (&line[len-10..len-7]).next_unsigned().unwrap();
    let checksum = &line[len-6..len-1];
    Room { encrypted, sector_id, checksum }
}

fn is_real_room(room: &Room) -> bool {
    let &Room { encrypted, checksum , ..} = room;
    let checksum = checksum.as_bytes();
    let mut freqs = [0; 26];
    let mut freq_freqs = [0; 32];
    freq_freqs[0] = encrypted.len() as u32;
    let mut max_freq = 0;
    for c in encrypted.bytes() {
        if c != b'-' {
            let index = (c - b'a') as usize;
            let freq = freqs[index];
            freqs[index] += 1;
            freq_freqs[freq+1] += 1;
            freq_freqs[freq] -= 1;
            max_freq = max_freq.max(freq+1);
        }
    }
    if freqs[(checksum[0] - b'a') as usize] != max_freq {
        return false;
    }

    for (&c1, &c2) in checksum.iter().tuple_windows() {
        let freq_c1 = freqs[(c1 - b'a') as usize];
        let freq_c2 = freqs[(c2 - b'a') as usize];
        if freq_c1 < freq_c2 || freq_c1 == freq_c2 && c1 >= c2 {
            return false;
        }
        if (freq_c2+1..freq_c1).any(|freq| freq_freqs[freq] != 0) {
            return false;
        }
    }
    true
}

fn is_northpole_room(room: &Room) -> bool {
    let encrypted = room.encrypted.as_bytes();
    if encrypted.len() != 24 || encrypted[9] != b'-' || encrypted[16] != b'-' {
        return false;
    }
    let decrypted: Vec<_> = encrypted
        .iter()
        .map(|&c|
            if c == b'-' {
                c
            } else {
                b'a' + (((c - b'a') as u32 + room.sector_id) % 26) as u8
            }
        ).collect();
    decrypted == b"northpole-object-storage"
}