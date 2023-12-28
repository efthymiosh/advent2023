use std::collections::{HashSet, BinaryHeap};

use super::util;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: u32,
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    path_id: u64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn longest_path(grid: &mut [&mut [char]]) -> u32 {
    let size = grid.len();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {x:0, y:1, dx:1, dy:0, cost: 0, path_id: 0});
    heap.push(State {x:0, y:1, dx:0, dy:1, cost: 0, path_id: 0});

    let mut dist_grid: Vec<Vec<u32>> = vec![vec![0; size]; size];

    let mut max_distance = 0;
    while let Some(state) = heap.pop() {
        if (state.x, state.y) == (size as i64 - 1, size as i64 - 2) {
            max_distance = state.cost;
            continue;
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
        match grid[x as usize][y as usize] {
            '#' => continue,
            '<' if state.dy == 1 => continue,
            '>' if state.dy == -1 => continue,
            '^' if state.dx == 1 => continue,
            'v' if state.dx == -1 => continue,
            _ => {},
        }
        let cost = state.cost + 1;
        neighbors.push(State { cost, x, y, dx: state.dy, dy: -state.dx, path_id: state.path_id});
        neighbors.push(State { cost, x, y, dx: -state.dy, dy: state.dx, path_id: state.path_id});
        neighbors.push(State { cost, x, y, dx: state.dx, dy: state.dy, path_id: state.path_id});
        for nstate in neighbors {
            if visited.contains(&(x, y, nstate.dx, nstate.dy)) {
                continue;
            }
            visited.insert((x, y, nstate.dx, nstate.dy));
            heap.push(nstate);
            dist_grid[x as usize][y as usize] = cost;
        }
    }
    util::grid::print_gridvec(&dist_grid, 5, 0);
    max_distance
}


pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec!['.'; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }
    let distance = longest_path(&mut grid);

    println!("Longest distance to goal {}", distance);
    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
