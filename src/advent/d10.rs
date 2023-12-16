use super::util;

fn get_valid_neighbors(grid: &[&mut [char]], (x,y): (usize, usize)) -> Vec<(usize,usize)> {
    let mut valid_paths = Vec::new();
    match grid[x - 1][y] {
        '-' | 'L' | 'F' => { valid_paths.push((x - 1, y)); },
        _ => {},
    };
    match grid[x][y - 1] {
        '|' | '7' | 'F' => { valid_paths.push((x , y - 1)); },
        _ => {},
    };
    match grid[x + 1][y] {
        '-' | '7' | 'J' => { valid_paths.push((x + 1, y)); },
        _ => {},
    };
    match grid[x][y + 1] {
        '|' | 'L' | 'J' => { valid_paths.push((x, y + 1)); },
        _ => {},
    };
    if valid_paths.len() != 2 {
        panic!("More than 2 valid neighbors, invalid data");
    }
    valid_paths
}

fn discover_weights(grid: &Vec<&mut [char]>, weights: &mut Vec<&mut [u64]>, (x, y): (usize, usize), parent_weight: u64) {
    if weights[x][y] <= parent_weight {
        return;
    }
    weights[x][y] = parent_weight + 1;
    match grid[x][y] {
        '|' => {
            discover_weights(grid, weights, (x, y - 1), parent_weight + 1);
            discover_weights(grid, weights, (x, y + 1), parent_weight + 1);
        }
        '-' => {
            discover_weights(grid, weights, (x + 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x - 1, y), parent_weight + 1);
        }
        'L' => {
            discover_weights(grid, weights, (x + 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y - 1), parent_weight + 1);
        }
        'J' => {
            discover_weights(grid, weights, (x - 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y - 1), parent_weight + 1);
        }
        '7' => {
            discover_weights(grid, weights, (x - 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y + 1), parent_weight + 1);
        }
        'F' => {
            discover_weights(grid, weights, (x + 1, y), parent_weight + 1);
            discover_weights(grid, weights, (x, y + 1), parent_weight + 1);
        }
        _ => {
            panic!("Reached unexpected {}", grid[x][y]);
        }
    }
}

fn discover_loop(grid: &Vec<&mut [char]>, mark: &mut Vec<&mut [u64]>, vertices: &mut Vec<(u64, u64)>, (x, y): (usize, usize)) {
    if mark[x][y] == 1 {
        return;
    }
    mark[x][y] = 1;
    vertices.push((x as u64, y as u64));
    match grid[x][y] {
        '|' => {
            discover_loop(grid, mark, vertices, (x, y - 1));
            discover_loop(grid, mark, vertices, (x, y + 1));
        }
        '-' => {
            discover_loop(grid, mark, vertices, (x + 1, y));
            discover_loop(grid, mark, vertices, (x - 1, y));
        }
        'L' => {
            discover_loop(grid, mark, vertices, (x + 1, y));
            discover_loop(grid, mark, vertices, (x, y - 1));
        }
        'J' => {
            discover_loop(grid, mark, vertices, (x - 1, y));
            discover_loop(grid, mark, vertices, (x, y - 1));
        }
        '7' => {
            discover_loop(grid, mark, vertices, (x - 1, y));
            discover_loop(grid, mark, vertices, (x, y + 1));
        }
        'F' => {
            discover_loop(grid, mark, vertices, (x + 1, y));
            discover_loop(grid, mark, vertices, (x, y + 1));
        }
        _ => {
            panic!("Reached unexpected {}", grid[x][y]);
        }
    }
}

fn polygon_area(vertices: &[(u64, u64)]) -> f64 {
    let n = vertices.len();
    let mut sum = 0.0;

    for cur in 0..n {
        let next = (cur + 1) % n;
        sum += (vertices[cur].0 as f64 * vertices[next].1 as f64) - (vertices[next].0 as f64 * vertices[cur].1 as f64);
    }

    0.5 * sum.abs()
}

fn picks_theorem(area: f64, vertices: &[(u64, u64)]) -> u64 {
    let boundary_points = vertices.len() as u64;
    let interior_points = area - (boundary_points / 2) as f64 + 1.0;

    interior_points as u64
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![' '; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    let mut weights_raw = vec![u64::max_value(); size * size];
    let mut weights: Vec<&mut [u64]> = weights_raw.as_mut_slice().chunks_mut(size).collect();

    let (mut startx, mut starty) = (0,0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
            if c == 'S' {
                (startx, starty) = (j, i);
            }
        }
    }

    util::print_grid(grid.as_mut_slice(), 1);

    weights[startx][starty] = 0;
    let v = get_valid_neighbors(&grid, (startx, starty));

    for (x,y) in v {
        discover_weights(&grid, &mut weights, (x, y), 0);
    }
    weights_raw.iter_mut().for_each(|e| if *e == u64::max_value() { *e = 0 });

    let mut weights: Vec<&mut [u64]> = weights_raw.as_mut_slice().chunks_mut(size).collect();
    util::print_grid(weights.as_mut_slice(), 1);
    if let Some(max) = weights_raw.iter().max() {
        println!("Max value: {}", max);
    }

    Ok(())
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = util::parse_in_lines(&path)?.peekable();

    let size = lines.peek().ok_or("Bad input file")?.len();

    let mut grid_raw = vec![' '; size * size];
    let mut grid: Vec<&mut [char]> = grid_raw.as_mut_slice().chunks_mut(size).collect();
    let mut mark_raw = vec![0; size * size];
    let mut mark: Vec<&mut [u64]> = mark_raw.as_mut_slice().chunks_mut(size).collect();

    let (mut startx, mut starty) = (0,0);
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[j][i] = c;
            if c == 'S' {
                (startx, starty) = (j, i);
            }
        }
    }
    mark[startx][starty] = 1;
    let v = get_valid_neighbors(&grid, (startx, starty));
    let mut vertices: Vec<(u64, u64)> = Vec::new();
    vertices.push((startx as u64, starty as u64));
    for (x,y) in v {
        discover_loop(&grid, &mut mark, &mut vertices, (x, y));
    }

    let area = polygon_area(vertices.as_slice());
    println!("Area: {}\nPolygon Count: {}", area, vertices.len());
    let inside_tiles: u64 = picks_theorem(area, vertices.as_slice());

    println!("Inside tiles: {:?}", inside_tiles);

    Ok(())
}
