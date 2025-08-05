use crate::util::parser::*;

pub fn solve(input: &str) -> (String, i32) {
    let mut points: Vec<_> = input.iter_signed::<i32>().array_chunks::<4>().collect();
    let point1 = points.iter().find(|p| p[3] == 5).unwrap();
    let point2 = points.iter().find(|p| p[3] == -5).unwrap();
    let mut t = (point2[1] - point1[1]) / 10;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    for [_, py, _, vy] in &points[..10] {
        let py2 = py + vy * t;
        ymin = ymin.min(py2);
        ymax = ymax.max(py2);
    }
    if ymax - ymin >= 10 {
        t += 1;
    }

    let mut xmin = i32::MAX;
    let mut ymin = i32::MAX;
    for point in &mut points {
        point[0] += point[2] * t;
        point[1] += point[3] * t;
        xmin = xmin.min(point[0]);
        ymin = ymin.min(point[1]);
    }

    let mut message = vec![b'.'; 630];
    for i in 0..10 {
        message[i*63] = b'\n';
    }
    for [px, py, _, _] in points {
        message[63 * (py - ymin) as usize + (px - xmin) as usize + 1] = b'#';
    }
    let message = String::from_utf8(message).unwrap();

    (message, t)
}