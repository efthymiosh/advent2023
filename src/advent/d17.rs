use std::collections::{HashSet, BinaryHeap};

use super::util;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: u32,
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    straight_for: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn shortest_path(grid: &mut [&mut [u32]], minstraight: usize, maxstraight: usize) -> u32 {
    let size = grid.len();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {x:0, y:0, dx:1, dy:0, straight_for:1, cost:0});
    heap.push(State {x:0, y:0, dx:0, dy:1, straight_for:1, cost:0});

    let mut min_distance = u32::max_value();
    while let Some(state) = heap.pop() {
        if (state.x, state.y) == (size as i64 - 1, size as i64 - 1) {
            min_distance = state.cost;
            break;
        }
        let mut neighbors = Vec::new();
        let x = state.x + state.dx;
        let y = state.y + state.dy;
        if x < 0 || x >= size as i64 {
            continue;
        }
        if y < 0 || y >= size as i64 {
            continue;
        }
        let cost = state.cost + grid[x as usize][y as usize];
        if state.straight_for >= minstraight {
            neighbors.push(State { cost, x, y, straight_for: 1, dx: state.dy, dy: -state.dx, });
            neighbors.push(State { cost, x, y, straight_for: 1, dx: -state.dy, dy: state.dx, });
        }
        if state.straight_for < maxstraight {
            neighbors.push(State { cost, x, y, straight_for: state.straight_for + 1, dx: state.dx, dy: state.dy, });
        }
        for nstate in neighbors {
            if visited.contains(&(x, y, nstate.dx, nstate.dy, nstate.straight_for)) {
                continue;
            }
            visited.insert((x, y, nstate.dx, nstate.dy, nstate.straight_for));
            heap.push(nstate);
        }
    }
    min_distance
}


pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![0; size * size];
    let mut grid: Vec<&mut [u32]> = grid_raw.as_mut_slice().chunks_mut(size).collect();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c.to_digit(10).unwrap();
        }
    }
    let distance = shortest_path(&mut grid, 0, 3);

    println!("Least distance to goal {}", distance);
    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();
    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![0; size * size];
    let mut grid: Vec<&mut [u32]> = grid_raw.as_mut_slice().chunks_mut(size).collect();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c.to_digit(10).unwrap();
        }
    }
    let distance = shortest_path(&mut grid, 4, 10);

    println!("Least distance to goal {}", distance);
    Ok(())
}
