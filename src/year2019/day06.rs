const COM: usize = hash(b'C', b'O', b'M');

pub fn solve(input: &str) -> (u32, u16) {
    let input = input.as_bytes();
    let mut direct_orbit = [usize::MAX; 36*36*36];
        let mut objects = Vec::with_capacity(input.len() / 8 + 1);
    objects.push(COM);
    for line in input.as_chunks::<8>().0 {
        let object = hash(line[0], line[1], line[2]);
        let orbit = hash(line[4], line[5], line[6]);
        objects.push(orbit);
        direct_orbit[orbit] = object;
    }
    
    let mut distance_to_com = [u16::MAX; 36*36*36];
    distance_to_com[COM] = 0;
    let p1 = objects.iter().map(|&o|
        distance_helper(&direct_orbit, &mut distance_to_com, o) as u32
    ).sum();

    let p2 = part2(&direct_orbit, &distance_to_com);

    (p1, p2)
}

fn distance_helper(direct_orbit: &[usize], distance_to_com: &mut [u16], object: usize) -> u16 {
    if distance_to_com[object] != u16::MAX {
        distance_to_com[object]
    } else {
        let dist = 1 + distance_helper(direct_orbit, distance_to_com, direct_orbit[object]);
        distance_to_com[object] = dist;
        dist
    }

}

fn part2(direct_orbit: &[usize], distance_to_com: &[u16]) -> u16 {
    let mut object1 = hash(b'Y', b'O', b'U');
    let mut object2 = hash(b'S', b'A', b'N');

    let dist1 = distance_to_com[object1];
    let dist2 = distance_to_com[object2];
    if dist1 > dist2 {
        for _ in 0..dist1-dist2 {
            object1 = direct_orbit[object1];
        }
    } else {
        for _ in 0..dist2-dist1 {
            object2 = direct_orbit[object2];
        }
    }
    let mut lca_dist = dist1.min(dist2);
    while object1 != object2 {
        object1 = direct_orbit[object1];
        object2 = direct_orbit[object2];
        lca_dist -= 1;
    }

    dist1 + dist2 - 2 * lca_dist - 2
}


const fn hash(c1: u8, c2: u8, c3: u8) -> usize {
    hash_char(c1) * 36 * 36 + hash_char(c2) * 36 + hash_char(c3)
}

#[inline]
const fn hash_char(c: u8) -> usize {
    (if c <= b'9' { c - b'0' } else { c - b'A' + 10 }) as usize
}