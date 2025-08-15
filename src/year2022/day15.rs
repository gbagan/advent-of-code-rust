use ahash::{HashSet, HashSetExt};
use crate::util::{coord::*, parser::*, range::*};

struct Scan {
    sensor: Coord<i32>,
    beacon: Coord<i32>,
    distance: i32,
}

pub fn solve(input: &str) -> (i32, u64) {
    let scans: Vec<_> = input
        .iter_signed()
        .tuples()
        .map(|(x1, y1, x2,y2)| {
            let sensor = Coord::new(x1, y1);
            let beacon = Coord::new(x2, y2);
            let distance = sensor.manhattan(beacon);
            Scan {sensor, beacon, distance}
        })
        .collect();

    let p1 = part1(&scans);
    let p2 = part2(&scans).unwrap();
    (p1, p2)
    
}

// interval between a ball (w.r.t. Manhattan distance) and a row
fn intersection_ball_with_row(center: Coord<i32>, radius: i32, row: i32) -> Option<Range<i32>> {
    let dx = radius - center.y.abs_diff(row) as i32;
    if dx < 0 {
        None
    } else {
        Some(Range::new(center.x - dx, center.x + dx))
    }
}

// union of disjoint intervals that does not cointain non detected beacons
fn intervals_without_beacons(y: i32, scans: &[Scan]) -> Vec<Range<i32>> {
    let ranges = scans.iter().filter_map(|scan|
            intersection_ball_with_row(scan.sensor, scan.distance, y)
    );
    Range::disjoint_union(ranges)
}

fn part1(scans: &[Scan]) -> i32 {
    let y_target = 2_000_000;
    let nb_beacons: i32 = intervals_without_beacons(y_target, scans).iter().map(Range::length).sum();
    let mut detected_beacons: Vec<_> =
        scans
        .iter()
        .filter(|b| b.beacon.y == y_target)
        .map(|b| b.beacon.x)
        .collect();
    detected_beacons.sort_unstable();
    detected_beacons.dedup();
    nb_beacons - detected_beacons.len() as i32
}


fn is_not_detected(point: Coord<i32>, scans: &[Scan]) -> bool {
    scans.iter().all(|scan|
        point != scan.beacon && scan.sensor.manhattan(point) >= scan.distance
    )
}

fn part2(scans: &[Scan]) -> Option<u64> {
    let range = 0..4_000_000;
    let mut top = HashSet::new();
    let mut left = HashSet::new();
    let mut bottom = HashSet::new();
    let mut right = HashSet::new();
    for Scan { sensor, distance, .. } in scans {
        top.insert(sensor.x + sensor.y - distance - 1);
        left.insert(sensor.x - sensor.y - distance - 1);
        bottom.insert(sensor.x + sensor.y + distance + 1);
        right.insert(sensor.x - sensor.y + distance + 1);
    }
    
    let xs: Vec<_> = left.intersection(&right).collect();
    let ys: Vec<_> = top.intersection(&bottom).collect();

    let point =
            xs
            .iter()
            .cartesian_product(&ys)
            .map(|(&&x, &&y)| Coord::new((x + y) / 2, (y - x) / 2))
            .find(|p|
                range.contains(&p.x)
                && range.contains(&p.y)
                && is_not_detected(*p, scans)
            )?;
    Some(point.x as u64 * 4_000_000 + point.y as u64)
}