use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::Hash;

use super::util;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Hash for State {
    // Omit Cost
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.dx.hash(state);
        self.dy.hash(state);
    }
}

fn fin_path(grid: &mut [&mut [char]], queue: &mut VecDeque<State>, visited: &mut HashSet<State>, dist_grid: &mut Vec<Vec<u32>>) -> u32 {
    let size = grid.len();
    let mut max_distances: HashMap<u64, u32> = HashMap::new();
    while let Some(state) = queue.pop_front() {
        if (state.x, state.y) == (size as i64 - 1, size as i64 - 2) {
            max_distances.insert(state.path_id, state.cost);
            //break;
        }
        let mut neighbors = Vec::new();
        let x = state.x + state.dx;
        let y = state.y + state.dy;
        let cost = state.cost + 1;
        let mut dpath = 0;
        for (dx, dy) in [(state.dy, -state.dx), (-state.dy, state.dx), (state.dx, state.dy)] {
            if x + dx < 0 || x + dx >= size as i64 {
                continue;
            }
            if y + dy < 0 || y + dy >= size as i64 {
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
            neighbors.push(State { cost, x, y, dx, dy, path_id: state.path_id + dpath});
            dpath += 1;
        }
        for nstate in neighbors {
            if visited.contains(&nstate) {
                continue;
            }
            visited.insert(nstate.clone());
            queue.push_back(nstate);
            dist_grid[x as usize][y as usize] = cost;
        }
//      util::grid::print_gridvec(&dist_grid, 5, 0);
//      util::debug::wait_for(10);
    }
    max_distances.values().max().unwrap().clone()
}

fn longest_path(grid: &mut [&mut [char]]) -> u32 {
    let size = grid.len();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(State {x:0, y:1, dx:1, dy:0, cost: 0, path_id: 0});

    let mut dist_grid: Vec<Vec<u32>> = vec![vec![0; size]; size];

    fin_path(grid, &mut queue, &mut visited, &mut dist_grid)
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
