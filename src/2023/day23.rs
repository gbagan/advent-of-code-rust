use aoc::aoc;
use aoc::coord::Coord;
use aoc::grid::Grid;

fn neighbors1 (grid: &Grid<u8>, c: Coord) -> Vec<Coord> {
    match grid[c] {
        b'.' => c.adjacent()
                       .iter()
                       .filter(|&n| grid.contains(*n) && grid[*n] != b'#')
                       .map(|&n| n)
                       .collect(),
        b'^' => vec!(c.above()),
        b'v' => vec!(c.below()),
        b'<' => vec!(c.left()),
        b'>' => vec!(c.right()),
        _ => vec!()
    }
}

fn neighbors2 (grid: &Grid<u8>, c: Coord) -> Vec<Coord> {
    if grid[c] == b'#' {
        vec!()
    } else {
        c.adjacent()
        .iter()
        .filter(|&n| grid.contains(*n) && grid[*n] != b'#')
        .map(|&n| n)
        .collect()
    }
}

fn follow_path<F>(grid: &Grid<u8>, neighbors: F, pos: Coord, pred: Coord, goal: Coord) -> Option<(Coord, usize)>
    where F: Fn(&Grid<u8>, Coord) -> Vec<Coord>
{
    let mut pred = pred;
    let mut pos = pos;
    let mut len = 1;
    loop {
        let nbors = neighbors(&grid, pos);
        match nbors.len() {
            1 => {
                let next = nbors[0];
                if next != pred {
                    pred = pos;
                    pos = next;
                    len += 1;
                } else if pos == goal {
                    return Some((pos, len))
                } else {
                    return None
                }
            },
            2 => {
                let next1 = nbors[0];
                let next2 = nbors[1];
                len += 1;
                let pos2 = if next1 == pred { next2 } else { next1 };
                pred = pos;
                pos = pos2;
            },
            _ => return Some((pos, len))
        }
    }
}

fn compress_grid<F>(grid: &Grid<u8>, neighbors: F) -> Vec<Vec<(usize, usize)>>
    where F: Fn(&Grid<u8>, Coord) -> Vec<Coord>
{
    let h = grid.height as i64;
    let w = grid.width as i64;
    let start = Coord::new(1, 0);
    let goal = Coord::new(w-2, h-1);
    let mut junctions = vec!();
    let mut n = 0;
    let grid2 = grid.map_with_indices(|c, _| {
        let nbors = neighbors1(&grid, c);
        if c == start || c == goal || nbors.len() > 2 {
            let m = n;
            n += 1;
            let nbors2 = nbors.iter().map(|&next| follow_path(&grid, &neighbors, next, c, goal)).flatten().collect();
            junctions.push(c);
            (m, nbors2)
        } else {
            (n, vec!())
        }
    });
    junctions.iter().map(|&c| {
        let nbors = &grid2[c];
        nbors.1.iter().map(|(c2, len)| (grid2[*c2].0, *len)).collect()
    }).collect()
}

pub fn longest_path(graph: &Vec<Vec<(usize, usize)>>) -> usize {
    let goal = graph.len()-1;
    let mut todo = Vec::new();
    todo.push((0, 1 as usize, 0));
    let mut best_score = 0;

    while let Some((current, visited, len)) = todo.pop() {
        if current == goal {
            best_score = best_score.max(len);
        } else {
            for (nbor, len2) in graph[current].iter() {
                let mask = 1 << nbor;
                if visited & mask == 0 {
                    todo.push((*nbor, visited | mask, len+len2));
                }
            }
        }
    };
    best_score
}

fn previous_border (graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let after_start = graph[0][0].0;
    let goal = graph.len()-1;
    let before_goal = graph[goal][0].0;
    let mut after_it = graph[after_start].iter().map(|&x| x.0).filter(|&x| x != 0);
    let after_start1 = after_it.next().unwrap();
    let after_start2 = after_it.next().unwrap();
    let mut prev_border = vec!(0; graph.len());

    for y in [after_start1, after_start2] {
        let mut current = y;
        let mut prev = after_start;
        while current != before_goal {
            prev_border[current] = prev;
            let next = graph[current].iter().map(|&x| x.0).find(|&x| x != prev && graph[x].len() <= 3).unwrap();
            prev = current;
            current = next;
        }
    }

    prev_border
}

pub fn longest_path_heuristic(graph: &Vec<Vec<(usize, usize)>>, prev_border: &Vec<usize>) -> usize {
    let goal = graph.len()-1;
    let mut todo = Vec::new();
    todo.push((0, 1 as usize, 0));
    let mut best_score = 0;

    while let Some((current, visited, len)) = todo.pop() {
        if current == goal {
            best_score = best_score.max(len);
        } else {
            for (nbor, len2) in graph[current].iter() {
                let mask = 1 << nbor;
                if visited & mask == 0 && visited & (1 << prev_border[*nbor]) != 0 {
                    todo.push((*nbor, visited | mask, len+len2));
                }
            }
        }
    };
    best_score
}

fn main() {
    let input = include_str!("../../inputs/2023/23");
    let grid = Grid::parse(input);
    aoc(|| {
        let graph = compress_grid(&grid, neighbors1);
        let p1 = longest_path(&graph);

        let graph = compress_grid(&grid, neighbors2);
        let prev_border = previous_border(&graph);
        let p2 = longest_path_heuristic(&graph, &prev_border);

        (p1, p2)
    })
}